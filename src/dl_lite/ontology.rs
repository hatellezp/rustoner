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

use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item::AbiDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::json_filetype_utilities::{parse_symbols_json, parse_tbox_json};
use crate::dl_lite::native_filetype_utilities::{
    abox_to_native_string_quantum, parse_abox_native_quantum, parse_symbols_native,
    parse_tbox_native,
};
use crate::dl_lite::node::{ItemDllite, Mod};
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;

use crate::kb::knowledge_base::{ABox, ABoxItem, AggrFn, SymbolDict, TBox, TBoxItem};
use crate::kb::types::DLType;
use crate::kb::types::FileType;

use crate::interface::utilities::write_str_to_file;

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;

/*
some types to avoid large names
 */
type CleanMatrixDictOpt = Option<(
    Vec<i8>,
    HashMap<usize, Option<usize>>,
    HashMap<usize, usize>,
    Option<(usize, usize)>,
)>;
type ConflictMatrixDict = (
    Vec<i8>,
    HashMap<usize, Option<usize>>,
    HashMap<usize, usize>,
);

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
pub struct OntologyDllite {
    name: String,
    symbols: SymbolDict,
    tbox: TBDllite,
    cln_positive: TBDllite,
    cln_negative: TBDllite,
    current_abox: Option<AbqDllite>,
}

impl fmt::Display for OntologyDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut formatted: String;

        // add the name
        formatted = format!("Ontology<{}>: {{\n", &self.name);
        s.push_str(formatted.as_str());

        // add the symbols
        formatted = format!(
            "--<Symbols>\n{}\n",
            OntologyDllite::symbols_to_string(&self.symbols)
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
        s.push('}');

        write!(f, "{}", s)
    }
}

impl OntologyDllite {
    //-------------------------------------------------------------------------
    // public functions for the interface

    pub fn new(s: String) -> OntologyDllite {
        let mut symbols: SymbolDict = HashMap::new();

        /*
        bottom and top are added by default
         */
        symbols.insert(String::from("Top"), (1, DLType::Top));
        symbols.insert(String::from("Bottom"), (0, DLType::Bottom));

        OntologyDllite {
            name: s,
            symbols,
            tbox: TBDllite::new(),
            cln_negative: TBDllite::new(),
            cln_positive: TBDllite::new(),
            current_abox: Option::None,
        }
    }

    pub fn tbox(&self) -> &TBDllite {
        &self.tbox
    }

    pub fn cln(&self, positive: bool) -> &TBDllite {
        match positive {
            true => &self.cln_positive,
            false => &self.cln_negative,
        }
    }

    pub fn abox(&self) -> Option<&AbqDllite> {
        match &self.current_abox {
            Option::None => Option::None,
            Some(ab) => Some(ab),
        }
    }

    // ------------------------------------------------------------------------
    // modifications of the ontology

    /*
    pub fn sort(&mut self) {
        self.tbox.sort();
    }

     */

    pub fn add_tbis_from_vec(&mut self, v: &[TbiDllite]) {
        for tbi in v {
            if !self.tbox.contains(&tbi) {
                self.tbox.add(tbi.clone());
            }
        }
    }

    /*
    pub fn add_abis_from_vec(&mut self, v: &Vec<AbiqDllite>) {
        match self.current_abox.as_mut() {
            Option::None => (),
            Some(ab) => {
                for abi in v {
                    if !ab.contains(&abi) {
                        ab.add(abi.clone());
                    }
                }
            }
        }
    }

     */

    // ----------------------------------------------------------------------------------------
    // for parsing

    pub fn add_symbols_from_file(&mut self, filename: &str, filetype: FileType, verbose: bool) {
        let new_symbols_result = match filetype {
            FileType::Json => parse_symbols_json(filename),
            FileType::Native => {
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
                for key in new_symbols.keys() {
                    self.add_symbol(&new_symbols, key);
                }
            }
        }
    }

    pub fn add_tbis_from_file(&mut self, filename: &str, filetype: FileType, verbose: bool) {
        if !self.symbols.is_empty() {
            let tb_result = match filetype {
                FileType::Json => parse_tbox_json(filename, &self.symbols, verbose),
                FileType::Native => parse_tbox_native(filename, &self.symbols, verbose),
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

    pub fn new_abox_from_file_quantum(
        &mut self,
        filename: &str,
        filetype: FileType,
        verbose: bool,
    ) {
        if !self.symbols.is_empty() {
            match filetype {
                FileType::Json => {
                    if verbose {
                        println!("the json parser is not yet implemented");
                    }

                    panic!("not implemented yet!")
                }
                FileType::Native => {
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

    /*
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

     */

    pub fn add_abi(&mut self, abi: &AbiqDllite) {
        // you must have created a new abox
        if self.current_abox.is_some() {
            let abox = self.current_abox.as_mut().unwrap();
            abox.add(abi.clone());
        }
    }

    pub fn add_abis_from_abox(&mut self, ab: &AbqDllite) {
        for abi in ab.items() {
            self.add_abi(abi)
        }
    }

    /*
    pub fn add_tbis_from_tbox(&mut self, tb: &TB_DLlite) {
        for tbi in tb.items() {
            self.add_tbi(tbi);
        }
    }

     */

    // ------------------------------------------------------------------------
    // reasoner tasks

    // here I define a very important method, it will find conflicts in a abox with
    // respect to a tbox and store them in a matrix

    /*
    pub fn complete_tbox(&self, deduction_tree: bool, verbose: bool) -> TBDllite {
        self.tbox.complete(deduction_tree, verbose)
    }

    pub fn complete_abox(&self, deduction_tree: bool, verbose: bool) -> Option<AbqDllite> {
        self.current_abox
            .as_ref()
            .map(|ab| ab.complete(self.tbox(), deduction_tree, verbose))
    }

     */

    // this generate both closures for the matrix
    // if you only need the negative closure then pass a negative number,
    // for the positive closure pass a positive number
    // and for both pass 0
    pub fn generate_cln(&mut self, deduction_tree: bool, verbose: bool, positive_or_negative: i8) {
        match positive_or_negative.cmp(&0) {
            Ordering::Less => {
                self.cln_negative = self.tbox.cln_completion(true, deduction_tree, verbose)
            }
            Ordering::Equal => {
                self.cln_negative = self.tbox.cln_completion(true, deduction_tree, verbose);
                self.cln_positive = self.tbox.cln_completion(false, deduction_tree, verbose);
            }
            Ordering::Greater => {
                self.cln_positive = self.tbox.cln_completion(false, deduction_tree, verbose)
            }
        }
    }

    // this function **suppose** that not self-conflicting assertions are present!
    // UPDATE: fix the needless hypothesis, all aboxes are accepted now
    pub fn conflict_matrix_refs_only(&self, abq: &AbqDllite, verbose: bool) -> ConflictMatrixDict {
        /*
        so the idea here is to first detect self conflicting nodes and not include them in
        the afore computation, the second vector helps to keep track of which abi is mapped to
        which abi

        WARNING: if the order of elements is changed this matrix is worthless
         */
        let abq_length = abq.len();
        let matrix: Vec<i8> = Vec::new();

        // map to both sides
        let mut real_to_virtual: HashMap<usize, Option<usize>> = HashMap::new();
        let mut virtual_to_real: HashMap<usize, usize> = HashMap::new();

        let mut already_computed: HashMap<(usize, usize), i8> = HashMap::new();

        if abq_length == 0 {
            if verbose {
                println!(" -- Ontology::conflict_matrix: abox is empty, nothing to analyse");
            }

            (matrix, real_to_virtual, virtual_to_real)
        } else {
            // first find every self conflicting node
            let mut self_conflicting: Vec<usize> = Vec::new();

            for index in 0..abq_length {
                if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: analysing if {:?} is self conflicting",
                        abq.get(index)
                    );
                }

                /*
                   when analysing if an item is self-conflicting, it comes from the original
                   abox and thus is not negated, the negative closure is the only one needed
                   to find a contradcition
                */

                // using get_unchecked force me to label the function unsafe, thus passing
                let (abox_is_inconsistent, _) = AbqDllite::is_inconsistent_refs_only(
                    vec![abq.items().get(index).unwrap()],
                    self.cln(false),
                    false,
                );
                if abox_is_inconsistent {
                    if verbose {
                        println!(
                            " -- Ontology::conflict_matrix: {:?} found to be self conflicting",
                            abq.get(index)
                        );
                    }

                    self_conflicting.push(index);
                } else if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: {:?} found to be NOT self conflicting",
                        abq.get(index)
                    );
                }
            }
            // now we know which are the self conflicting elements

            // before the matrix create a vector pointing to the good values
            let mut current_length: usize = 0;

            for index in 0..abq_length {
                if !self_conflicting.contains(&index) {
                    real_to_virtual.insert(index, Some(current_length));
                    virtual_to_real.insert(current_length, index);
                    current_length += 1;
                } else {
                    real_to_virtual.insert(index, Option::None);
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
                let real_index_i = virtual_to_real.get(&i).unwrap();

                let abiq_i = abq.get(*real_index_i).unwrap();

                if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: filling column {} for item {}",
                        i, abiq_i
                    );
                }

                for j in 0..virtual_length {
                    // no need of same element analysis
                    if i != j {
                        if already_computed.contains_key(&(i, j))
                            && already_computed.get(&(i, j)).unwrap() == &(-1)
                        {
                            matrix[virtual_length * i + j] = -1;
                            already_computed.insert((j, i), -1);

                            if verbose {
                                println!(" -- Ontology::conflict_matrix: already found negative coefficient for index: ({}, {}), passing", i, j);
                            }
                        } else {
                            let real_index_j = virtual_to_real.get(&j).unwrap();

                            let abiq_j = abq.get(*real_index_j).unwrap();

                            // first analyse if (i) implies (not j)

                            /*
                               again, for this is the negative closure that is needed
                            */

                            let mut deduction_found = false;
                            let (abox_is_inconsistent, _) = AbqDllite::is_inconsistent_refs_only(
                                vec![abiq_i, abiq_j],
                                self.cln(false),
                                false,
                            );

                            if abox_is_inconsistent {
                                if verbose {
                                    println!(" -- Ontology::conflict_matrix: setting position ({}, {}) to -1", j, i);
                                }

                                matrix[virtual_length * j + i] = -1;
                                already_computed.insert((i, j), -1);
                                deduction_found = true;
                            }

                            // if not deduction was found then
                            // check if (i) implies (j)

                            /*
                               now we use the positive closure, that have all consequences
                            */

                            if !deduction_found {
                                let abiq_j_neg = abiq_j.negate();

                                let (abox_is_inconsistent, _) =
                                    AbqDllite::is_inconsistent_refs_only(
                                        vec![abiq_i, &abiq_j_neg],
                                        self.cln(true),
                                        false,
                                    );

                                if abox_is_inconsistent {
                                    if verbose {
                                        println!(
                                            " -- Ontology::conflict_matrix: setting position ({}, {}) to 1",
                                            j, i
                                        );
                                    }

                                    matrix[virtual_length * j + i] = 1;
                                    already_computed.insert((i, j), 1);
                                }
                            }
                        }
                    }
                }
            }

            (matrix, real_to_virtual, virtual_to_real)
        }
    }

    // please note that this matrix detect also implications
    pub fn conflict_matrix(
        &self,
        abq: &AbqDllite,
        deduction_tree: bool,
        verbose: bool,
    ) -> ConflictMatrixDict {
        /*
        so the idea here is to first detect self conflicting nodes and not include them in
        the afore computation, the second vector helps to keep track of which abi is mapped to
        which abi

        WARNING: if the order of elements is changed this matrix is worthless
         */
        let abq_length = abq.len();
        let matrix: Vec<i8> = Vec::new();

        // map to both sides
        let mut real_to_virtual: HashMap<usize, Option<usize>> = HashMap::new();
        let mut virtual_to_real: HashMap<usize, usize> = HashMap::new();

        // let inner_verbose = false;
        let inner_verbose = verbose;
        let mut already_computed: HashMap<(usize, usize), i8> = HashMap::new();

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

                let tmp_abq = tmp_abq.complete(self.tbox(), deduction_tree, inner_verbose);

                if tmp_abq.is_inconsistent(self.tbox(), verbose) {
                    if verbose {
                        println!(
                            " -- Ontology::conflict_matrix: {:?} found to be self conflicting",
                            abq.get(index)
                        );
                        // println!("\n{} \n {}\n", self.tbox_to_string(self.tbox(), false), self.abox_to_string_quantum(&tmp_abq));
                    }

                    self_conflicting.push(index);
                } else if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: {:?} found to be NOT self conflicting",
                        abq.get(index)
                    );
                }
            }
            // now we know which are the self conflicting elements

            // before the matrix create a vector pointing to the good values
            let mut current_length: usize = 0;

            for index in 0..abq_length {
                if !self_conflicting.contains(&index) {
                    real_to_virtual.insert(index, Some(current_length));
                    virtual_to_real.insert(current_length, index);
                    current_length += 1;
                } else {
                    real_to_virtual.insert(index, Option::None);
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
                let real_index_i = virtual_to_real.get(&i).unwrap();

                let abiq_i = abq.get(*real_index_i).unwrap();

                if verbose {
                    println!(
                        " -- Ontology::conflict_matrix: filling column {} for item {}",
                        i, abiq_i
                    );
                }

                for j in 0..virtual_length {
                    // no need of same element analysis
                    if i != j {
                        let _ee = already_computed.get(&(i, j));
                        if already_computed.contains_key(&(i, j))
                            && already_computed.get(&(i, j)).unwrap() == &(-1)
                        {
                            matrix[virtual_length * i + j] = -1;
                            already_computed.insert((j, i), -1);

                            if verbose {
                                println!(" -- Ontology::conflict_matrix: already found negative coefficient for index: ({}, {}), passing", i, j);
                            }
                        } else {
                            let real_index_j = virtual_to_real.get(&j).unwrap();

                            let abiq_j = abq.get(*real_index_j).unwrap();
                            let abiq_j_neg = abiq_j.negate();

                            if verbose {
                                println!(" -- Ontology::conflict_matrix: comparing against {} and its negation {}", abiq_j, &abiq_j_neg);
                            }

                            let abq_tmp =
                                AbqDllite::from_vec("tmp", vec![abiq_i.clone(), abiq_j.clone()]);
                            let abq_tmp_neg =
                                AbqDllite::from_vec("tmp_neg", vec![abiq_i.clone(), abiq_j_neg]);

                            if verbose {
                                println!(
                                    "    --  abq_tmp: {}\n    --  abq_tmp_neg: {}",
                                    self.abox_to_string_quantum(&abq_tmp),
                                    self.abox_to_string_quantum(&abq_tmp_neg)
                                );
                            }

                            let abq_tmp =
                                abq_tmp.complete(self.tbox(), deduction_tree, inner_verbose);
                            let abq_tmp_neg =
                                abq_tmp_neg.complete(self.tbox(), deduction_tree, inner_verbose);

                            if verbose {
                                println!(
                                    "    -- (after completion)\n    -- abq_tmp: {}\n    --  abq_tmp_neg: {}",
                                    self.abox_to_string_quantum(&abq_tmp),
                                    self.abox_to_string_quantum(&abq_tmp_neg)
                                );
                            }

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
                                already_computed.insert((i, j), -1);
                            }

                            if i_implies_j {
                                if verbose {
                                    println!(
                                        " -- Ontology::conflict_matrix: setting position ({}, {}) to 1",
                                        j, i
                                    );
                                }

                                matrix[virtual_length * j + i] = 1;
                                already_computed.insert((i, j), 1);
                            }
                        }
                    }
                }
            }

            (matrix, real_to_virtual, virtual_to_real)
        }
    }

    pub fn clean_index_matrix(matrix: &[i8]) -> Option<(Vec<usize>, bool, bool)> {
        let mut conflict_index_hashset: HashSet<usize> = HashSet::new();
        let mut conflict_clean: Vec<usize> = Vec::new();

        let n_squared = matrix.len();
        let n = (n_squared as f64).sqrt() as usize;

        if n * n != n_squared {
            Option::None
        } else {
            for i in 0..n {
                for j in 0..n {
                    if matrix[i * n + j] != 0 {
                        conflict_index_hashset.insert(i);
                        conflict_index_hashset.insert(j);
                    }
                }
            }

            for i in 0..n {
                if !conflict_index_hashset.contains(&i) {
                    conflict_clean.push(i);
                }
            }

            let all_clean = conflict_clean.len() == n;
            let all_conflict = conflict_clean.is_empty();
            Some((conflict_clean, all_clean, all_conflict))
        }
    }

    pub fn from_conflict_to_clean_matrix(matrix: &[i8]) -> CleanMatrixDictOpt {
        let res_op = OntologyDllite::clean_index_matrix(matrix);

        let mut chosen_index: Option<(usize, usize)> = Option::None;

        match res_op {
            Option::None => Option::None,
            Some((conflict_clean, all_clean, all_conflict)) => {
                // here we create a new matrix
                // first is everything is dirty then not modification

                // this is an aliasing to dont forget
                let before_matrix = matrix;

                let nsquared = before_matrix.len();
                let n = (nsquared as f64).sqrt() as usize;

                let mut before_matrix_to_done_matrix: HashMap<usize, Option<usize>> =
                    HashMap::new();
                let mut done_matrix_to_before_matrix: HashMap<usize, usize> = HashMap::new();
                let mut done_matrix: Vec<i8> = Vec::new();

                // if everything is a conflict the copy all
                if all_conflict {
                    // done_matrix = matrix.clone();
                    done_matrix = matrix.to_owned();

                    for i in 0..n {
                        before_matrix_to_done_matrix.insert(i, Some(i));
                        done_matrix_to_before_matrix.insert(i, i);
                    }

                    Some((
                        done_matrix,
                        before_matrix_to_done_matrix,
                        done_matrix_to_before_matrix,
                        chosen_index,
                    ))
                } else if all_clean {
                    // if everything is clean then output an empty vector
                    for i in 0..n {
                        before_matrix_to_done_matrix.insert(i, Option::None);
                    }

                    Some((
                        done_matrix,
                        before_matrix_to_done_matrix,
                        done_matrix_to_before_matrix,
                        chosen_index,
                    ))
                } else {
                    // you must leave at least one clean fact
                    let one_clean_fact_index = conflict_clean[0]; // this can't be empty

                    let new_dim = n - conflict_clean.len();
                    let new_dim = new_dim + 1; // we add one for the one clean fact!!!
                    done_matrix = vec![0; new_dim * new_dim];
                    let mut current_index: usize = 0;
                    let mut changed = false;

                    for i in 0..n {
                        if !conflict_clean.contains(&i) {
                            before_matrix_to_done_matrix.insert(i, Some(current_index));
                            done_matrix_to_before_matrix.insert(current_index, i);
                            current_index += 1;
                        } else if i == one_clean_fact_index {
                            before_matrix_to_done_matrix.insert(i, Some(current_index));
                            done_matrix_to_before_matrix.insert(current_index, i);

                            // new point to old!!!
                            if !changed {
                                chosen_index = Some((current_index, i));
                                changed = true;
                            }

                            current_index += 1;
                        } else {
                            before_matrix_to_done_matrix.insert(i, Option::None);
                        }
                    }

                    let mut before_i: usize;
                    let mut before_j: usize;

                    if current_index != new_dim {
                        // something went wrong and the matrice have not the same size
                        Option::None
                    } else {
                        for i in 0..new_dim {
                            before_i = *(done_matrix_to_before_matrix.get(&i).unwrap());
                            for j in 0..new_dim {
                                before_j = *(done_matrix_to_before_matrix.get(&j).unwrap());

                                done_matrix[new_dim * i + j] =
                                    before_matrix[n * before_i + before_j];
                            }
                        }

                        Some((
                            done_matrix,
                            before_matrix_to_done_matrix,
                            done_matrix_to_before_matrix,
                            chosen_index,
                        ))
                    }
                }
            }
        }
    }

    // here we compute the A matrix
    // remember: a*1 - b*A = c*(1,...,1)
    // (Vec<i8>, HashMap<usize, Option<usize>>, HashMap<usize, usize>)
    pub fn compute_aggregation_matrix(
        abq: &AbqDllite,
        matrix: &[i8],
        virtual_to_real: &HashMap<usize, usize>,
        aggr: AggrFn,
        verbose: bool,
    ) -> Vec<f64> {
        let matrix_len = matrix.len();
        let mut v: Vec<f64> = vec![0 as f64; matrix_len];
        let mut real_index_op: Option<&usize>;
        let mut abiq_op: Option<&AbiqDllite>;
        let mut aggr_v: f64;
        let mut i: usize;
        let mut j: usize;
        let root = (matrix_len as f64).sqrt() as usize;

        for index in 0..matrix_len {
            i = index / root;
            j = index - i * root;

            if verbose {
                println!(" -- Ontology::compute_A_matrix: for lenght {} found index i: {} and index j: {} with original index: {}", matrix_len, i, j, index);
            }

            // TODO: verify that this should be j
            // real_index_op = virtual_to_real.get(&i);
            real_index_op = virtual_to_real.get(&j);

            match real_index_op {
                Option::None => {
                    if verbose {
                        println!(
                            " -- Ontology::compute_A_matrix: real index gave nothing for index: {}",
                            j
                        );
                    }
                } // simply pass
                Some(real_index) => {
                    abiq_op = abq.get(*real_index);

                    match abiq_op {
                        Option::None => {
                            if verbose {
                                println!(" -- Ontology::compute_A_matrix: index {} gave real index {} that gave nothing!", i, real_index);
                            }
                        } // pass again
                        Some(abiq) => {
                            aggr_v = aggr(vec![abiq.credibility()]);
                            v[index] = aggr_v * (matrix[index] as f64);
                        }
                    }
                }
            }
        }

        v
    }

    /*
    pub fn contains_contradiction(&self) -> bool {
        let mut contains_contradiction = false;

        for tbi in self.tbox.items() {
            contains_contradiction = contains_contradiction || tbi.is_contradiction();
        }

        contains_contradiction
    }

    pub fn find_consequences_from_file(
        &self,
        filename: &str,
        _filetype: FileType,
    ) -> io::Result<AbqDllite> {
        let ab_name = parse_name_from_filename(filename);
        Ok(AbqDllite::new(ab_name))
    }

     */

    // -------------------------------------------------------------------------------------------
    // get methods

    pub fn symbols(&self) -> &SymbolDict {
        &self.symbols
    }

    /*
    // this function returns two different sizes: symbol size and tbox size
    pub fn len(&self) -> (usize, usize) {
        (self.symbols.len(), self.tbox.len())
    }

     */

    // ------------------------------------------------------------------------
    // private functions for the inner work of 'Ontology'

    fn add_symbol(&mut self, new_symbols: &SymbolDict, new_name: &str) -> bool {
        if new_symbols.contains_key(new_name) {
            if !self.symbols.contains_key(new_name) {
                let (_, t) = new_symbols[new_name];

                // we need to update the id to avoid conflict with the current numbers
                let (_low, high) =
                    OntologyDllite::find_lower_and_highest_value_from_symbols(self.symbols());

                self.symbols.insert(new_name.to_string(), (high + 1, t));
                // self.number_of_symbols += 1;

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn add_tbi(&mut self, tbi: &TbiDllite) -> bool {
        if self.tbox.contains(&tbi) {
            false
        } else {
            self.tbox.add(tbi.clone());
            // self.number_of_tbi += 1;

            true
        }
    }

    fn find_lower_and_highest_value_from_symbols(symbols: &SymbolDict) -> (usize, usize) {
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

    fn node_to_string(&self, node: &ItemDllite) -> String {
        let left_current = String::new();
        let right_current = String::new();

        self.node_to_string_helper(node, left_current, right_current)
    }

    fn node_to_string_helper(
        &self,
        node: &ItemDllite,
        mut left_current: String,
        mut right_current: String,
    ) -> String {
        match node {
            ItemDllite::T => String::from("Top"),    // format!("{}", node),
            ItemDllite::B => String::from("Bottom"), // format!("{}", node),
            ItemDllite::N(n) | ItemDllite::R(n) | ItemDllite::C(n) => {
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
                    ItemDllite::N(_) => format!("{}{}{}", left_current, name, right_current),
                    ItemDllite::R(_) => format!("{}{}{}", left_current, name, right_current),
                    ItemDllite::C(_) => format!("{}{}{}", left_current, name, right_current),
                    _ => String::from("you shouldn't be here"),
                }
            }
            ItemDllite::X(m, bn) => {
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

    pub fn tbox_to_string(&self, tb: &TBDllite, dont_write_trivial: bool) -> String {
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

    pub fn abox_to_string_quantum(&self, ab: &AbqDllite) -> String {
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
    fn tbi_to_string(&self, tbi: &TbiDllite) -> String {
        let lside = self.node_to_string(tbi.lside());
        let rside = self.node_to_string(tbi.rside());

        let s = format!("{} (<) {}", lside, rside);

        s
    }

    fn abiq_to_string(&self, abiq: &AbiqDllite) -> String {
        let abi_to_string = self.abi_to_string(abiq.abi());

        let v = match abiq.value() {
            Option::None => "NC".to_string(),
            Some(n) => format!("{}", n),
        };

        let res = format!("{} (pv: {}, v: {})", abi_to_string, abiq.credibility(), v);
        res
    }

    fn abi_to_string(&self, abi: &AbiDllite) -> String {
        match abi {
            AbiDllite::RA(r, a, b) => {
                let r = self.node_to_string(r);
                let a = self.node_to_string(a);
                let b = self.node_to_string(b);

                let s = format!("{},{} : {}", a, b, r);
                s
            }
            AbiDllite::CA(c, a) => {
                let c = self.node_to_string(c);
                let a = self.node_to_string(a);

                let s = format!("{} : {}", a, c);
                s
            }
        }
    }

    /*
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

     */

    pub fn abox_to_file(
        &self,
        filename: &str,
        filetype: FileType,
        dont_write_trivial: bool,
    ) -> bool {
        match filetype {
            FileType::Native => {
                let abox_as_string_op = abox_to_native_string_quantum(
                    &self.current_abox.as_ref().unwrap(),
                    &self.symbols,
                    dont_write_trivial,
                );

                match abox_as_string_op {
                    Some(abox_as_string) => {
                        // this function successfully does as is told
                        write_str_to_file(&abox_as_string, filename)
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

    /*
    pub fn populate_db(&mut self, conn: &Connection, verbose: bool) -> bool {
        // sort your tbis before
        self.sort();

        add_basic_tables_to_db(&conn, verbose);
        add_symbols_to_db(&self.symbols, &conn, verbose);
        add_tbis_to_db(self.symbols(), self.tbox.items(), &conn, verbose);
        true
    }

     */

    /*
    pub fn initiate_from_db(filename: &str, verbose: bool) -> Result<Ontology_DLlite> {
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

                let mut onto = Ontology_DLlite::new(String::from(tb_name));

                let _symbol_res = add_symbols_from_db(&mut onto.symbols, &conn, verbose);
                let _tbis_res = add_tbis_from_db(&onto.symbols, &mut onto.tbox, &conn, verbose);

                Ok(onto)
            }
        }
    }

     */
}
