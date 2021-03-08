use crate::shiq::abox_item::ABI_SHIQ;
use crate::shiq::json_filetype_utilities::{parse_symbols_json, parse_tbox_json, tbox_to_value};
use crate::shiq::node::{Mod, Node_SHIQ};
use crate::shiq::tbox::TB_SHIQ;
use crate::shiq::tbox_item::TBI_SHIQ;
use crate::kb::types::DLType;
use crate::kb::types::FileType;
use serde_json::json;
use std::collections::HashMap;
// use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::{fmt, io};
// use std::iter::Map;
// use crate::shiq::abox::AB;
use crate::shiq::native_filetype_utilities::{
    abox_to_native_string_quantum, parse_abox_native_quantum, parse_symbols_native,
    parse_tbox_native, tbox_to_native_string,
};

use rusqlite::{Connection, Result};

use crate::shiq::abox::ABQ_SHIQ;
use crate::shiq::abox_item_quantum::ABIQ_SHIQ;
use crate::shiq::sqlite_interface::{
    add_basic_tables_to_db, add_symbols_from_db, add_symbols_to_db, add_tbis_from_db,
    add_tbis_to_db,
};
use crate::interface::utilities::parse_name_from_filename;

// import traits
use crate::kb::knowledge_base::{ABoxItem, ABox, TBoxItem, TBox, SymbolDict};

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
pub struct Ontology_SHIQ {
    name: String,
    symbols: SymbolDict,
    tbox: TB_SHIQ,
    current_abox: Option<ABQ_SHIQ>,
}

impl fmt::Display for Ontology_SHIQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut formatted: String;

        // add the name
        formatted = format!("Ontology<{}>: {{\n", &self.name);
        s.push_str(formatted.as_str());

        // add the symbols
        formatted = format!(
            "--<Symbols>\n{}\n",
            Ontology_SHIQ::symbols_to_string(&self.symbols)
        );
        s.push_str(formatted.as_str());

        // add the tbox
        formatted = format!("----<TBox>\n{}\n", &self.tbox_to_string(&self.tbox, false));
        s.push_str(formatted.as_str());

        // add the tbox
        if self.current_abox.is_some() {
            formatted = format!(
                "----<ABox({})>\n{}\n",
                self.current_abox.as_ref().unwrap().name(),
                &self.abox_to_string_quantum(&self.current_abox.as_ref().unwrap())
            );
            s.push_str(formatted.as_str());
        }

        // last bracket
        s.push_str("}");

        write!(f, "{}", s)
    }
}

impl Ontology_SHIQ {
    //-------------------------------------------------------------------------
    // public functions for the interface

    pub fn new(s: String) -> Ontology_SHIQ {
        let mut symbols: SymbolDict = HashMap::new();

        /*
        bottom and top are added by default
         */
        symbols.insert(String::from("Top"), (1, DLType::Top));
        symbols.insert(String::from("Bottom"), (0, DLType::Bottom));

        Ontology_SHIQ {
            name: s,
            symbols,
            tbox: TB_SHIQ::new(),
            current_abox: Option::None,
        }
    }

    pub fn abox(&self) -> Option<&ABQ_SHIQ> {
        match &self.current_abox {
            Option::None => Option::None,
            Some(ab) => Some(ab),
        }
    }

    pub fn abox_name(&self) -> String {
        match &self.current_abox {
            Option::None => String::from("NONE"),
            Some(ab) => String::from(ab.name()),
        }
    }

    pub fn symbols_as_mut(&mut self) -> &mut SymbolDict {
        &mut self.symbols
    }
    // ------------------------------------------------------------------------
    // modifications of the ontology

    pub fn sort(&mut self) {
        self.tbox.sort();
    }

    // ----------------------------------------------------------------------------------------
    // for parsing

    pub fn add_symbols_from_file(&mut self, filename: &str, filetype: FileType, verbose: bool) {
        let new_symbols_result = match filetype {
            FileType::JSON => parse_symbols_json(filename),
            FileType::NATIVE => {
                parse_symbols_native(filename, verbose) // don't like this :/ (this is a smiley face)
            }
        };
        match new_symbols_result {
            Err(error) => {
                println!(
                    "couldn't parse symbols from json file: {}",
                    &error.to_string()
                );
            }
            Ok(new_symbols) => {
                for (key, _) in &new_symbols {
                    self.add_symbol(&new_symbols, key);
                }
            }
        }
    }

    pub fn add_tbis_from_file(&mut self, filename: &str, filetype: FileType, verbose: bool) {
        if self.symbols.len() != 0 {
            let tb_result = match filetype {
                FileType::JSON => parse_tbox_json(filename, &self.symbols, verbose),
                FileType::NATIVE => parse_tbox_native(filename, &self.symbols, verbose),
            };
            match tb_result {
                Err(error) => {
                    println!("couldn't parse tbox from file: {}", &error);
                }
                Ok(tb) => {
                    for tbi in tb.items() {
                        self.add_tbi(tbi);
                    }
                }
            }
        } else {
            println!("warning: no symbols detected, no tbox item will be added");
        }
    }

    pub fn new_abox_from_aboxq(&mut self, ab: ABQ_SHIQ) {
        self.current_abox = Some(ab);
    }

    pub fn new_abox_from_file_quantum(
        &mut self,
        filename: &str,
        filetype: FileType,
        verbose: bool,
    ) {
        if self.symbols.len() != 0 {
            match filetype {
                FileType::JSON => {
                    if verbose {
                        println!("the json parser is not yet implemented");
                    }

                    panic!("not implemented yet!")
                }
                FileType::NATIVE => {
                    let ab_result = parse_abox_native_quantum(filename, &mut self.symbols, verbose);

                    match ab_result {
                        Err(_error) => {
                            if verbose {
                                println!("couldn't parse abox from file: {}", filename);
                            }
                        }
                        Ok(ab) => {
                            self.current_abox = Some(ab);
                        }
                    }
                }
            }
        } else {
            println!("warning: no symbols detected, no abox item will be added");
        }
    }

    pub fn add_abis_from_file_quantum(
        &mut self,
        filename: &str,
        filetype: FileType,
        verbose: bool,
    ) {
        if self.symbols.len() != 0 {
            match filetype {
                FileType::JSON => {
                    if verbose {
                        println!("the json parser is not yet implemented");
                    }

                    panic!("not implemented yet!")
                }
                FileType::NATIVE => {
                    let ab_result = parse_abox_native_quantum(filename, &mut self.symbols, verbose);

                    match ab_result {
                        Err(_error) => {
                            if verbose {
                                println!("couldn't parse abox from file: {}", filename);
                            }
                        }
                        Ok(ab) => {
                            let c_ab = self.current_abox.as_mut().unwrap();

                            for item in ab.items() {
                                c_ab.add(item.clone());
                            }
                            // TODO: come to this line if the abox is not updated
                            //self.current_abox = Some(c_ab);
                        }
                    }
                }
            }
        } else {
            println!("warning: no symbols detected, no abox item will be added");
        }
    }

    pub fn add_abi(&mut self, abi: &ABIQ_SHIQ) {
        // you must have created a new abox
        if self.current_abox.is_some() {
            let abox = self.current_abox.as_mut().unwrap();
            abox.add(abi.clone());
        }
    }

    pub fn add_abis_from_abox(&mut self, ab: &ABQ_SHIQ) {
        for abi in ab.items() {
            self.add_abi(abi)
        }
    }

    pub fn add_tbis_from_tbox(&mut self, tb: &TB_SHIQ) {
        for tbi in tb.items() {
            self.add_tbi(tbi);
        }
    }

    // ------------------------------------------------------------------------
    // reasoner tasks

    // here I define a very important method, it will find conflicts in a abox with
    // respect to a tbox and store them in a matrix

    pub fn complete_tbox(&self, deduction_tree: bool, verbose: bool) -> TB_SHIQ {
        let tb = self.tbox.complete(deduction_tree, verbose);

        tb
    }

    pub fn auto_complete(&mut self, deduction_tree: bool, verbose: bool) {
        // the tbox
        let tb = self.complete_tbox(deduction_tree, verbose);

        self.add_tbis_from_tbox(&tb);

        self.tbox.remove_trivial();

        // the name of the abox changes
        let ab_op = self.complete_abox(verbose);
        self.current_abox = ab_op;
    }

    pub fn complete_abox(&self, verbose: bool) -> Option<ABQ_SHIQ> {
        match &self.current_abox {
            Some(ab) => Some(ab.complete(self.tbox(), verbose)),
            _ => Option::None,
        }
    }

    // please note that this matrix detect also implications
    pub fn conflict_matrix(
        &self,
        abq: &ABQ_SHIQ,
        verbose: bool,
    ) -> (Vec<i8>, Vec<(usize, Option<usize>)>, Vec<(usize, usize)>) {
        /*
        so the idea here is to first detect self conflicting nodes and not include them in
        the afore computation, the second vector helps to keep track of wich abi is mapped to
        which abi

        WARNING: if the order of elements is changed the matrix is worthless
         */
        let abq_length = abq.len();
        let matrix: Vec<i8> = Vec::new();
        let mut real_to_virtual: Vec<(usize, Option<usize>)> = Vec::new();
        let mut virtual_to_real: Vec<(usize, usize)> = Vec::new();

        if abq_length == 0 {
            if verbose {
                println!(" -- Ontology::conflict_matrix: abox is empty, nothing to analyse");
            }

            (matrix, real_to_virtual, virtual_to_real)
        } else {
            // first find every self conflicting node
            let mut self_conflicting: Vec<usize> = Vec::new();

            for index in 0..abq_length {
                let tmp_abq = abq.sub_abox(vec![index], Option::None).unwrap();

                if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: analysing if {:?} is self conflicting",
                        abq.get(index)
                    );
                }

                let tmp_abq = tmp_abq.complete(self.tbox(), verbose);

                if tmp_abq.is_inconsistent(self.tbox(), verbose) {
                    if verbose {
                        println!(
                            " -- Ontology::conflict_matrix: {:?} found to be self conflicting",
                            abq.get(index)
                        );
                    }

                    self_conflicting.push(index);
                } else {
                    if verbose {
                        println!(
                            " -- Ontology::conflict_matrix: {:?} found to be NOT self conflicting",
                            abq.get(index)
                        );
                    }
                }
            }
            // now we know which are the self conflicting elements

            // before the matrix create a vector pointing to the good values
            let mut current_length: usize = 0;
            for index in 0..abq_length {
                if !self_conflicting.contains(&index) {
                    real_to_virtual.push((index, Some(current_length)));
                    virtual_to_real.push((current_length, index));
                    current_length += 1;
                } else {
                    real_to_virtual.push((index, Option::None));
                }
            }
            // now we have two mappers:
            // real_to_virtual: given a real index of abq it will output the index of the item in the virtual list
            // virtual_to_real: given a virtual index it outputs the real index

            // now the matrix
            let virtual_length = virtual_to_real.len();
            let mut matrix: Vec<i8> = vec![0; virtual_length * virtual_length];

            // conflicts are always binary at this point
            /*
            we capture two different things:
            a => b with [a, -b] is inconsistent
            a => -b with [a,b] is inconsistent

            normally we can't have both, we weed out the self conflicting nodes...
             */
            for i in 0..virtual_length {
                let (_virtual_index_i, real_index_i) = virtual_to_real.get(i).unwrap();

                let abiq_i = abq.get(*real_index_i).unwrap();

                if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: filling column {} for item {}",
                        i, abiq_i
                    );
                }

                for j in 0..virtual_length {
                    if i != j {
                        let (_virtual_index_j, real_index_j) = virtual_to_real.get(j).unwrap();

                        let abiq_j = abq.get(*real_index_j).unwrap();
                        let abiq_j_neg = abiq_j.negate();

                        if verbose {
                            println!(" -- Ontology::conflict_matrix: comparing against {} and its negation {}", abiq_j, &abiq_j_neg);
                        }

                        let abq_tmp = ABQ_SHIQ::from_vec("tmp", vec![abiq_i.clone(), abiq_j.clone()]);
                        let abq_tmp_neg =
                            ABQ_SHIQ::from_vec("tmp_neg", vec![abiq_i.clone(), abiq_j_neg]);

                        println!(
                            "    --  abq_tmp: {}\n    --  abq_tmp_neg: {}",
                            self.abox_to_string_quantum(&abq_tmp),
                            self.abox_to_string_quantum(&abq_tmp_neg)
                        );

                        let abq_tmp = abq_tmp.complete(self.tbox(), verbose);
                        let abq_tmp_neg = abq_tmp_neg.complete(self.tbox(), verbose);

                        println!(
                            "    -- abq_tmp: {}\n    --  abq_tmp_neg: {}",
                            self.abox_to_string_quantum(&abq_tmp),
                            self.abox_to_string_quantum(&abq_tmp_neg)
                        );

                        let i_implies_not_j = abq_tmp.is_inconsistent(self.tbox(), verbose);
                        let i_implies_j = abq_tmp_neg.is_inconsistent(self.tbox(), verbose);

                        if verbose {
                            println!(" -- Ontology::conflict_matrix: found that {} with index {} implies {} with index {} to be {}", abiq_i, i, abiq_j, j, i_implies_j);
                            println!(" -- Ontology::conflict_matrix: found that {} with index {} implies the negation of {} with index {} to be {}", abiq_i, i, abiq_j, j, i_implies_not_j);
                        }

                        if i_implies_not_j {
                            if verbose {
                                println!(" -- Ontology::conflict_matrix: setting position ({}, {}) to -1", j, i);
                            }

                            matrix[virtual_length * j + i] = -1;
                        }

                        if i_implies_j {
                            if verbose {
                                println!(
                                    " -- Ontology::conflict_matrix: setting position ({}, {}) to 1",
                                    j, i
                                );
                            }

                            matrix[virtual_length * j + 1] = 1;
                        }
                    }
                }
            }

            (matrix, real_to_virtual, virtual_to_real)
        }
    }

    pub fn contains_contradiction(&self) -> bool {
        let mut contains_contradiction = false;

        for tbi in self.tbox.items() {
            contains_contradiction = contains_contradiction || tbi.is_contradiction();
        }

        contains_contradiction
    }

    /*
    pub fn find_consequences(&self, abox: &AB) -> AB {
        AB::new(abox.name())
    }

     */

    pub fn find_consequences_from_file(
        &self,
        filename: &str,
        _filetype: FileType,
    ) -> io::Result<ABQ_SHIQ> {
        let ab_name = parse_name_from_filename(filename);
        Ok(ABQ_SHIQ::new(ab_name))
    }

    // -------------------------------------------------------------------------------------------
    // get methods

    pub fn symbols(&self) -> &SymbolDict {
        &self.symbols
    }

    pub fn tbox(&self) -> &TB_SHIQ {
        &self.tbox
    }

    // this function returns two different sizes: symbol size and tbox size
    pub fn len(&self) -> (usize, usize) {
        (self.symbols.len(), self.tbox.len())
    }

    // ------------------------------------------------------------------------
    // private functions for the inner work of 'Ontology'

    fn add_symbol(
        &mut self,
        new_symbols: &SymbolDict,
        new_name: &String,
    ) -> bool {
        if new_symbols.contains_key(new_name) {
            if !self.symbols.contains_key(new_name) {
                let (_, t) = new_symbols[new_name];

                // we need to update the id to avoid conflict with the current numbers
                let (_low, high) =
                    Ontology_SHIQ::find_lower_and_highest_value_from_symbols(self.symbols());

                self.symbols.insert(new_name.clone(), (high + 1, t));
                // self.number_of_symbols += 1;

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn add_tbi(&mut self, tbi: &TBI_SHIQ) -> bool {
        if self.tbox.contains(&tbi) {
            false
        } else {
            self.tbox.add(tbi.clone());
            // self.number_of_tbi += 1;

            true
        }
    }

    fn find_lower_and_highest_value_from_symbols(
        symbols: &SymbolDict,
    ) -> (usize, usize) {
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
                }
            };

            highest = match highest {
                Option::None => Some(*b),
                Some(old_b) => {
                    if *b > old_b {
                        Some(*b)
                    } else {
                        highest
                    }
                }
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

    fn node_to_string(&self, node: &Node_SHIQ) -> String {
        let left_current = String::new();
        let right_current = String::new();

        self.node_to_string_helper(node, left_current, right_current)
    }

    fn node_to_string_helper(
        &self,
        node: &Node_SHIQ,
        mut left_current: String,
        mut right_current: String,
    ) -> String {
        match node {
            Node_SHIQ::T => String::from("Top"),    // format!("{}", node),
            Node_SHIQ::B => String::from("Bottom"), // format!("{}", node),
            Node_SHIQ::N(n) | Node_SHIQ::R(n) | Node_SHIQ::C(n) => {
                // find the name
                let mut name_found = false;
                let mut name: String = String::new();

                for symbol in &self.symbols {
                    let (a, (b, _c)) = symbol;

                    if b == n {
                        name_found = true;
                        name = a.clone();
                    }
                }

                if !name_found {
                    name = String::from("<NAME NOT FOUND>");
                }

                match node {
                    Node_SHIQ::N(_) => format!("{}{}{}", left_current, name, right_current),
                    Node_SHIQ::R(_) => format!("{}{}{}", left_current, name, right_current),
                    Node_SHIQ::C(_) => format!("{}{}{}", left_current, name, right_current),
                    _ => String::from("you shouldn't be here"),
                }
            }
            Node_SHIQ::X(m, bn) => {
                let left_addition = match m {
                    Mod::N => "-",
                    Mod::I => "(",
                    Mod::E => "E.(",
                };

                let right_addition = match m {
                    Mod::N => "",
                    Mod::I => "^-)",
                    Mod::E => ")",
                };

                left_current = format!("{}{}", left_current, left_addition);
                right_current = format!("{}{}", right_addition, right_current);

                self.node_to_string_helper(bn, left_current, right_current)
            }
        }
    }

    // this is helper function, but because is particular to symbols here defined I won't
    // move somewhere else
    fn symbols_to_string(symbols: &SymbolDict) -> String {
        let mut s = String::from("    {\n");

        for symbol in symbols {
            let (key, (integer, dltype)) = symbol;
            let symbol_formatted = format!("     : ({} -> ({}, {})),\n", key, integer, dltype);

            s.push_str(symbol_formatted.as_str());
        }

        s.push_str("    }");
        s
    }

    pub fn tbox_to_string(&self, tb: &TB_SHIQ, dont_write_trivial: bool) -> String {
        let mut s = String::from("    {\n");

        for tbi in tb.items() {
            if !tbi.is_trivial() || !dont_write_trivial {
                let tbi_string = self.tbi_to_string(tbi);
                let tbi_formatted = format!("     : {}\n", tbi_string);

                s.push_str(tbi_formatted.as_str());
            }
        }

        s.push_str("    }");
        s
    }

    pub fn abox_to_string_quantum(&self, ab: &ABQ_SHIQ) -> String {
        let mut s = String::from("    {\n");

        for abi in ab.items() {
            let abi_string = self.abiq_to_string(abi);

            // println!("{} gave {}", abi, &abi_string);

            let abi_formatted = format!("     : {}\n", abi_string);

            s.push_str(abi_formatted.as_str());
        }

        s.push_str("    }");
        s
    }

    // I suppose that the tbi is in the self.tbox
    fn tbi_to_string(&self, tbi: &TBI_SHIQ) -> String {
        let lside = self.node_to_string(tbi.lside());
        let rside = self.node_to_string(tbi.rside());

        let s = format!("{} (<) {}", lside, rside);

        s
    }

    fn abiq_to_string(&self, abiq: &ABIQ_SHIQ) -> String {
        let abi_to_string = self.abi_to_string(abiq.abi());

        let v = match abiq.value() {
            Option::None => "NC".to_string(),
            Some(n) => format!("{}", n),
        };

        let res = format!("{} (pv: {}, v: {})", abi_to_string, abiq.prevalue(), v);
        res
    }

    fn abi_to_string(&self, abi: &ABI_SHIQ) -> String {
        let s = match abi {
            ABI_SHIQ::RA(r, a, b) => {
                let r = self.node_to_string(r);
                let a = self.node_to_string(a);
                let b = self.node_to_string(b);

                let s = format!("{},{} : {}", a, b, r);
                s
            }
            ABI_SHIQ::CA(c, a) => {
                let c = self.node_to_string(c);
                let a = self.node_to_string(a);

                let s = format!("{} : {}", a, c);
                s
            }
        };

        s
    }

    pub fn tbox_to_file(
        &self,
        filename: &str,
        filetype: FileType,
        dont_write_trivial: bool,
    ) -> bool {
        match filetype {
            FileType::JSON => {
                let tbox_as_value = tbox_to_value(&self.tbox, &self.symbols, dont_write_trivial);

                match tbox_as_value {
                    Some(tbox) => {
                        let json_parsed = json!({ "tbox": tbox });

                        let file_res = File::create(filename);

                        match file_res {
                            Result::Err(e) => {
                                println!("something went wrong: {}", e);
                                false
                            }
                            Result::Ok(mut file) => {
                                let result = file.write(json_parsed.to_string().as_bytes());

                                match result {
                                    Result::Err(e) => {
                                        println!(
                                            "something went wrong while writting to the file: {}",
                                            e
                                        );
                                        false
                                    }
                                    Result::Ok(_) => true,
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
            FileType::NATIVE => {
                let tbox_as_string_op =
                    tbox_to_native_string(&self.tbox, &self.symbols, dont_write_trivial);

                match tbox_as_string_op {
                    Some(tbox_as_string) => {
                        let file_res = File::create(filename);

                        match file_res {
                            Result::Err(e) => {
                                println!("something went wrong: {}", e);
                                false
                            }
                            Result::Ok(mut file) => {
                                let result = file.write(tbox_as_string.as_bytes());

                                match result {
                                    Result::Err(e) => {
                                        println!(
                                            "something went wrong while writing to the file: {}",
                                            e
                                        );
                                        false
                                    }
                                    Result::Ok(_) => true,
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
        }
    }

    pub fn abox_to_file(
        &self,
        filename: &str,
        filetype: FileType,
        dont_write_trivial: bool,
    ) -> bool {
        match filetype {
            FileType::NATIVE => {
                let abox_as_string_op = abox_to_native_string_quantum(
                    &self.current_abox.as_ref().unwrap(),
                    &self.symbols,
                    dont_write_trivial,
                );

                match abox_as_string_op {
                    Some(abox_as_string) => {
                        let file_res = File::create(filename);

                        match file_res {
                            Result::Err(e) => {
                                println!("something went wrong: {}", e);
                                false
                            }
                            Result::Ok(mut file) => {
                                let result = file.write(abox_as_string.as_bytes());

                                match result {
                                    Result::Err(e) => {
                                        println!(
                                            "something went wrong while writing to the file: {}",
                                            e
                                        );
                                        false
                                    }
                                    Result::Ok(_) => true,
                                }
                            }
                        }
                    }
                    _ => false,
                }
            }
            _ => {
                println!("not implemented!");
                false
            }
        }
    }

    // ------------------------------------------------------------------------------------------
    // this part is the sqlite interface
    // ------------------------------------------------------------------------------------------
    pub fn populate_db(&mut self, conn: &Connection, verbose: bool) -> bool {
        // sort your tbis before
        self.sort();

        add_basic_tables_to_db(&conn, verbose);
        add_symbols_to_db(&self.symbols, &conn, verbose);
        add_tbis_to_db(self.symbols(), self.tbox.items(), &conn, verbose);
        true
    }

    pub fn initiate_from_db(filename: &str, verbose: bool) -> Result<Ontology_SHIQ> {
        let conn_res = Connection::open(filename);

        match conn_res {
            Err(e) => {
                if verbose {
                    println!("an error occurred: {}", &e);
                }
                Err(e)
            }
            Ok(conn) => {
                let tb_name = parse_name_from_filename(filename);

                let mut onto = Ontology_SHIQ::new(String::from(tb_name));

                let _symbol_res = add_symbols_from_db(&mut onto.symbols, &conn, verbose);
                let _tbis_res = add_tbis_from_db(&onto.symbols, &mut onto.tbox, &conn, verbose);

                Ok(onto)
            }
        }
    }
}
