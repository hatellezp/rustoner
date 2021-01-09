use std::{fs, fmt};

use serde_json::{Error, Result, Value};

use crate::dl_lite::abox::AB;
use crate::dl_lite::json_utilities::{parse_symbols_from_json, __parse_string_to_node_helper};
use crate::dl_lite::json_utilities::SJT;
use crate::dl_lite::node::{Node, Mod};
use crate::dl_lite::tbox::TB;
use crate::dl_lite::types::DLType;
use crate::kb::knowledge_base::{Axioms, Data, Interpreter};
use std::collections::HashMap;

use crate::dl_lite::json_utilities::parse_tbox_from_json;
use crate::dl_lite::tbox_item::TBI;

#[derive(Debug, Clone, PartialEq)]
pub struct Ontology {
    name: String,
    symbols: HashMap<String, (usize, DLType)>,
    tbox: TB,
    abox: AB,
}


impl fmt::Display for Ontology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut formatted: String;

        formatted = format!("Ontology<{}>: {{\n", &self.name);
        s.push_str(formatted.as_str());

        formatted = format!("--<Symbols>\n{}\n", Ontology::symbols_to_string(&self.symbols));
        s.push_str(formatted.as_str());

        formatted = format!("----<TBox>\n{}\n", &self.tbox_to_string(&self.tbox));
        s.push_str(formatted.as_str());

        formatted = format!("----{}\n", &self.abox);
        s.push_str(formatted.as_str());

        s.push_str("}");

        write!(f, "{}", s)
    }
}

impl Ontology {
    pub fn new(s: String) -> Ontology {
        let symbols: HashMap<String, (usize, DLType)> = HashMap::new();
        Ontology {
            name: s,
            symbols,
            tbox: TB::new(),
            abox: AB::new(),
        }
    }

    pub fn initialize_from_json(
        &mut self,
        tbox_filename: &str,
        symbols_filename: &str,
        verbose: bool,
    ) {
        let new_symbols = parse_symbols_from_json(symbols_filename);

        if new_symbols.is_some() {
            let new_symbols = new_symbols.unwrap();

            let new_tbox = parse_tbox_from_json(tbox_filename, &new_symbols, verbose);

            if new_tbox.is_some() {
                self.symbols = new_symbols;
                self.tbox = new_tbox.unwrap();
            }
        }
    }

    pub fn parse_abox_from_file_json(filename: &str) -> AB {
        AB::new()
    }

    pub fn symbols(&self) -> &HashMap<String, (usize, DLType)> {
        &self.symbols
    }

    pub fn tbox(&self) -> &TB {
        &self.tbox
    }

    pub fn abox(&self) -> &AB {
        &self.abox
    }

    pub fn complete_tbox(&self, verbose: bool) -> TB {
        let new_tb = self.tbox.complete2(verbose);

        new_tb
    }

    // this is helper function, but because is particular to symbols here defined I won't
    // move somewhere else
    fn symbols_to_string(symbols: &HashMap<String, (usize, DLType)>) -> String {
        let mut s = String::from("    {\n");

        for symbol in symbols {
            let (key, (integer, dltype)) = symbol;
            let symbol_formatted = format!("     - ({}: ({}, {})),\n", key, integer, dltype);

            s.push_str(symbol_formatted.as_str());
        }

        s.push_str("    }");
        s
    }

    fn tbox_to_string(&self, tb: &TB) -> String {
        let mut s = String::from("    {\n");

        for tbi in tb.items() {
            let tbi_string = self.tbi_to_string(tbi);
            let tbi_formatted = format!("     - {}\n", tbi_string);

            s.push_str(tbi_formatted.as_str());
        }

        s.push_str("    }");
        s
    }

    // I suppose that the tbi is in the self.tbox
    fn tbi_to_string(&self, tbi: &TBI) -> String {
        let lside = self.node_to_string(tbi.lside());
        let rside = self.node_to_string(tbi.rside());

        let s = format!("{} (<) {}", lside, rside);

        s
    }

    fn node_to_string(&self, node: &Node) -> String {
        let mut left_current = String::new();
        let mut right_current = String::new();

        self.node_to_string_helper(node, left_current, right_current)
    }

    fn node_to_string_helper(&self, node: &Node, mut left_current: String, mut right_current: String) -> String {
        match node {
            Node::T => format!("{}", node),
            Node::B => format!("{}", node),
            Node::N(n) | Node::R(n) | Node::C(n) => {
                // find the name
                let mut name_found = false;
                let mut name: String = String::new();

                for symbol in &self.symbols {
                    let (a, (b, c)) = symbol;

                    if b == n {
                        name_found = true;
                        name = a.clone();
                    }
                }

                if !name_found {
                    name = String::from("NAME NOT FOUND");
                }

                match node {
                    Node::N(_) => format!("{}{}{}", left_current, name, right_current),
                    Node::R(_) => format!("{}{}{}", left_current, name, right_current),
                    Node::C(_) => format!("{}{}{}", left_current, name, right_current),
                    _ => String::from("you should't be here"),

                    /*
                    Node::N(_) => format!("{} N({})", current, name),
                    Node::R(_) => format!("{} R({})", current, name),
                    Node::C(_) => format!("{} C({})", current, name),
                    _ => String::from("you should't be here"),

                     */
                }
            },
            Node::X(m, bn) => {
                let left_addition = match m {
                    Mod::N => "-",
                    Mod::I => "(",
                    Mod::E =>  "E.(",
                };

                let right_addition = match m {
                    Mod::N => "",
                    Mod::I => "^-)",
                    Mod::E =>  ")",
                };

                /*
                let addition = match m {
                    Mod::N => "NOT ",
                    Mod::I => "INV ",
                    Mod::E =>  "EXISTS ",
                };

                 */
                println!("l: {}  r: {}", &left_current, &right_current);

                left_current = format!("{}{}", left_addition, left_current);
                right_current = format!("{}{}", right_current, right_addition);

                println!("l: {}  r: {}", &left_current, &right_current);

                // current.push_str(addition);
                self.node_to_string_helper(bn, left_current, right_current)
            },
        }
    }
}

impl Interpreter for Ontology {
    fn parse_axioms_from_file(filename: &str) -> Box<dyn Axioms> {
        Box::new(TB::new())
    }

    fn parse_data_from_file(filename: &str) -> Box<dyn Data> {
        let abox = Ontology::parse_abox_from_file_json(filename);
        Box::new(abox)
    }
}
