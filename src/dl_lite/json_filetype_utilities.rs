use std::{fs, io};

use serde_json::{Result, Value};

// use crate::dl_lite::abox::AB;
use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::node::Node;
// use crate::dl_lite::node::Mod;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
// use serde::Deserializer;
use crate::dl_lite::string_formatter::{node_to_string, string_to_node};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

/*
how this works:
 you need one or several files,
 symbols: it will take the real name of symbols and put them in a dictionary (map, or vec of tuples)
          that way the whole system works with integers and after we can translate back to names
 */

pub fn parse_value_to_string(v: &Value) -> io::Result<&str> {
    match v {
        Value::String(s) => Ok(s.as_str()),
        _ => invalid_data_result(format!("not a string json type: {}", v).as_str()),
    }
}

pub fn parse_symbol(
    s_vec: &Vec<Value>,
    latest: usize,
) -> (io::Result<(&str, usize, DLType)>, usize) {
    /*
    how parsing works: ["r", "myrole"] -> ("myrole", number, DLType::RoleBase)
     */
    if s_vec.len() != 2 {
        let new_error = Error::new(
            ErrorKind::InvalidData,
            format!("bad number of elements in list: {}", s_vec.len()).as_str(),
        );
        (Err(new_error), latest)
    } else {
        let t_result = parse_value_to_string(&s_vec[0]);
        let name_result = parse_value_to_string(&s_vec[1]);

        match (t_result, name_result) {
            (Err(e1), Err(e2)) => {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!(
                        "multiples errors ocurred e1: {}, e2: {}",
                        e1.to_string(),
                        e2.to_string()
                    ),
                );
                (Err(new_error), latest)
            }
            (Err(e1), _) => {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!("couldn't trasform to string: {}", e1.to_string()),
                );
                (Err(new_error), latest)
            }
            (_, Err(e2)) => {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!("couldn't trasform to string: {}", e2.to_string()),
                );
                (Err(new_error), latest)
            }
            (Ok(t), Ok(mut name)) => {
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
                    let new_error = Error::new(
                        ErrorKind::InvalidData,
                        format!("not a valid dl type: {}", &t),
                    );
                    (Err(new_error), latest)
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

                    (Ok((name, id, dlt.unwrap())), new_latest)
                }
            }
        }
    }
}

pub fn parse_value_to_tbi(
    value: &Value,
    symbols: &HashMap<String, (usize, DLType)>,
    _verbose: bool,
) -> io::Result<TBI> {
    match value {
        Value::Array(vec_of_values) => {
            if vec_of_values.len() != 2 {
                let new_error = Error::new(
                    ErrorKind::InvalidData,
                    format!("bad number of elements on list: {}", &vec_of_values.len()),
                );
                Err(new_error)
            } else {
                let lside_result = parse_value_to_string(&vec_of_values[0]);
                let rside_result = parse_value_to_string(&vec_of_values[1]);

                match (lside_result, rside_result) {
                    (Err(e1), Err(e2)) => {
                        let new_error = Error::new(
                            ErrorKind::InvalidData,
                            format!(
                                "multiples errors ocurred e1: {}, e2: {}",
                                e1.to_string(),
                                e2.to_string()
                            ),
                        );
                        Err(new_error)
                    }
                    (Err(e1), _) => {
                        let new_error = Error::new(
                            ErrorKind::InvalidData,
                            format!("couldn't trasform to string: {}", e1.to_string()),
                        );
                        Err(new_error)
                    }
                    (_, Err(e2)) => {
                        let new_error = Error::new(
                            ErrorKind::InvalidData,
                            format!("couldn't trasform to string: {}", e2.to_string()),
                        );
                        Err(new_error)
                    }
                    (Ok(lside), Ok(rside)) => {
                        let lside = string_to_node(lside, symbols);
                        let rside = string_to_node(rside, symbols);

                        match (&lside, &rside) {
                            (Err(e1), Err(e2)) => Err(Error::new(
                                ErrorKind::InvalidData,
                                format!(
                                    "mutiple errors, e1: {}, e2: {}",
                                    e1.to_string(),
                                    e2.to_string()
                                ),
                            )),
                            (Err(e1), _) => Err(Error::new(ErrorKind::InvalidData, e1.to_string())),
                            (_, Err(e2)) => Err(Error::new(ErrorKind::InvalidData, e2.to_string())),
                            (Ok(ls), Ok(rs)) => {
                                let new_tbi_op = TBI::new(ls.clone(), rs.clone());

                                match new_tbi_op {
                                    Some(new_tbi) => Ok(new_tbi),
                                    _ => {
                                        let new_error = Error::new(
                                            ErrorKind::InvalidData,
                                            format!("invalid syntax: {}", value),
                                        );
                                        Err(new_error)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => invalid_data_result(format!("not valid type of json value: {}", &value).as_str()),
    }
}

pub fn parse_value_abi(_value: &Value, _symbols: Vec<&Node>) -> Option<ABI> {
    Option::None
}

// when manipulating use &str to avoid unnecessary copies and when returning the data
// then use String
pub fn parse_symbols_json(filename: &str) -> io::Result<HashMap<String, (usize, DLType)>> {
    let data = fs::read_to_string(filename);

    // here I have to precise from where the 'Result' enum comes from
    match data {
        std::result::Result::Err(e) => {
            println!("something went wrong: {}", e);
            let new_error = Error::new(e.kind(), e.to_string());
            Err(new_error)
        }
        std::result::Result::Ok(data_string) => {
            let result_value: Result<Value> = serde_json::from_str(data_string.as_str());

            match result_value {
                Result::Err(error) => {
                    let new_error = Error::new(
                        ErrorKind::InvalidData,
                        format!("something went wrong during parsing: {}", error.to_string()),
                    );
                    Err(new_error)
                }
                Result::Ok(value) => match &value {
                    Value::Object(map) => {
                        if map.contains_key("symbols") {
                            let value_array = &map["symbols"];

                            match value_array {
                                Value::Array(vec_of_values) => {
                                    let mut latest: usize = 2;
                                    let mut result: (io::Result<(&str, usize, DLType)>, usize);
                                    let mut parsed_result: io::Result<(&str, usize, DLType)>;
                                    let _unwrapped_parsed: (&str, usize, DLType);

                                    let mut symbols: HashMap<String, (usize, DLType)> =
                                        HashMap::new();

                                    for value in vec_of_values {
                                        match value {
                                            Value::Array(symbol_spec) => {
                                                result = parse_symbol(symbol_spec, latest);

                                                parsed_result = result.0;
                                                latest = result.1;

                                                match parsed_result {
                                                    Err(error) => {
                                                        println!(
                                                            "couldn't add symbol: {}",
                                                            &error.to_string()
                                                        );
                                                        // result_from_error(&error)
                                                    }
                                                    Ok(parsed) => {
                                                        symbols.insert(
                                                            String::from(parsed.0),
                                                            (parsed.1, parsed.2),
                                                        );
                                                    }
                                                }
                                            }
                                            _ => (), // invalid_data_result(format!("not a list item: {}", &value).as_str()),
                                        }
                                    }

                                    if symbols.len() == 0 {
                                        invalid_data_result("not symbols were added")
                                    } else {
                                        Ok(symbols)
                                    }
                                }
                                _ => invalid_data_result(
                                    format!("not a list item: {}", &value_array).as_str(),
                                ),
                            }
                        } else {
                            println!("not symbols in this file: {}", &value);
                            invalid_data_result(
                                format!("no symbols in this file: {}", &value).as_str(),
                            )
                        }
                    }
                    _ => invalid_data_result(format!("not a map item: {}", &value).as_str()),
                },
            }
        }
    }
}

pub fn parse_tbox_json(
    filename: &str,
    symbols: &HashMap<String, (usize, DLType)>,
    verbose: bool,
) -> io::Result<TB> {
    let data = fs::read_to_string(filename);

    // here I have to precise from where the 'Result' enum comes from
    match data {
        std::result::Result::Err(error) => {
            println!("something went wrong: {}", &error);
            result_from_error(&error)
        }
        std::result::Result::Ok(data_string) => {
            let result_value: Result<Value> = serde_json::from_str(data_string.as_str());

            match result_value {
                Result::Err(error) => {
                    if verbose {
                        println!(
                            " -- json_utilities::parse_tbox_json: something went wrong: {}",
                            &error
                        );
                    }

                    let new_error = Error::new(
                        ErrorKind::InvalidData,
                        format!("couldn't parse the file: {}", &error.to_string()),
                    );
                    Err(new_error)
                }
                Result::Ok(value) => {
                    match &value {
                        Value::Object(map) => {
                            if map.contains_key("tbox") {
                                let value_array = &map["tbox"];

                                match value_array {
                                    Value::Array(vec_of_values) => {
                                        let mut tb = TB::new();
                                        let mut tbi_result: io::Result<TBI>;

                                        for v in vec_of_values {
                                            tbi_result = parse_value_to_tbi(v, symbols, verbose);

                                            match tbi_result {
                                                Err(error) => {
                                                    if verbose {
                                                        println!(
                                                        " -- json_utilities::parse_tbox_jons: couldn't parse value: {}, error: {}",
                                                        value, &error
                                                    );
                                                    }
                                                }
                                                Ok(tbi) => {
                                                    tb.add(tbi);
                                                }
                                            };
                                        }

                                        Ok(tb)
                                    }
                                    _ => invalid_data_result(
                                        format!("not a list of values: {}", &value_array).as_str(),
                                    ),
                                }
                            } else {
                                if verbose {
                                    println!(" -- json_utilities::parse_tbox_json: no tbox in this file: {}", &value);
                                }

                                invalid_data_result(
                                    format!("the file doesn't containt a 'tbox' item: {}", &value)
                                        .as_str(),
                                )
                            }
                        }
                        _ => invalid_data_result(format!("not a map item : {}", &value).as_str()),
                    }
                }
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

pub fn invalid_data_result<T>(error: &str) -> io::Result<T> {
    let new_error = Error::new(ErrorKind::InvalidData, error);
    Err(new_error)
}

pub fn result_from_error<T>(error: &Error) -> io::Result<T> {
    let new_error = Error::new(error.kind(), error.to_string());
    Err(new_error)
}
