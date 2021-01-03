use serde_json::{Result, Value};

use crate::node::Node;
use crate::tbox::TB;
use crate::abox::AB;
use crate::knowledge_base::{Interpreter, Axioms, Data};

pub struct Ontology {
    names: Vec<(String, usize)>,
    tbox: TB,
    abox: AB,
}

impl Ontology {
    pub fn new() -> Ontology {
        Ontology { names: Vec::new(), tbox: TB::new(), abox: AB::new() }
    }
    pub fn parse_tbox_from_file_json(filename: &str) -> TB {
        TB::new()
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