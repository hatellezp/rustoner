/*
© - 2021 – UMONS
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/

use crate::dl_lite::abox_item::AbiDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::json_filetype_utilities::{invalid_data_result, result_from_error};
use crate::dl_lite::node::{Mod, NodeDllite};
use crate::dl_lite::tbox_item::TbiDllite;
use crate::kb::types::DLType;
use std::cmp::Ordering;

use crate::dl_lite::tbox::TBDllite;
use std::hash::Hash;
use std::io;
use std::io::{Error, ErrorKind};

use crate::dl_lite::abox::AbqDllite;
use crate::kb::knowledge_base::{ABox, Implier, SymbolDict, TBox, TBoxItem};

//--------------------------------------------------------------------------------------------------

type AbiqDlliteParseResult = (
    io::Result<(AbiqDllite, Vec<(String, (usize, DLType))>)>,
    usize,
);
type AbiDlliteParseResult = (
    io::Result<(AbiDllite, Vec<(String, (usize, DLType))>)>,
    usize,
);
// ------------------------------------------------------------------

pub fn string_to_symbol(string: &str) -> io::Result<(&str, DLType)> {
    let new_string = string.trim();
    let vec_of_string: Vec<&str> = new_string.split(':').collect();

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

pub fn string_to_node(s: &str, symbols: &SymbolDict) -> io::Result<NodeDllite> {
    /*
    this function need a symbols dictionary reference to function
     */
    let splitted = s.trim();
    let splitted = splitted.split(' ').collect::<Vec<&str>>();

    __parse_string_to_node_helper(splitted, symbols)
}

pub fn node_to_string(
    node: &NodeDllite,
    symbols: &SymbolDict,
    mut current: String,
) -> Option<String> {
    match node {
        NodeDllite::B => Some(String::from("Bottom")),
        NodeDllite::T => Some(String::from("Top")),
        NodeDllite::N(n) => {
            let vec_of_s = find_keys_for_value(symbols, *n);

            if !vec_of_s.is_empty() {
                current.push_str(vec_of_s[0].as_str());
                Some(current)
            } else {
                Option::None
            }
        }
        NodeDllite::R(n) | NodeDllite::C(n) => {
            let vec_of_s = find_keys_for_value(symbols, *n);

            if !vec_of_s.is_empty() {
                current.push_str(vec_of_s[0].as_str()); // no space here, need to account for it when doing the modifiers
                Some(current)
            } else {
                Option::None
            }
        }
        NodeDllite::X(m, bn) => {
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

// TESTING: this function creates a tbi from nothing so level is ZERO
pub fn string_to_tbi(s: &str, symbols: &SymbolDict) -> io::Result<Vec<TbiDllite>> {
    let pre_splitted = s.trim();

    let equiv = pre_splitted.contains('=');
    let sub = pre_splitted.contains('<');

    match (sub, equiv) {
        (true, true) => {
            let new_error = Error::new(
                ErrorKind::InvalidData,
                "badly formed file '=' and '<' bot appear",
            );
            Err(new_error)
        }
        (false, false) => {
            let new_error = Error::new(
                ErrorKind::InvalidData,
                "badly formed file not '=' nor '<' found",
            );
            Err(new_error)
        }
        (_, _) => {
            let mut lside_result1 = invalid_data_result("not done yet");
            let mut rside_result1 = invalid_data_result("not done yet");
            let mut lside_result2 = invalid_data_result("not done yet");
            let mut rside_result2 = invalid_data_result("not done yet");

            let splitted: Vec<&str>;
            let mut tuples: Vec<(io::Result<NodeDllite>, io::Result<NodeDllite>)>;
            let mut tbis: Vec<TbiDllite> = Vec::new();

            if sub {
                splitted = pre_splitted.split('<').collect();
            } else {
                splitted = pre_splitted.split('=').collect();
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

                    tuples = vec![
                        (lside_result1, rside_result1),
                        (lside_result2, rside_result2),
                    ];
                }

                let mut error_happened = false;
                let mut try_to_add: Result<Vec<TbiDllite>, Error> = Ok(Vec::new());
                while !tuples.is_empty() {
                    let (lside_result, rside_result) = tuples.pop().unwrap();

                    try_to_add = match (&lside_result, &rside_result) {
                        (Err(e1), Err(e2)) => {
                            let new_error = Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "several errors, error1: {}, error2: {}",
                                    e1.to_string(),
                                    e2.to_string()
                                ),
                            );
                            Err(new_error)
                        }
                        (Err(e), _) => {
                            let new_error = Error::new(
                                ErrorKind::InvalidData,
                                format!("couldn't parse left side {}", e.to_string()),
                            );
                            Err(new_error)
                        }
                        (_, Err(e)) => {
                            let new_error = Error::new(
                                ErrorKind::InvalidData,
                                format!("couldn't parse right side {}", e.to_string()),
                            );
                            Err(new_error)
                        }
                        (Ok(lside), Ok(rside)) => {
                            let level = 0; // newly created tbi level should be zero
                            let new_tbi_op = TbiDllite::new(lside.clone(), rside.clone(), level);

                            match new_tbi_op {
                                Some(new_tbi) => {
                                    tbis.push(new_tbi);
                                    Ok(Vec::new())
                                }
                                _ => {
                                    let new_error = Error::new(
                                        ErrorKind::InvalidData,
                                        format!("invalid tbox item {}", s),
                                    );
                                    Err(new_error)
                                }
                            }
                        }
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

pub fn tbi_to_string(tbi: &TbiDllite, symbols: &SymbolDict) -> Option<String> {
    let lstr_op = node_to_string(tbi.lside(), symbols, "".to_string());
    let rstr_op = node_to_string(tbi.rside(), symbols, "".to_string());

    match (lstr_op, rstr_op) {
        (Some(lstr), Some(rstr)) => {
            let mut res = String::new();
            res.push_str(lstr.as_str());
            res.push_str(" < ");
            res.push_str(rstr.as_str());

            Some(res)
        }
        (_, _) => Option::None,
    }
}

// this approach is a dynamic one, concepts must be present in symbols,
// but nominals are added dynamically
pub fn string_to_abi(
    s: &str,
    symbols: &mut SymbolDict,
    mut current_id: usize,
    for_completion: bool,
) -> AbiDlliteParseResult {
    let splitted = s.trim();
    let splitted: Vec<&str> = splitted.split(':').collect();

    if splitted.len() != 2 {
        (
            invalid_data_result(
                format!("abox item must have exactly one ':' character {}", s).as_str(),
            ),
            current_id,
        )
    } else {
        // remeber that abi must have only base concepts
        let abox_symbol = splitted[1].trim();

        if symbols.contains_key(abox_symbol) {
            let (_, dltype) = symbols[abox_symbol];
            let abox_symbol = string_to_node(abox_symbol, symbols);

            match &abox_symbol {
                Err(e) => (result_from_error(e), current_id),
                Ok(abi_symbol) => {
                    let constants: Vec<&str> = splitted[0].trim().split(',').collect();
                    let mut to_be_added: Vec<(String, (usize, DLType))>;

                    match (dltype, constants.len()) {
                        (DLType::BaseRole, 2) => {
                            let a1 = constants[0].trim();
                            let a2 = constants[1].trim();

                            // before augmenting current_id we need to know that the elements are not in symbols
                            to_be_added = Vec::new();
                            let node1: NodeDllite;
                            let node2: NodeDllite;

                            // each nominal
                            if !symbols.contains_key(a1) {
                                node1 = NodeDllite::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a1.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id1, _) = symbols[a1];
                                node1 = NodeDllite::new(Some(id1), DLType::Nominal).unwrap();
                            }

                            // adding symbols whenever you can
                            while !to_be_added.is_empty() {
                                let (k, v) = to_be_added.pop().unwrap();
                                symbols.insert(k, v);
                            }

                            // then a2
                            if !symbols.contains_key(a2) {
                                node2 = NodeDllite::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a2.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id2, _) = symbols[a2];
                                node2 = NodeDllite::new(Some(id2), DLType::Nominal).unwrap();
                            }

                            let abi =
                                AbiDllite::new_ra(abi_symbol.clone(), node1, node2, for_completion)
                                    .unwrap();

                            (Ok((abi, to_be_added)), current_id)
                        }
                        (DLType::BaseConcept, 1) => {
                            let a1 = constants[0].trim();

                            // before augmenting current_id we need to know that the elements are not in symbols
                            to_be_added = Vec::new();
                            let node1: NodeDllite;

                            // each nominal
                            if !symbols.contains_key(a1) {
                                node1 = NodeDllite::new(Some(current_id), DLType::Nominal).unwrap();

                                to_be_added.push((a1.to_string(), (current_id, DLType::Nominal)));
                                current_id += 1;
                            } else {
                                let (id1, _) = symbols[a1];
                                node1 = NodeDllite::new(Some(id1), DLType::Nominal).unwrap();
                            }

                            let abi = AbiDllite::new_ca(abi_symbol.clone(), node1, for_completion)
                                .unwrap();

                            (Ok((abi, to_be_added)), current_id)
                        }
                        (_, _) => (
                            invalid_data_result(
                                format!(
                                    "incompatible type for abox item with number of elements: {}",
                                    s
                                )
                                .as_str(),
                            ),
                            current_id,
                        ),
                    }
                }
            }
        } else {
            (
                invalid_data_result(
                    format!("unknown symbol in abox item: {}", abox_symbol).as_str(),
                ),
                current_id,
            )
        }
    }
}

pub fn abi_to_string(abi: &AbiDllite, symbols: &SymbolDict) -> Option<String> {
    match abi {
        AbiDllite::CA(c, a) => {
            let c_str_op = node_to_string(c, symbols, "".to_string());
            let a_str_op = node_to_string(a, symbols, "".to_string());

            match (c_str_op, a_str_op) {
                (Some(c_str), Some(a_str)) => {
                    let mut res = String::new();
                    res.push_str(a_str.as_str());
                    res.push_str(" : ");
                    res.push_str(c_str.as_str());

                    Some(res)
                }
                (_, _) => Option::None,
            }
        }
        AbiDllite::RA(r, a, b) => {
            let r_str_op = node_to_string(r, symbols, "".to_string());
            let a_str_op = node_to_string(a, symbols, "".to_string());
            let b_str_op = node_to_string(b, symbols, "".to_string());

            match (r_str_op, a_str_op, b_str_op) {
                (Some(r_str), Some(a_str), Some(b_str)) => {
                    let res = format!("{}, {}: {}", a_str, b_str, r_str);
                    Some(res)
                }
                (_, _, _) => Option::None,
            }
        }
    }
}

//-------------------------------------------------------------------------------------------------
// real functions live here
fn __parse_string_to_node_helper(
    splitted: Vec<&str>,
    symbols: &SymbolDict,
) -> io::Result<NodeDllite> {
    match splitted.len() {
        1 => {
            /*
            here are only base symbols
             */

            if symbols.contains_key(splitted[0]) {
                let value = symbols[splitted[0]];
                let new_node = NodeDllite::new(Some(value.0), value.1).unwrap();

                Ok(new_node)
            } else {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!("this symbols is not recognized '{}'", splitted[0]),
                );
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
                "NOT" => |x: NodeDllite| Some(x.negate()),
                "INV" => NodeDllite::inverse,
                "EXISTS" => NodeDllite::exists,
                _ => |_: NodeDllite| Option::None,
            };

            let base_node_result = __parse_string_to_node_helper(vec![splitted[1]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => Ok(complex_node),
                        _ => {
                            let new_error = Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "couldn't build a valid node with this combination {} and {}",
                                    splitted[0], splitted[1]
                                ),
                            );
                            Err(new_error)
                        }
                    }
                }
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
                "NOT" => |x: NodeDllite| Some(x.negate()),
                "EXISTS" => NodeDllite::exists,
                _ => |_: NodeDllite| Option::None,
            };
            let base_node_result =
                __parse_string_to_node_helper(vec![splitted[1], splitted[2]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => Ok(complex_node),
                        _ => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't build a valid node with this combination {}, {} and {}", splitted[0], splitted[1], splitted[2]));
                            Err(new_error)
                        }
                    }
                }
                Err(_) => base_node_result,
            }
        }
        4 => {
            /*
            here we can only have
            NOT EXISTS INV r
             */
            let function_to_call = match splitted[0] {
                "NOT" => |x: NodeDllite| Some(x.negate()),
                _ => |_: NodeDllite| Option::None,
            };
            let base_node_result =
                __parse_string_to_node_helper(vec![splitted[1], splitted[2], splitted[3]], symbols);

            match base_node_result {
                Ok(basenode) => {
                    let complex_node_op = function_to_call(basenode);

                    match complex_node_op {
                        Some(complex_node) => Ok(complex_node),
                        _ => {
                            let new_error = Error::new(ErrorKind::InvalidData, format!("couldn't build a valid node with this combination {}, {}, {} and {}", splitted[0], splitted[1], splitted[2], splitted[3]));
                            Err(new_error)
                        }
                    }
                }
                Err(_) => base_node_result,
            }
        }
        _ => {
            let new_error = Error::new(
                ErrorKind::InvalidData,
                format!("invalid input: {:?}", splitted),
            );
            Err(new_error)
        }
    }
}

fn find_keys_for_value(symbols: &SymbolDict, value: usize) -> Vec<String> {
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
        } else if (self.t.is_concept_type() && !other.t.is_concept_type())
            || (self.t.is_role_type() && other.t.is_nominal_type())
        {
            Some(Ordering::Less)
        } else if (self.t.is_nominal_type() && !other.t.is_nominal_type())
            || (self.t.is_role_type() && other.t.is_concept_type())
        {
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

// this approach is a dynamic one, concepts must be present in symbols,
// but nominals are added dynamically
pub fn string_to_abiq(
    s: &str,
    symbols: &mut SymbolDict,
    current_id: usize,
    for_completion: bool,
) -> AbiqDlliteParseResult {
    let splitted = s.trim();

    // a normal abi is splitted by ':', here we split by ';' first
    let splitted: Vec<&str> = splitted.split(',').collect();
    let splitted: Vec<&str> = splitted.iter().map(|x| x.trim()).collect();
    let lenght = splitted.len();

    let mut new_s = splitted[0].to_string();

    let (for_abi, pvalue, value) = match lenght {
        1 => (s.to_string(), 1.0, Option::None),
        2 => {
            let possible_pv = splitted[1].parse::<f64>();

            match possible_pv {
                Err(_) => {
                    new_s.push(',');
                    new_s.push_str(splitted[1]);

                    (new_s, 1.0, Option::None)
                }
                Ok(n) => (new_s, n, Option::None),
            }
        }
        3 => {
            let possible_pv = splitted[1].parse::<f64>();

            match possible_pv {
                Err(_) => {
                    // the second term is not a number
                    new_s.push(',');
                    new_s.push_str(splitted[1]);

                    // now parse the rest
                    let pv_res = splitted[2].parse::<f64>();
                    let v = Option::None;

                    match pv_res {
                        Err(_) => (new_s, 1.0, v),
                        Ok(n) => (new_s, n, v),
                    }
                }
                Ok(n) => {
                    // it is indeed a number
                    let v_res = splitted[2].parse::<f64>();

                    match v_res {
                        Err(_) => (new_s, n, Option::None),
                        Ok(nvalue) => (new_s, n, Some(nvalue)),
                    }
                }
            }
        }
        _ => {
            new_s.push(',');
            new_s.push_str(splitted[1]);

            let pv_res = splitted[2].parse::<f64>();
            let v_res = splitted[3].parse::<f64>();

            match (pv_res, v_res) {
                (Err(_), Err(_)) => (new_s, 1.0, Option::None),
                (Err(_), Ok(nvalue)) => (new_s, 1.0, Some(nvalue)),
                (Ok(npvalue), Err(_)) => (new_s, npvalue, Option::None),
                (Ok(npvalue), Ok(nvalue)) => (new_s, npvalue, Some(nvalue)),
            }
        }
    };

    // println!("---- len: {}\n     abi: {}\n     pv: {}\n     v: {:?}", splitted.len(), for_abi, pvalue, value);

    let (abi_res, cid) = string_to_abi(&for_abi, symbols, current_id, for_completion);

    match abi_res {
        Err(e) => (Err(e), cid),
        Ok((abi, v)) => {
            // if we are parsing then the level is forcefully 0
            let level = 0;
            let abiq = AbiqDllite::new(abi, Some(pvalue), value, level);

            (Ok((abiq, v)), cid)
        }
    }
}

pub fn abiq_to_string(abiq: &AbiqDllite, symbols: &SymbolDict, to_native: bool) -> Option<String> {
    let abi_to_string = abi_to_string(abiq.abi(), symbols);

    let pv_to_string = format!("{}", abiq.prevalue());
    let v_to_string = match abiq.value() {
        Option::None => "NC".to_string(),
        Some(v) => format!("{}", v),
    };

    match abi_to_string {
        Option::None => Option::None,
        Some(s) => {
            let res: String;

            if !to_native {
                res = format!("{}, (pv: {}, v: {})", s, pv_to_string, v_to_string);
            } else {
                res = match abiq.value() {
                    Option::None => format!("{}, {}", s, pv_to_string),
                    Some(v) => format!("{}, {}, {}", s, pv_to_string, v),
                };
            }
            Some(res)
        }
    }
}

pub fn pretty_print_abiq_conflict(
    conflict_tuple: (&TbiDllite, &Vec<AbiqDllite>),
    symbols: &SymbolDict,
) -> String {
    /*
    quelque chose comme
    [
        tbi: fdkfjldfj
        abisq: dlgjglkfjglfj
               dlkjgljfglfkjgljg
               flgjflkgjlfkgj
    ]
     */

    let mut s = String::from("  {\n");
    let to_native = false;

    let tbi = conflict_tuple.0;
    let v_abiq = conflict_tuple.1;
    let v_abiq_len = v_abiq.len();

    let tbi_string = tbi_to_string(tbi, symbols).unwrap();

    s.push_str("      tbi: ");
    s.push_str(&tbi_string);
    s.push('\n');

    let abiq_zero_string = abiq_to_string(v_abiq.get(0).unwrap(), symbols, to_native).unwrap();

    s.push_str("      abis: ");
    s.push_str(&abiq_zero_string);
    s.push('\n');

    for i in 1..v_abiq_len {
        let abiq_string = abiq_to_string(v_abiq.get(i).unwrap(), symbols, to_native).unwrap();

        s.push_str("            ");
        s.push_str(&abiq_string);
        s.push('\n');
    }

    s.push_str("  },");
    s
}

pub fn pretty_vector_tbi_to_string(vec: &[TbiDllite], symbols: &SymbolDict) -> String {
    let mut s = String::from("[");

    let vec_len = vec.len();
    let mut tbi_string_op: Option<String>;
    let _tbi_string: String;

    for i in 0..vec_len {
        tbi_string_op = tbi_to_string(vec.get(i).unwrap(), symbols);

        if let Some(some_tbi) = tbi_string_op {
            s.push_str(&some_tbi);
            s.push_str(", ");
        }
    }

    s.push(']');
    s
}

pub fn pretty_vector_abiq_to_string(vec: &[AbiqDllite], symbols: &SymbolDict) -> String {
    let mut s = String::from("[");
    let to_native = false;
    let vec_len = vec.len();
    let mut tbi_string_op: Option<String>;
    let _tbi_string: String;

    for i in 0..vec_len {
        tbi_string_op = abiq_to_string(vec.get(i).unwrap(), symbols, to_native);

        if let Some(some_tbi) = tbi_string_op {
            s.push_str(&some_tbi);
            s.push_str(", ");
        }
    }

    s.push(']');
    s
}

pub fn create_string_for_gencontb(
    tb: &TBDllite,
    symbols: &SymbolDict,
    dont_write_trivial: bool,
    _verbose: bool,
) -> String {
    let mut s = String::new();
    let mut temp_s: String;
    let new_tb = tb;

    // first compute all levels
    let _levels = new_tb.levels();
    let max_level = new_tb.get_max_level();

    s.push_str("[\n");

    for level in 0..(max_level + 1) {
        temp_s = format!("  level {}: {{\n", level);
        s.push_str(&temp_s);

        for tbi in new_tb.items() {
            if (!tbi.is_trivial() || !dont_write_trivial) && tbi.level() == level {
                s.push_str("    {\n");

                let tbi_to_string = tbi_to_string(tbi, symbols).unwrap();

                temp_s = format!("      tbi: {}\n", &tbi_to_string);
                s.push_str(&temp_s);

                if level > 0 {
                    let impliers = tbi.implied_by();
                    let len_impliers = impliers.len();

                    let mut implier_string =
                        pretty_vector_tbi_to_string(&(impliers.get(0).unwrap().1), symbols);

                    temp_s = format!("      impliers: {}\n", &implier_string);
                    s.push_str(&temp_s);

                    for i in 1..len_impliers {
                        implier_string =
                            pretty_vector_tbi_to_string(&(impliers.get(i).unwrap().1), symbols);

                        temp_s = format!("                {}\n", &implier_string);
                        s.push_str(&temp_s);
                    }
                }
                s.push_str("    },\n");
            }
        }

        s.push_str("  },\n");
    }

    s.push(']');
    s
}

pub fn create_string_for_unravel_conflict_tbi(
    tbi: &TbiDllite,
    symbols: &SymbolDict,
    pad: usize,
) -> String {
    let mut s = format!("{}{}", space_string(pad + 4), "{\n"); // String::from("    {\n");
    let mut temp_s: String;

    let tbi_to_string = tbi_to_string(tbi, symbols).unwrap();

    temp_s = format!("{}tbi: {}\n", space_string(pad + 6), &tbi_to_string,);
    s.push_str(&temp_s);

    let impliers = tbi.implied_by();
    let len_impliers = impliers.len();

    if len_impliers > 0 {
        let mut implier_string =
            pretty_vector_tbi_to_string(&(impliers.get(0).unwrap().1), symbols);

        temp_s = format!("{}impliers: {}\n", space_string(pad + 6), &implier_string);
        s.push_str(&temp_s);

        for i in 1..len_impliers {
            implier_string = pretty_vector_tbi_to_string(&(impliers.get(i).unwrap().1), symbols);

            temp_s = format!("{}{}\n", space_string(pad + 16), &implier_string);
            s.push_str(&temp_s);
        }

        for i in 0..len_impliers {
            // here this is bad, we must first create the string to put in
            let mut implier_s = String::from("");
            let this_impliers = impliers.get(i).unwrap();
            for implier in &(this_impliers.1) {
                if implier.level() > 0 {
                    // if not, no need to unravel a grounded axiom
                    temp_s = create_string_for_unravel_conflict_tbi(implier, symbols, pad + 4);
                    implier_s.push_str(&temp_s)
                }
            }

            if !implier_s.is_empty() && implier_s.as_str() != "\n" {
                temp_s = format!("{}impliers {}\n", space_string(pad + 6), i + 1);
                s.push_str(&temp_s);
                s.push_str(&implier_s);
            }
        }
    }

    temp_s = format!("{}}}\n", space_string(pad + 4));
    s.push_str(&temp_s);
    s
}

pub fn create_string_for_unravel_conflict_tbox(
    tb: &TBDllite,
    symbols: &SymbolDict,
    only_conflicts: bool,
) -> String {
    let max_level = tb.get_max_level();
    let mut s = String::from("");
    let mut temp_s: String;
    let mut actual_level: usize;
    let tbis_by_level = tb.get_tbis_by_level(only_conflicts);
    let pad: usize = 0;

    s.push_str("[\n");

    for lev in 0..(max_level + 1) {
        actual_level = max_level - lev;

        if tbis_by_level[actual_level] > 0 {
            let mut inner_temp = String::new();

            for tbi in tb.items() {
                if (tbi.level() == actual_level)
                    && (tbi.is_contradiction() || !only_conflicts)
                    && !tbi.is_trivial()
                {
                    temp_s = create_string_for_unravel_conflict_tbi(tbi, symbols, pad);
                    inner_temp.push_str(&temp_s);
                }
            }

            if !inner_temp.is_empty() {
                temp_s = format!("  level {}: {{\n", actual_level);
                s.push_str(&temp_s);

                s.push_str(&inner_temp);
                s.push_str("  },\n");
            }
        }
    }

    s.push(']');
    s
}
pub fn create_string_for_unravel_conflict_abiq(
    abiq: &AbiqDllite,
    symbols: &SymbolDict,
    pad: usize,
) -> String {
    let mut s = format!("{}{}", space_string(pad + 4), "{\n"); // String::from("    {\n");
    let mut temp_s: String;
    let to_native = false;

    let abiq_to_string = abiq_to_string(abiq, symbols, to_native).unwrap();

    temp_s = format!("{}abi: {}\n", space_string(pad + 6), &abiq_to_string,);
    s.push_str(&temp_s);

    let impliers = abiq.implied_by();
    let len_impliers = impliers.len();

    if len_impliers > 0 {
        let mut current_implier = impliers.get(0).unwrap();
        let mut tbis = &current_implier.1;
        let mut abis = &current_implier.2;

        let mut tbis_string = pretty_vector_tbi_to_string(tbis, symbols);
        let mut abis_string = pretty_vector_abiq_to_string(abis, symbols);

        temp_s = match (&tbis_string).as_str() {
            "[]" => {
                format!(
                    "{}impliers:\n{}{}: abis: {}\n",
                    space_string(pad + 6),
                    space_string(pad + 12),
                    1,
                    &abis_string
                )
            }
            _ => {
                format!(
                    "{}impliers:\n{}{}: tbis: {}\n{}{}: abis: {}\n",
                    space_string(pad + 6),
                    space_string(pad + 12),
                    1,
                    &tbis_string,
                    space_string(pad + 12),
                    1,
                    &abis_string
                )
            }
        };

        s.push_str(&temp_s);

        for i in 1..len_impliers {
            current_implier = impliers.get(i).unwrap();
            tbis = &current_implier.1;
            abis = &current_implier.2;
            tbis_string = pretty_vector_tbi_to_string(tbis, symbols);
            abis_string = pretty_vector_abiq_to_string(abis, symbols);

            temp_s = match (&tbis_string).as_str() {
                "[]" => {
                    format!(
                        "{}impliers:\n{}{}: abis: {}\n",
                        space_string(pad + 6),
                        space_string(pad + 12),
                        1,
                        &abis_string
                    )
                }
                _ => {
                    format!(
                        "{}impliers:\n{}{}: tbis: {}\n{}{}: abis: {}\n",
                        space_string(pad + 6),
                        space_string(pad + 12),
                        1,
                        &tbis_string,
                        space_string(pad + 12),
                        1,
                        &abis_string
                    )
                }
            };

            s.push_str(&temp_s);
        }

        for i in 0..len_impliers {
            // here this is bad, we must first create the string to put in
            let mut implier_s = String::from("");
            let this_impliers = impliers.get(i).unwrap();
            let abis = &this_impliers.2;
            for implier in abis {
                if implier.level() > 0 {
                    // if not, no need to unravel a grounded axiom
                    temp_s = create_string_for_unravel_conflict_abiq(implier, symbols, pad + 4);
                    implier_s.push_str(&temp_s)
                }
            }

            if !implier_s.is_empty() && implier_s.as_str() != "\n" {
                temp_s = format!("{}impliers {}\n", space_string(pad + 6), i + 1);
                s.push_str(&temp_s);
                s.push_str(&implier_s);
            }
        }
    }

    temp_s = format!("{}}}\n", space_string(pad + 4));
    s.push_str(&temp_s);
    s
}

pub fn create_string_for_unravel_conflict_abox(
    tb: &TBDllite,
    ab: &AbqDllite,
    symbols: &SymbolDict,
    only_conflicts: bool,
    contradictions: &[(TbiDllite, Vec<AbiqDllite>)],
) -> String {
    // we only consider abis_contradictions if only_conflicts is present

    let max_level = ab.get_max_level();
    let mut s = String::from("");
    let mut temp_s: String;
    let mut actual_level: usize;
    let abis_by_level = ab.get_abis_by_level(tb, only_conflicts, contradictions);
    let pad: usize = 0;

    s.push_str("[\n");

    for lev in 0..(max_level + 1) {
        actual_level = max_level - lev;

        if abis_by_level[actual_level] > 0 {
            temp_s = format!("  level {}: {{\n", actual_level);
            s.push_str(&temp_s);

            for abi in ab.items() {
                if (abi.level() == actual_level)
                    && !abi.is_trivial()
                    && (!only_conflicts || abiq_in_vec_of_vec(abi, contradictions))
                {
                    temp_s = create_string_for_unravel_conflict_abiq(abi, symbols, pad);
                    s.push_str(&temp_s);
                }
            }
            s.push_str("  },\n");
        }
    }

    s.push(']');
    s
}

pub fn abiq_in_vec_of_vec(abiq: &AbiqDllite, v: &[(TbiDllite, Vec<AbiqDllite>)]) -> bool {
    let mut abiq_vec: &Vec<AbiqDllite>;

    for inner_v in v {
        abiq_vec = &inner_v.1;

        if abiq_vec.contains(abiq) {
            return true;
        }
    }
    false
}

fn space_string(l: usize) -> String {
    let v = vec![""; l];
    v.join(" ")
}
