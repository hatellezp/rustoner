use std::collections::HashMap;
use crate::dl_lite::types::{DLType, FileType};
use crate::dl_lite::tbox::TB;
use crate::dl_lite::abox::AB;
use crate::dl_lite::abox_item::ABI;
use std::fmt;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::node::{Node, Mod};
use crate::dl_lite::json_utilities::{parse_symbols_from_json, parse_tbox_from_json};

/*
an ontology model
    - name is the name of the ontology
    - symbols is a dictionary where symbols are stored in the form symbol_name -> (id, type)
    - tbox is the tbox of the system
    - number_of_symbols is the current number of symbols
    - number_of_tbi is the current number of tbi
    - latest_id is higher number present in the symbols dictionary
 */
#[derive(PartialEq, Clone, Debug)]
pub struct Ontology {
    name: String,
    symbols: HashMap<String, (usize, DLType)>,
    tbox: TB,
    number_of_symbols: usize,
    number_of_tbi: usize,
    latest_id: usize,
}

impl fmt::Display for Ontology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut formatted: String;

        // add the name
        formatted = format!("Ontology<{}>: {{\n", &self.name);
        s.push_str(formatted.as_str());

        // add the symbols
        formatted = format!("--<Symbols>\n{}\n", Ontology::symbols_to_string(&self.symbols));
        s.push_str(formatted.as_str());

        // add the tbox
        formatted = format!("----<TBox>\n{}\n", &self.tbox_to_string(&self.tbox));
        s.push_str(formatted.as_str());

        // last bracket
        s.push_str("}");

        write!(f, "{}", s)
    }
}

impl Ontology {
    //-------------------------------------------------------------------------
    // public functions for the interface

    pub fn new(s: String) -> Ontology {
        let symbols: HashMap<String, (usize, DLType)> = HashMap::new();
        Ontology {
            name: s,
            symbols,
            tbox: TB::new(),
            number_of_symbols: 0,
            number_of_tbi: 0,
            latest_id: 0
        }
    }

    // ------------------------------------------------------------------------
    // modifications of the ontology

    pub fn add_symbols(&mut self, filename: &str, filetype: FileType) {
        match filetype {
            FileType::JSON => {
                let new_symbols_op = parse_symbols_from_json(filename);

                if new_symbols_op.is_some() {
                    let new_symbols = new_symbols_op.unwrap();

                    for (key, _) in &new_symbols {
                        self.add_symbol(&new_symbols, key);
                    }
                }
            },
            _ => panic!("error while trying to add from file, unimplemented for this file type!"),
        }
    }

    pub fn add_tbis(&mut self, filename: &str, filetype: FileType, verbose: bool) {
        if self.symbols.len() != 0 {
            match filetype {
                FileType::JSON => {
                    let tb_op = parse_tbox_from_json(filename, self.symbols(), verbose);

                    if tb_op.is_some() {
                        let tb = tb_op.unwrap();

                        for tbi in tb.items() {
                            self.add_tbi(tbi);
                        }
                    }
                },
                _ => panic!("error while trying to add from file, unimplemented for this file type!"),
            }
        }
        else {
            println!("warning: no symbols detected, no tbox item will be added");
        }
    }

    pub fn add_tbox(&mut self, tb: &TB) {
        for tbi in tb.items() {
            self.add_tbi(tbi);
        }
    }

    // ------------------------------------------------------------------------
    // reasoner tasks

    pub fn complete_tbox(&self) -> TB {
        TB::new()
    }

    pub fn is_satisfiable(&self) -> bool { false }

    pub fn find_consequences(&self, abox_filename: &str) -> Vec<ABI> {
        let vec: Vec<ABI> = Vec::new();

        vec
    }

    pub fn symbols(&self) -> &HashMap<String, (usize, DLType)> {
        &self.symbols
    }

    pub fn tbox(&self) -> &TB {
        &self.tbox
    }

    // this function returns two different sizes: symbol size and tbox size
    pub fn len(&self) -> (usize, usize) {
        (self.symbols.len(), self.tbox.len())
    }

    // ------------------------------------------------------------------------
    // private functions for the inner work of 'Ontology'

    fn add_symbol(&mut self, new_symbols: &HashMap<String, (usize, DLType)>, new_name: &String) -> bool {
        if new_symbols.contains_key(new_name) {
           if !self.symbols.contains_key(new_name) {
               let (_, t) = new_symbols[new_name];

               // we need to update the id to avoid conflict with the current numbers
               let (low, high) = Ontology::find_lower_and_highest_value_from_symbols(self.symbols());

               self.symbols.insert(new_name.clone(), (high+1, t));
               self.number_of_symbols += 1;

               true
           } else {
               false
           }
        } else {
            false
        }
    }

    fn add_tbi(&mut self, tbi: &TBI) -> bool {
        if self.tbox.contains(&tbi) {
            false
        } else {
            self.tbox.add(tbi.clone());
            self.number_of_tbi += 1;

            true
        }
    }

    fn find_lower_and_highest_value_from_symbols(symbols: &HashMap<String, (usize, DLType)>) -> (usize, usize) {
        let mut lowest: Option<usize> = Option::None;
        let mut highest: Option<usize> = Option::None;

        for symbol in symbols {
            let (_, (b, _)) = symbol;

            lowest = match lowest {
                Option::None => Some(*b),
                Some(old_b) => {
                    if *b < old_b {
                        Some(*b)
                    } else {
                        lowest
                    }
                },
            };

            highest = match highest {
                Option::None => Some(*b),
                Some(old_b) => {
                    if *b > old_b {
                        Some(*b)
                    } else {
                        highest
                    }
                },
            };
        }

        let (lowest, highest) = match (lowest, highest) {
            (Option::None, Option::None) => (2, 2),
            (Some(l), Some(h)) => (l, h),
            (_, _) => (2, 2),
        };

        (lowest, highest)
    }

    // ------------------------------------------------------------------------
    // pretty print functions
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
                // println!("l: {}  r: {}", &left_current, &right_current);

                left_current = format!("{}{}", left_addition, left_current);
                right_current = format!("{}{}", right_current, right_addition);

                // println!("l: {}  r: {}", &left_current, &right_current);

                // current.push_str(addition);
                self.node_to_string_helper(bn, left_current, right_current)
            },
        }
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
}