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

use serde_json::{Result, Value};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::{fs, io};

use crate::dl_lite::string_formatter::string_to_node;
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;

use crate::kb::knowledge_base::{SymbolDict, TBox};
use crate::kb::types::DLType;

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

pub fn parse_symbol(s_vec: &[Value], latest: usize) -> (io::Result<(&str, usize, DLType)>, usize) {
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

                match dlt {
                    Option::None => {
                        let new_error = Error::new(
                            ErrorKind::InvalidData,
                            format!("not a valid dl type: {}", &t),
                        );
                        (Err(new_error), latest)
                    }
                    Some(dlt_unwrapped) => {
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
}

pub fn parse_value_to_tbi(
    value: &Value,
    symbols: &SymbolDict,
    _verbose: bool,
) -> io::Result<TbiDllite> {
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
                                let level: usize = 0;
                                let new_tbi_op = TbiDllite::new(ls.clone(), rs.clone(), level);

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

/*
pub fn parse_value_abi(_value: &Value, _symbols: Vec<&Node_DLlite>) -> Option<AbiDllite> {
    Option::None
}

 */

// when manipulating use &str to avoid unnecessary copies and when returning the data
// then use String
pub fn parse_symbols_json(filename: &str) -> io::Result<SymbolDict> {
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

                                    let mut symbols: SymbolDict = HashMap::new();

                                    for value in vec_of_values {
                                        if let Value::Array(symbol_spec) = value {
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
                                    }

                                    if symbols.is_empty() {
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
    symbols: &SymbolDict,
    verbose: bool,
) -> io::Result<TBDllite> {
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
                                        let mut tb = TBDllite::new();
                                        let mut tbi_result: io::Result<TbiDllite>;

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

pub fn invalid_data_result<T>(error: &str) -> io::Result<T> {
    let new_error = Error::new(ErrorKind::InvalidData, error);
    Err(new_error)
}

pub fn result_from_error<T>(error: &Error) -> io::Result<T> {
    let new_error = Error::new(error.kind(), error.to_string());
    Err(new_error)
}
