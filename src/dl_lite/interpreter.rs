use serde_json::{Result, Value};

use crate::dl_lite::abox::AB;
use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::kb::knowledge_base::{Axioms, Data, Interpreter};
use std::fs;

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

    pub fn parse_tbox_from_file_json(filename: &str) -> TB {
        let tb = TB::new();

        let data = fs::read_to_string(filename).expect("Unable to read file");

        let value: Value = serde_json::from_str(data.as_str()).expect("Badly parsed JSON");

        println!("{:?}", value["tbox"]);

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
