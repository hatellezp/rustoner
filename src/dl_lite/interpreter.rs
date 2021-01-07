use serde_json::{Result, Value};

use crate::dl_lite::abox::AB;
use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::kb::knowledge_base::{Axioms, Data, Interpreter};
use std::fs;

pub enum SJT { // stand for serde_json types
    Null,
    Bool,
    Number,
    String,
    Array,
    Object,
}

pub struct Ontology {
    names: Vec<(String, usize)>,
    tbox: TB,
    abox: AB,
}

impl Ontology {
    pub fn new() -> Ontology {
        Ontology {
            names: Vec::new(),
            tbox: TB::new(),
            abox: AB::new(),
        }
    }

    fn print_value(v: &Value) {
        match v {
            Value::Null => println!("Null"),
            Value::Bool(b) => println!("Bool: {}", b),
            Value::Number(n) => println!("Number: {}", n),
            Value::String(s) => println!("String: {}", s),
            Value::Array(vec) => println!("Array: {:?}", v),
            Value::Object(map) => println!("Object: {:?}", map),
        }
    }

    fn best_print_value(v: &Value) {
        match Ontology::value_type(v) {
            SJT::Null | SJT::Bool | SJT::Number | SJT::String | SJT::Array => Ontology::print_value(v),
            SJT::Object => {
                match v {
                    Value::Object(map) => {
                        for t in map {
                            let s= t.0;
                            let vv = t.1;
                            print!("{}: {{\n    ", s);
                            Ontology::best_print_value(vv);
                            println!("}} \n");
                        }
                    },
                    _ => (),
                }
            }
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

    pub fn parse_tbox_from_file_json(filename: &str) -> TB {
        let tb = TB::new();

        let data = fs::read_to_string(filename).expect("Unable to read file");

        let value: Value = serde_json::from_str(data.as_str()).expect("Badly parsed JSON");

        Ontology::best_print_value(&value);
        // Ontology::print_value(&value["tbox"]);
        // Ontology::print_value(&value["symbols"]);

        tb
    }

    pub fn parse_abox_from_file_json(filename: &str) -> AB {
        AB::new()
    }

    pub fn names(&self) -> &Vec<(String, usize)> {
        &self.names
    }

    pub fn tbox(&self) -> &TB {
        &self.tbox
    }

    pub fn abox(&self) -> &AB {
        &self.abox
    }
}

impl Interpreter for Ontology {
    fn parse_axioms_from_file(filename: &str) -> Box<dyn Axioms> {
        let tbox = Ontology::parse_tbox_from_file_json(filename);
        Box::new(tbox)
    }

    fn parse_data_from_file(filename: &str) -> Box<dyn Data> {
        let abox = Ontology::parse_abox_from_file_json(filename);
        Box::new(abox)
    }
}
