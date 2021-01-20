use std::collections::HashMap;
use crate::dl_lite::types::DLType;
use crate::dl_lite::node::{Node, Mod};
use std::io::{ErrorKind, Error};
use std::cmp::Ordering;
use std::io;
use std::hash::Hash;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::abox_item::ABI;

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
        let name = vec_of_string[1];

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
    let splitted = s.split(" ").collect::<Vec<&str>>();
    __parse_string_to_node_helper(splitted, symbols)
}

pub fn node_to_string(
    node: &Node,
    symbols: &HashMap<String, (usize, DLType)>,
    mut current: String,
) -> Option<String> {
    match node {
        Node::B => Some(String::from("BOTTOM")),
        Node::T => Some(String::from("TOP")),
        Node::N(n) => {
            let vec_of_s = find_keys_for_value(symbols, *n);

            if vec_of_s.len() > 0 {
                Some(vec_of_s[0].clone())
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

pub fn string_to_tbi(s: &str, symbols: &HashMap<String, (usize, DLType)>) -> Option<TBI> {
    Option::None
}

pub fn tbi_to_string(tbi: &TBI, symbols: &HashMap<String, (usize, DLType)>) -> Option<String> {
    Option::None
}

pub fn string_to_abi(s: &str, symbols: &HashMap<String, (usize, DLType)>) -> Option<ABI> {
    Option::None
}

pub fn abi_to_string(tbi: &ABI, symbols: &HashMap<String, (usize, DLType)>) -> Option<String> {
    Option::None
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
                let new_error = Error::new(ErrorKind::InvalidData, format!("this symbols is not recognized {}", splitted[0]));
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





