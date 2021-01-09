use std::fs;

use serde_json::{Error, Result, Value};

use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::node::Node;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::panic::resume_unwind;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum SJT {
    // stand for serde_json types
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

/*
how this works:
 you need one or several files,
 symbols: it will take the real name of symbols and put them in a dictionary (map, or vec of tuples)
          that way the whole system works with integers and after we can translate back to names
 */

pub fn parse_symbol(s_vec: &Vec<Value>, latest: usize) -> (Option<((String, usize), Node)>, usize) {
    if s_vec.len() != 2 {
        (Option::None, latest)
    } else {
        let t = parse_value_to_string(&s_vec[0]);
        let name = parse_value_to_string(&s_vec[1]);

        if t.is_none() || name.is_none() {
            (Option::None, latest)
        } else {
            let t = t.unwrap();
            let name = name.unwrap();

            let roles = ["r", "R", "role", "Role"];
            let concepts = ["c", "C", "concept", "Concept"];
            let nominals = ["n", "N", "nominal", "Nominal"];

            let dlt: Option<DLType>;
            let n: Node;

            if roles.contains(&t.as_str()) {
                dlt = Some(DLType::BaseRole);
            } else if concepts.contains(&t.as_str()) {
                dlt = Some(DLType::BaseConcept);
            } else if nominals.contains(&t.as_str()) {
                dlt = Some(DLType::Nominal);
            } else {
                dlt = Option::None;
            }

            if dlt.is_none() {
                (Option::None, latest)
            } else {
                let id = latest + 1;
                n = Node::new(Some(id), dlt.unwrap()).unwrap();

                (Some(((name, id), n)), id)
            }
        }
    }
}

pub fn parse_value_to_node(value: &Value) -> Option<Node> {
    Option::None
}

pub fn parse_value_to_tbi(value: &Value, symbols: Vec<&Node>) -> Option<TBI> {
    Option::None
}

pub fn parse_value_abi(value: &Value, symbols: Vec<&Node>) -> Option<ABI> {
    Option::None
}

pub fn best_print_value(v: &Value) {
    match value_type(v) {
        SJT::Null | SJT::Bool | SJT::Number | SJT::String | SJT::Array => println!("{}", v),
        SJT::Object => match v {
            Value::Object(map) => {
                for t in map {
                    let s = t.0;
                    let vv = t.1;
                    print!("{}: {{\n    ", s);
                    best_print_value(vv);
                    println!("}} \n");
                }
            }
            _ => (),
        },
        _ => println!("_"),
    }
}

fn value_type(v: &Value) -> SJT {
    match v {
        Value::Null => SJT::Null,
        Value::Bool(_) => SJT::Bool,
        Value::Number(_) => SJT::Number,
        Value::String(_) => SJT::String,
        Value::Array(_) => SJT::Array,
        Value::Object(_) => SJT::Object,
    }
}

pub fn parse_symbols_from_json(filename: &str) -> Option<Vec<((String, usize), Node)>> {
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
                        let value_array = &map["symbols"];

                        match value_array {
                            Value::Array(vec_of_values) => {
                                let mut vec: Vec<((String, usize), Node)> = Vec::new();
                                let mut latest: usize = 2;

                                for value in vec_of_values {
                                    match value {
                                        Value::Array(symbol_spec) => {
                                            let result = parse_symbol(symbol_spec, latest);

                                            let parsed = result.0;
                                            latest = result.1;

                                            if parsed.is_some() {
                                                vec.push(parsed.unwrap());
                                            }
                                        }
                                        _ => (),
                                    }
                                }

                                if vec.len() == 0 {
                                    Option::None
                                } else {
                                    Some(vec)
                                }
                            }
                            _ => Option::None,
                        }
                    }
                    _ => Option::None,
                },
            }
        }
    }
}

pub fn from_vec_of_values_to_vec_of_string(v: &Value) -> Option<Vec<String>> {
    match v {
        Value::Array(vec_of_values) => {
            let mut vec: Vec<String> = Vec::new();

            for value in vec_of_values {
                match value {
                    Value::String(s) => {
                        vec.push(s.clone());
                    }
                    _ => (),
                }
            }

            if vec.len() != 0 {
                Some(vec)
            } else {
                Option::None
            }
        }
        _ => Option::None,
    }
}

pub fn parse_value_to_string(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        _ => Option::None,
    }
}
