use std::collections::HashMap;
use crate::dl_lite::types::DLType;
use crate::dl_lite::node::{Node, Mod};
use std::io::{ErrorKind, Error};
use std::cmp::Ordering;
use std::io;
use std::hash::Hash;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::json_filetype_utilities::{invalid_data_result, result_from_error};

//--------------------------------------------------------------------------------------------------

pub fn string_to_symbol(string: &str) -> io::Result<(&str, DLType)> {
    let new_string = string.trim();
    let vec_of_string: Vec<&str> = new_string.split(":").collect();

    if vec_of_string.len() != 2 {
        let new_error = Error::new(
            ErrorKind::InvalidData,
            "badly formatted symbol: there must be exactly one ':' character",
        );
        Err(new_error)
    } else {
        let type_in_string = vec_of_string[0].trim();
        let name = vec_of_string[1].trim();

        let t_op = match type_in_string {
            "concept" => Some(DLType::BaseConcept),
            "role" => Some(DLType::BaseRole),
            "nominal" => Some(DLType::Nominal),
            _ => Option::None,
        };

        match t_op {
            Some(t) => Ok((name, t)),
            _ => {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!("invalid dl type: {}", type_in_string),
                );
                Err(new_error)
            }
        }
    }
}

pub fn string_to_node(s: &str, symbols: &HashMap<String, (usize, DLType)>) -> io::Result<Node> {
    /*
    this function need a symbols dictionary reference to function
     */
    let splitted = s.trim();
    let splitted = splitted.split(" ").collect::<Vec<&str>>();

    __parse_string_to_node_helper(splitted, symbols)
}

pub fn node_to_string(
    node: &Node,
    symbols: &HashMap<String, (usize, DLType)>,
    mut current: String,
) -> Option<String> {
    match node {
        Node::B => Some(String::from("Bottom")),
        Node::T => Some(String::from("Top")),
        Node::N(n) => {
            let vec_of_s = find_keys_for_value(symbols, *n);

            if vec_of_s.len() > 0 {
                current.push_str(vec_of_s[0].as_str());
                Some(current)
            } else {
                Option::None
            }
        }
        Node::R(n) | Node::C(n) => {
            let vec_of_s = find_keys_for_value(symbols, *n);

            if vec_of_s.len() > 0 {
                current.push_str(vec_of_s[0].as_str()); // no space here, need to account for it when doing the modifiers
                Some(current)
            } else {
                Option::None
            }
        }
        Node::X(m, bn) => {
            match m {
                Mod::I => {
                    current.push_str("INV "); // space here
                    node_to_string(bn, symbols, current)
                }
                Mod::E => {
                    current.push_str("EXISTS "); // space here
                    node_to_string(bn, symbols, current)
                }
                Mod::N => {
                    current.push_str("NOT "); // space here
                    node_to_string(bn, symbols, current)
                }
            }
        }
    }
}

pub fn string_to_tbi(s: &str, symbols: &HashMap<String, (usize, DLType)>) -> io::Result<Vec<TBI>> {
    let pre_splitted = s.trim();

    let equiv = pre_splitted.contains("=");
    let sub = pre_splitted.contains("<");

    match (sub, equiv) {
        (true, true) => {
            let new_error = Error::new(ErrorKind::InvalidData, format!("badly formed file '=' and '<' bot appear").as_str());
            Err(new_error)
        },
        (false, false) => {
            let new_error = Error::new(ErrorKind::InvalidData, format!("badly formed file not '=' nor '<' found").as_str());
            Err(new_error)
        },
        (_, _) => {
            let mut lside_result1 = invalid_data_result("not done yet");
            let mut rside_result1 = invalid_data_result("not done yet");
            let mut lside_result2 = invalid_data_result("not done yet");
            let mut rside_result2 = invalid_data_result("not done yet");
            let mut splitted: Vec<&str>;
            let mut tuples: Vec<(io::Result<Node>, io::Result<Node>)>;
            let mut tbis: Vec<TBI> = Vec::new();

            if sub {
                splitted = pre_splitted.split("<").collect();
            } else {
                splitted = pre_splitted.split("=").collect();
            }

            if splitted.len() != 2 {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    "badly formatted symbol: there must be exactly one ':' character",
                );

                Err(new_error)
            } else {

                if sub {
                    lside_result1 = string_to_node(splitted[0], symbols);
                    rside_result1 = string_to_node(splitted[1], symbols);

                    tuples = vec![(lside_result1, rside_result1)];
                } else {
                    lside_result1 = string_to_node(splitted[0], symbols);
                    rside_result1 = string_to_node(splitted[1], symbols);
                    lside_result2 = string_to_node(splitted[1], symbols);
                    rside_result2 = string_to_node(splitted[0], symbols);

                    tuples = vec![(lside_result1, rside_result1), (lside_result2, rside_result2)];
                }

                let mut error_happened = false;
                let mut try_to_add: Result<Vec<TBI>, Error> = Ok(Vec::new());
                while !tuples.is_empty() {
                    let (lside_result, rside_result) = tuples.pop().unwrap();

                    try_to_add = match (&lside_result, &rside_result) {
                        (Err(e1), Err(e2)) => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("several errors, error1: {}, error2: {}", e1.to_string(), e2.to_string()));
                            Err(new_error)
                        },
                        (Err(e), _) => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't parse left side {}", e.to_string()));
                            Err(new_error)
                        },
                        (_, Err(e)) => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't parse right side {}", e.to_string()));
                            Err(new_error)
                        },
                        (Ok(lside), Ok(rside)) => {
                            let new_tbi_op = TBI::new(lside.clone(), rside.clone());

                            match new_tbi_op {
                                Some(new_tbi) => {
                                    tbis.push(new_tbi);
                                    Ok(Vec::new())
                                },
                                _ => {
                                    let new_error = Error::new(ErrorKind::InvalidData, format!("invalid tbox item {}", s));
                                    Err(new_error)
                                },
                            }
                        },
                    };

                    if try_to_add.is_err() {
                        error_happened = true;
                        break;
                    } else {
                        continue;
                    }
                }

                if error_happened {
                    try_to_add
                } else {
                    Ok(tbis)
                }
            }
        }
    }
}

pub fn tbi_to_string(tbi: &TBI, symbols: &HashMap<String, (usize, DLType)>) -> Option<String> {
    let lstr_op = node_to_string(tbi.lside(), symbols, "".to_string());
    let rstr_op = node_to_string(tbi.rside(), symbols, "".to_string());

    match (lstr_op, rstr_op) {
        (Some(lstr), Some(rstr)) => {
            let mut res = String::new();
            res.push_str(lstr.as_str());
            res.push_str(" < ");
            res.push_str(rstr.as_str());

            Some(res)
        },
        (_, _) => Option::None,
    }
}

// this approach is a dynamic one, concepts must be present in symbols,
// but nominals are added dynamically
pub fn string_to_abi(s: &str, symbols: &mut HashMap<String, (usize, DLType)>, mut current_id: usize) -> (io::Result<(ABI, Vec<(String, (usize, DLType))>)>, usize) {
    let mut splitted = s.trim();
    let mut splitted: Vec<&str> = splitted.split(":").collect();

    if splitted.len() != 2 {
        (invalid_data_result(format!("abox item must have exactly one ':' character {}", s).as_str()), current_id)
    } else {
        // remeber that abi must have only base concepts
        let abox_symbol = splitted[1].trim();

        if symbols.contains_key(abox_symbol) {
            let (_, dltype) = symbols[abox_symbol];
            let abox_symbol = string_to_node(abox_symbol, symbols);

            match &abox_symbol {
                Err(e) => (result_from_error(e), current_id),
                Ok(abi_symbol) => {
                    let constants: Vec<&str> = splitted[0].trim().split(",").collect();
                    let mut to_be_added: Vec<(String, (usize, DLType))>;

                    match (dltype, constants.len()) {
                        (DLType::BaseRole, 2) => {
                            let a1 = constants[0].trim();
                            let a2 = constants[1].trim();

                            // before augmenting current_id we need to know that the elements are not in symbols
                            to_be_added = Vec::new();
                            let node1: Node;
                            let node2: Node;

                            // each nominal
                            if !symbols.contains_key(a1) {
                                node1 = Node::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a1.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id1, _) = symbols[a1];
                                node1 = Node::new(Some(id1), DLType::Nominal).unwrap();
                            }

                            // then a2
                            if !symbols.contains_key(a2) {
                                node2 = Node::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a2.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id2, _) = symbols[a2];
                                node2 = Node::new(Some(id2), DLType::Nominal).unwrap();
                            }

                            let abi = ABI::new_ra(abi_symbol.clone(), node1, node2).unwrap();

                            (Ok((abi, to_be_added)), current_id)
                        },
                        (DLType::BaseConcept, 1) => {
                            let a1 = constants[0].trim();

                            // before augmenting current_id we need to know that the elements are not in symbols
                            to_be_added = Vec::new();
                            let node1: Node;

                            // each nominal
                            if !symbols.contains_key(a1) {
                                node1 = Node::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a1.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id1, _) = symbols[a1];
                                node1 = Node::new(Some(id1), DLType::Nominal).unwrap();
                            }

                            let abi = ABI::new_ca(abi_symbol.clone(), node1).unwrap();

                            (Ok((abi, to_be_added)), current_id)
                        },
                        (_, _) => (invalid_data_result(format!("incompatible type for abox item with number of elements: {}", s).as_str()), current_id)
                    }
                },
            }
        } else {
            (invalid_data_result(format!("unknown symbol in abox item: {}", abox_symbol).as_str()), current_id)
        }
    }
}

pub fn abi_to_string(abi: &ABI, symbols: &HashMap<String, (usize, DLType)>) -> Option<String> {
    match abi {
        ABI::CA(c, a) => {
            let c_str_op = node_to_string(c, symbols, "".to_string());
            let a_str_op = node_to_string(a, symbols, "".to_string());

            match (c_str_op, a_str_op) {
                (Some(c_str), Some(a_str)) => {
                    let mut res = String::new();
                    res.push_str(a_str.as_str());
                    res.push_str(" : ");
                    res.push_str(c_str.as_str());

                    Some(res)
                },
                (_, _) => Option::None,
            }
        },
        ABI::RA(r, a, b) => {
            let r_str_op = node_to_string(r, symbols, "".to_string());
            let a_str_op = node_to_string(a, symbols, "".to_string());
            let b_str_op = node_to_string(b, symbols, "".to_string());

            match (r_str_op, a_str_op, b_str_op) {
                (Some(r_str), Some(a_str), Some(b_str)) => {
                    let mut res = String::from("(");
                    res.push_str(a_str.as_str());
                    res.push_str(", ");
                    res.push_str(b_str.as_str());
                    res.push_str(")");
                    res.push_str(" : ");
                    res.push_str(r_str.as_str());

                    Some(res)
                },
                (_, _, _) => Option::None,
            }
        },
    }
}

//-------------------------------------------------------------------------------------------------
// real functions live here
fn __parse_string_to_node_helper(
    splitted: Vec<&str>,
    symbols: &HashMap<String, (usize, DLType)>,
) -> io::Result<Node> {
    // two auxiliary functions to do everything more tidy
    fn option_negate(n: Node) -> Option<Node> {
        Some(n.negate())
    }
    fn none_default(_: Node) -> Option<Node> {
        Option::None
    }

    match splitted.len() {
        1 => {
            /*
            here are only base symbols
             */

            if symbols.contains_key(splitted[0]) {
                let value = symbols[splitted[0]];
                let new_node = Node::new(Some(value.0), value.1).unwrap();

                Ok(new_node)
            } else {
                let new_error = Error::new(ErrorKind::InvalidData, format!("this symbols is not recognized '{}'", splitted[0]));
                Err(new_error)
            }
        }
        2 => {
            /*
            here we can have:
                NOT r
                NOT c
                INV r
                EXISTS r
             */
            let function_to_call = match splitted[0] {
                "NOT" => option_negate,
                "INV" => Node::inverse,
                "EXISTS" => Node::exists,
                _ => none_default,
            };

            let base_node_result = __parse_string_to_node_helper(vec![splitted[1]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => {
                            Ok(complex_node)
                        },
                        _ => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't build a valid node with this combination {} and {}", splitted[0], splitted[1]));
                            Err(new_error)
                        }
                    }
                },
                Err(_) => base_node_result,
            }
        }
        3 => {
            /*
            here we can have
            NOT INV r
            EXISTS INV r
             */
            let function_to_call = match splitted[0] {
                "NOT" => option_negate,
                "EXISTS" => Node::exists,
                _ => none_default,
            };
            let base_node_result = __parse_string_to_node_helper(vec![splitted[1], splitted[2]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => {
                            Ok(complex_node)
                        },
                        _ => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't build a valid node with this combination {}, {} and {}", splitted[0], splitted[1], splitted[2]));
                            Err(new_error)
                        }
                    }
                },
                Err(_) => base_node_result,
            }
        }
        4 => {
            /*
            here we can only have
            NOT EXISTS INV r
             */
            let function_to_call = match splitted[0] {
                "NOT" => option_negate,
                _ => none_default,
            };
            let base_node_result = __parse_string_to_node_helper(vec![splitted[1], splitted[2], splitted[3]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => {
                            Ok(complex_node)
                        },
                        _ => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't build a valid node with this combination {}, {}, {} and {}", splitted[0], splitted[1], splitted[2], splitted[3]));
                            Err(new_error)
                        }
                    }
                },
                Err(_) => base_node_result,
            }
        }
        _ => {
            let new_error = Error::new(ErrorKind::InvalidData, format!("invalid input: {:?}", splitted));
            Err(new_error)
        },
    }
}

fn find_keys_for_value(symbols: &HashMap<String, (usize, DLType)>, value: usize) -> Vec<String> {
    symbols
        .iter()
        .filter_map(|(key, &val)| {
            if val.0 == value {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect()
}




/*
I need this so the compilator can leave me alone
 */
#[derive(PartialEq, Eq, Debug, Hash)]
pub struct PS {
    // stands for presymbol
    name: String,
    t: DLType,
}

impl PS {
    pub fn new(name: String, t: DLType) -> PS {
        PS { name, t }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn t(&self) -> DLType {
        self.t
    }
}

impl PartialOrd for PS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.t.is_concept_type() && !other.t.is_concept_type() {
            Some(Ordering::Less)
        } else if self.t.is_role_type() && other.t.is_nominal_type() {
            Some(Ordering::Less)
        } else if self.t.is_nominal_type() && !other.t.is_nominal_type() {
            Some(Ordering::Greater)
        } else if self.t.is_role_type() && other.t.is_concept_type() {
            Some(Ordering::Greater)
        } else {
            self.name.partial_cmp(&other.name)
        }
    }
}

impl Ord for PS {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}





