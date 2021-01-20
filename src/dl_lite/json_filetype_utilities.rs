use std::fs;

use serde_json::{Result, Value};

use crate::dl_lite::abox::AB;
use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::node::{Mod, Node};
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use serde::Deserializer;
use std::collections::HashMap;
use crate::dl_lite::string_formatter::{node_to_string, string_to_node};

/*
how this works:
 you need one or several files,
 symbols: it will take the real name of symbols and put them in a dictionary (map, or vec of tuples)
          that way the whole system works with integers and after we can translate back to names
 */

pub fn parse_value_to_string(v: &Value) -> Option<&str> {
    match v {
        Value::String(s) => Some(s.as_str()),
        _ => Option::None,
    }
}

pub fn parse_symbol(s_vec: &Vec<Value>, latest: usize) -> (Option<(&str, usize, DLType)>, usize) {
    /*
    how parsing works: ["r", "myrole"] -> ("myrole", number, DLType::RoleBase)
     */
    if s_vec.len() != 2 {
        (Option::None, latest)
    } else {
        let t = parse_value_to_string(&s_vec[0]);
        let name = parse_value_to_string(&s_vec[1]);

        if t.is_none() || name.is_none() {
            (Option::None, latest)
        } else {
            let t = t.unwrap();
            let mut name = name.unwrap();

            /*
            this was a test, I'm giving now the correct values
            let roles = ["r", "R", "role", "Role"];
            let concepts = ["c", "C", "concept", "Concept"];
            let nominals = ["n", "N", "nominal", "Nominal"];
            */
            let roles = ["role"];
            let concepts = ["concept"];
            let nominals = ["nominal"];
            let top = ["Top", "top"];
            let bottom = ["Botom", "bottom"];

            let dlt: Option<DLType>;

            if roles.contains(&t) {
                dlt = Some(DLType::BaseRole);
            } else if concepts.contains(&t) {
                dlt = Some(DLType::BaseConcept);
            } else if nominals.contains(&t) {
                dlt = Some(DLType::Nominal);
            } else if top.contains(&t) {
                dlt = Some(DLType::Top)
            } else if bottom.contains(&t) {
                dlt = Some(DLType::Bottom)
            } else {
                dlt = Option::None;
            }

            if dlt.is_none() {
                (Option::None, latest)
            } else {
                let dlt_unwrapped = dlt.unwrap();
                let id: usize;
                let new_latest: usize;

                if dlt_unwrapped == DLType::Bottom {
                    name = "Bottom";
                    id = 0;
                    new_latest = latest;
                } else if dlt_unwrapped == DLType::Top {
                    name = "Top";
                    id = 1;
                    new_latest = latest;
                } else {
                    id = latest + 1;
                    new_latest = id;
                }

                (Some((name, id, dlt.unwrap())), new_latest) // TODO: verify this
            }
        }
    }
}

pub fn parse_value_to_tbi(
    value: &Value,
    symbols: &HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> Option<TBI> {
    match value {
        Value::Array(vec_of_values) => {
            if vec_of_values.len() != 2 {
                Option::None
            } else {
                let lside_op = parse_value_to_string(&vec_of_values[0]);
                let rside_op = parse_value_to_string(&vec_of_values[1]);

                match (&lside_op, &rside_op) {
                    (Option::Some(lside), Option::Some(rside)) => {
                        let lside = string_to_node(lside, symbols);
                        let rside = string_to_node(rside, symbols);

                        match (&lside, &rside) {
                            (Option::Some(ls), Option::Some(rs)) => {
                                let new_tbi = TBI::new(ls.clone(), rs.clone());

                                new_tbi
                            }
                            (_, _) => {
                                if verbose {
                                    println!("could't parse: {}", &value);
                                    Option::None
                                } else {
                                    Option::None
                                }
                            }
                        }
                    }
                    (_, _) => {
                        if verbose {
                            println!("could't parse: {}", &value);
                            Option::None
                        } else {
                            Option::None
                        }
                    }
                }
            }
        }
        _ => Option::None,
    }
}

pub fn parse_value_abi(value: &Value, symbols: Vec<&Node>) -> Option<ABI> {
    Option::None
}

// when manipulating use &str to avoid unnecessary copies and when returning the data
// then use String
pub fn parse_symbols_from_json(filename: &str) -> Option<HashMap<String, (usize, DLType)>> {
    let data = fs::read_to_string(filename);

    // here I have to precise from where the 'Result' enum comes from
    match data {
        std::result::Result::Err(e) => {
            println!("something went wrong: {}", e);
            Option::None
        }
        std::result::Result::Ok(data_string) => {
            let result_value: Result<Value> = serde_json::from_str(data_string.as_str());

            match result_value {
                Result::Err(e) => {
                    println!("something went wrong: {}", e);
                    Option::None
                }
                Result::Ok(value) => match &value {
                    Value::Object(map) => {
                        if map.contains_key("symbols") {
                            let value_array = &map["symbols"];

                            match value_array {
                                Value::Array(vec_of_values) => {
                                    let mut latest: usize = 2;
                                    let mut result: (Option<(&str, usize, DLType)>, usize);
                                    let mut parsed: Option<(&str, usize, DLType)>;
                                    let mut unwrapped_parsed: (&str, usize, DLType);

                                    let mut symbols: HashMap<String, (usize, DLType)> =
                                        HashMap::new();

                                    for value in vec_of_values {
                                        match value {
                                            Value::Array(symbol_spec) => {
                                                result = parse_symbol(symbol_spec, latest);

                                                parsed = result.0;
                                                latest = result.1;

                                                if parsed.is_some() {
                                                    unwrapped_parsed = parsed.unwrap();

                                                    symbols.insert(
                                                        String::from(unwrapped_parsed.0),
                                                        (unwrapped_parsed.1, unwrapped_parsed.2),
                                                    );
                                                }
                                            }
                                            _ => (),
                                        }
                                    }

                                    if symbols.len() == 0 {
                                        Option::None
                                    } else {
                                        Some(symbols)
                                    }
                                }
                                _ => Option::None,
                            }
                        } else {
                            println!("not symbols in this file: {}", &value);
                            Option::None
                        }
                    }
                    _ => Option::None,
                },
            }
        }
    }
}

pub fn parse_tbox_from_json(
    filename: &str,
    symbols: &HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> Option<TB> {
    let data = fs::read_to_string(filename);

    // here I have to precise from where the 'Result' enum comes from
    match data {
        std::result::Result::Err(e) => {
            println!("something went wrong: {}", e);
            Option::None
        }
        std::result::Result::Ok(data_string) => {
            let result_value: Result<Value> = serde_json::from_str(data_string.as_str());

            match result_value {
                Result::Err(e) => {
                    println!("something went wrong: {}", e);
                    Option::None
                }
                Result::Ok(value) => match &value {
                    Value::Object(map) => {
                        if map.contains_key("tbox") {
                            let value_array = &map["tbox"];

                            match value_array {
                                Value::Array(vec_of_values) => {
                                    let mut tb = TB::new();
                                    let mut tbi: Option<TBI>;

                                    for v in vec_of_values {
                                        tbi = parse_value_to_tbi(v, symbols, verbose);

                                        if tbi.is_some() {
                                            tb.add(tbi.unwrap());
                                        }
                                    }

                                    Some(tb)
                                }
                                _ => Option::None,
                            }
                        } else {
                            println!("no tbox in this file: {}", &value);
                            Option::None
                        }
                    }
                    _ => Option::None,
                },
            }
        }
    }
}

fn node_to_value(node: &Node, symbols: &HashMap<String, (usize, DLType)>) -> Option<Value> {
    let string_op = node_to_string(node, symbols, String::new());

    match string_op {
        Option::Some(s) => Some(Value::String(s)),
        _ => Option::None,
    }
}

fn tbi_to_value(tbi: &TBI, symbols: &HashMap<String, (usize, DLType)>) -> Option<Value> {
    let lside_op = node_to_value(tbi.lside(), symbols);
    let rside_op = node_to_value(tbi.rside(), symbols);

    match (lside_op, rside_op) {
        (Some(lside), Some(rside)) => Some(Value::Array(vec![lside, rside])),
        (_, _) => Option::None,
    }
}

pub fn tbox_to_value(
    tbox: &TB,
    symbols: &HashMap<String, (usize, DLType)>,
    dont_write_trivial: bool,
) -> Option<Value> {
    let mut vec_of_tbis: Vec<Value> = Vec::new();

    for tbi in tbox.items() {
        let tbi_op: Option<Value>;

        if dont_write_trivial && tbi.is_trivial() {
            tbi_op = Option::None
        } else {
            tbi_op = tbi_to_value(tbi, symbols);
        }

        match tbi_op {
            Some(tbi_value) => vec_of_tbis.push(tbi_value),
            _ => (),
        }
    }

    if vec_of_tbis.len() > 0 {
        Some(Value::Array(vec_of_tbis))
    } else {
        Option::None
    }
}
