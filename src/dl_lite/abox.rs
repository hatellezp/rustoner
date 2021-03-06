/*
UMONS 2021
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

use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::helpers_and_utilities::{
    complete_helper_add_if_necessary_general, complete_helper_dump_from_mutex_temporal_to_current,
};

use crate::dl_lite::node::ItemDllite;
use crate::dl_lite::rule::{dl_lite_abox_rule_one, dl_lite_abox_rule_three, dl_lite_abox_rule_two};
use crate::dl_lite::string_formatter::abiq_in_vec_of_vec;
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;
use crate::kb::knowledge_base::{ABox, ABoxItem, AbRule, Item, LeveledItem, TBox, TBoxItem};
use crate::kb::types::{DLType, CR};

use crate::dl_lite::utilities::get_max_level_abstract;

/// An ABox is basically an array of ABox items.
/// It has a name, a vector containing the items,
/// a witness of completion and a length.
#[derive(PartialEq, Debug, Clone)]
pub struct AbqDllite {
    name: String,
    items: Vec<AbiqDllite>,
    completed: bool,
    length: usize,
}

/// Implementation of the ABox trait for the materialization of
/// ABox in the dl_lite_r setting.
impl ABox for AbqDllite {
    type AbiItem = AbiqDllite;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    /// Will try to add the item abi to self.
    /// If successful it returns true, false otherwise.
    fn add(&mut self, abi: AbiqDllite) -> bool {
        /*
        returns true if the item was successfully inserted, false otherwise
         */
        if !self.items.contains(&abi) {
            self.items.push(abi);
            self.length += 1;
            self.completed = false;
            true
        } else {
            false
        }
    }

    /// Returns a non mutable reference to the ABox items in self.
    fn items(&self) -> &Vec<AbiqDllite> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut Vec<Self::AbiItem> {
        &mut self.items
    }

    fn get(&self, index: usize) -> Option<&AbiqDllite> {
        self.items.get(index)
    }

    fn sort(&mut self) {
        self.items.sort()
    }

    fn is_empty(&self) -> bool {
        self.items.len() == 0
    }

    fn contains(&self, abi: &AbiqDllite) -> bool {
        self.items.contains(abi)
    }
}

impl fmt::Display for AbqDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 {
            write!(f, "<ABQ>[]")
        } else {
            let mut s: String = format!("<ABQ({})>[", self.name);

            for item in &self.items {
                s.push_str(item.to_string().as_str());
                s.push_str(", ");
            }

            s.push(']');

            write!(f, "{}", s)
        }
    }
}

impl AbqDllite {
    /// An ABox is always created by a name, we can only add
    /// items after.
    pub fn new(name: &str) -> AbqDllite {
        AbqDllite {
            name: name.to_string(),
            items: vec![],
            length: 0,
            completed: false,
        }
    }

    /// Gets a mutable reference to an ABox item in self, at index 'index'.
    /// Wrapped in an Option, it will return None if nothing is found and index
    /// 'index'.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut AbiqDllite> {
        self.items.get_mut(index)
    }

    /// Build an ABox from a mutable vector of ABox items and an string to
    /// name the ABox.
    pub fn from_vec(name: &str, mut v: Vec<AbiqDllite>) -> AbqDllite {
        let mut abq = AbqDllite::new(name);

        while !v.is_empty() {
            let abiq = v.pop().unwrap();

            abq.add(abiq);
        }

        abq
    }

    /// Will create an ABox from self and a vector of indexes.
    /// Literally copies each ABox item at an index present in  self.
    /// A name is necessary to construct the new ABox.
    /// Wrapped as always in an Option in case it fails.
    pub fn sub_abox(&self, index: Vec<usize>, name: Option<&str>) -> Option<AbqDllite> {
        // create an abox from a vec of index, will help when finding conflicts in a database
        let name = name.unwrap_or("tmp");

        let mut sub_abox = AbqDllite::new(name);

        for i in index {
            if i < self.length {
                sub_abox.add(self.items[i].clone());
            }
        }

        match sub_abox.len() {
            0 => Option::None,
            _ => Some(sub_abox),
        }
    }

    /// Checks if self is inconsistent with respect to the
    /// TBox tb provided. Once an ABox is completed checking for
    /// inconsistency is comparing each ABox assertion (or couple of
    /// ABox assertions) with a TBox item and checking for a logical
    /// error.
    /// e.g. 'a Human IS NOT a Dog' and 'max IS a Human' and 'max IS a Dog'.
    pub fn is_inconsistent_detailed(
        &self,
        tb: &TBDllite,
        verbose: bool,
    ) -> Vec<(TbiDllite, Vec<AbiqDllite>)> {
        let tbis = tb.items();

        // we can have a:A and A < (-A)
        // also a:A a:B and A < (-B) the first case is an special case, so we test
        // for the second only

        // store contradictions here
        let mut contradictions: Vec<(TbiDllite, Vec<AbiqDllite>)> = Vec::new();

        // these two arrays keep track of which combinations have been tested
        // to avoid adding or to test known values
        let mut combs_added_one: Vec<&AbiqDllite> = Vec::new();
        let mut combs_added_two: Vec<(&AbiqDllite, &AbiqDllite)> = Vec::new();

        let self_length = self.length;

        for tbi in tbis {
            // pick a tbox item
            if verbose {
                println!(" -- ABQ::is_inconsistent: comparing against {}", &tbi);
            }

            // get left, right and negated right side
            let left = tbi.lside();
            let right = tbi.rside();
            let right_negated = right.clone().negate(); // gets the right side negation

            for i in 0..self_length {
                // test against each member of self
                let abq_i = self.items.get(i).unwrap();
                let node_i = abq_i.abi().symbol();

                if verbose {
                    println!(
                        " -- ABQ::is_inconsistent: analysing {} with index {}",
                        abq_i, i
                    );
                }

                if node_i == left {
                    // found a match for the left side, now we continue to test
                    // for the right side
                    if verbose {
                        println!(" -- ABQ::is_inconsistent: found match for left side, continuing analysis");
                    }

                    for j in 0..self_length {
                        let abq_j = self.items.get(j).unwrap();
                        let node_j = abq_j.abi().symbol();

                        if verbose {
                            println!(
                                " -- ABQ::is_inconsistent: analysing against {} with index {}",
                                abq_j, j
                            );
                        }

                        // we need same nominal on both
                        if abq_i.same_nominal(abq_j) && node_j == (&right_negated) {
                            if verbose {
                                println!(
                                    " -- ABQ::is_inconsistent: found conflict, adding ton conflicts"
                                );
                            }

                            let mut to_add: Vec<AbiqDllite> = Vec::new();

                            // verify that elements has not yet been added
                            let only_one = abq_i == abq_j; // checks if abq_i is self
                                                           // conflicting
                            let do_we_add: bool;

                            if only_one {
                                do_we_add = !combs_added_one.contains(&abq_i);
                            } else {
                                do_we_add = !combs_added_two.contains(&(abq_i, abq_j))
                                    && !combs_added_two.contains(&(abq_j, abq_i));
                            }

                            if do_we_add {
                                to_add.push(abq_i.clone());
                                if !only_one {
                                    to_add.push(abq_j.clone());
                                }

                                // add to contradictions
                                contradictions.push((tbi.clone(), to_add));

                                // add to tracker
                                if only_one {
                                    combs_added_one.push(abq_i);
                                } else {
                                    combs_added_two.push((abq_i, abq_j));
                                }
                            }
                        }
                    }
                }
            }
        }

        contradictions
    }

    /// This version of the function check for inconsistency without giving
    /// a detailed rapport. Returns true if self is inconsistent with respect
    /// to the TBox provided tb.
    pub fn is_inconsistent(&self, tb: &TBDllite, _verbose: bool) -> bool {
        let take_trivial = false;
        let tbis = tb.negative_inclusions(take_trivial);

        // we can have a:A and A < (-A)
        // also a:A a:B and A < (-B) the first case is an special case, so we test for the second
        // only

        let self_lenght = self.len();
        let mut abiq_i: &AbiqDllite;
        let mut abiq_j: &AbiqDllite;
        let mut is_match: bool;
        let mut lside: &ItemDllite;
        let mut rside: &ItemDllite;

        for tbi in tbis {
            // this are only negative inclusions
            lside = tbi.lside();
            rside = tbi.rside();

            for i in 0..self_lenght {
                abiq_i = self.items.get(i).unwrap();

                // also return true whenever the item is bottom !!
                // this is the case a: Bottom
                if abiq_i.abi().symbol().t() == DLType::Bottom {
                    return true;
                }

                // this is the case a:A and A<(-A)
                if abiq_i.item() == lside && abiq_i.item().is_negation(rside) {
                    return true;
                }

                // you can begin at i
                let index_for_j = if i < (self_lenght - 1) { i + 1 } else { i };
                for j in index_for_j..self_lenght {
                    abiq_j = self.items.get(j).unwrap();

                    if abiq_i.same_nominal(abiq_j) {
                        // here is the first case
                        // a:A a:B and A < - B

                        /*
                           i :- a:X_i
                           j :- a:X_j
                           tbi:- A < (-B)
                           several cases:
                               X_i = A and X_j = B
                               X_j = A and X_i = B
                        */

                        is_match = (abiq_i.item() == lside) && abiq_j.item().is_negation(rside)
                            || (abiq_j.item() == lside) && abiq_i.item().is_negation(rside);

                        // there a second test:
                        if i != j && abiq_j.item().is_negation(abiq_i.item()) {
                            return true;
                        }

                        if is_match {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    // remove duplicates in self items
    fn clean_duplicates(&mut self) {
        if !self.items.is_empty() {
            self.items.sort();

            let first = self.items.pop().unwrap();
            let mut new_items: Vec<AbiqDllite> = vec![first];
            let mut current_tail = new_items.last().unwrap();

            while !self.items.is_empty() {
                let item = self.items.pop().unwrap();

                if item.abi() != current_tail.abi() {
                    new_items.push(item);
                    current_tail = new_items.last().unwrap();
                }
            }

            new_items.sort();
            self.items = new_items;
            self.length = (&self.items).len();
        }
    }

    pub fn complete(&self, tbox: &TBDllite, deduction_tree: bool, verbose: bool) -> AbqDllite {
        type T = TbiDllite;
        type A = AbiqDllite;

        if self.items.is_empty() {
            if verbose {
                println!(" -- ABQ::complete: the abox is empty, nothing to complete");
            }

            let new_name = format!("{}_completion_not", &self.name);
            AbqDllite::new(&new_name)
        } else {
            /*
            the strategy is as follows, for each Vec or VecDeque keeps two, one that change during the
            loop, one that is updated at the end of each loop
             */

            // keep the items
            let items: Arc<Mutex<VecDeque<AbiqDllite>>> = Arc::new(Mutex::new(VecDeque::new()));
            let items_temporal: Arc<Mutex<VecDeque<AbiqDllite>>> =
                Arc::new(Mutex::new(VecDeque::new()));

            // keep the index to be treated
            let to_treat: Arc<Mutex<VecDeque<usize>>> = Arc::new(Mutex::new(VecDeque::new()));

            // keep the index that have already been treated
            let already_treated: Arc<Mutex<VecDeque<usize>>> =
                Arc::new(Mutex::new(VecDeque::new()));
            let already_treated_temporal: Arc<Mutex<VecDeque<usize>>> =
                Arc::new(Mutex::new(VecDeque::new()));

            // indicators for the while main loop
            let mut stop_condition: bool; // stop condition for the loop, see if to_treat is 'empty' or not
            let mut current_index: usize; // at each iteration keeps the index to be treated
            let mut current_item: AbiqDllite; // at each iteration keeps the item to be treated
            let mut is_already_treated: bool;
            let mut iterations: usize;

            // index trackers
            let mut length: usize; // updates whenever a phase is finished
            let mut length_temporal: usize; // the length of the 'items_temporal' queue

            /*
            I WILL PUT THE RULES HERE, WE CAN ADD OTHERS IF NEEDED
            */
            let rule_one: AbRule<T, A> = dl_lite_abox_rule_one;
            let rule_two: AbRule<T, A> = dl_lite_abox_rule_two;
            let rule_three: AbRule<T, A> = dl_lite_abox_rule_three;

            let number_of_rules: usize = 3;
            let rules: [&AbRule<T, A>; 3] = [&rule_one, &rule_two, &rule_three];

            let rule_ordinal = [CR::First, CR::Second, CR::Third];

            /*
            RULES DECLARATION END HERE
             */

            // begin by putting every element in the items queue
            {
                // initialize the length
                length = 0;

                // lock 'items' for you
                let mut items = items.lock().unwrap();

                for item in &self.items {
                    items.insert(length, item.clone());

                    // update the length
                    length += 1;
                }
            }

            // now update 'to_treat'
            {
                let mut to_treat = to_treat.lock().unwrap();
                for index in 0..length {
                    to_treat.push_front(index);
                }
            }

            // here is the main loop
            stop_condition = true;
            iterations = 0;
            while stop_condition {
                // message for the status
                {
                    if verbose {
                        let items = items.lock().unwrap();
                        let to_treat = to_treat.lock().unwrap();
                        let already_treated = already_treated.lock().unwrap();

                        println!(
                            "    ==================================================================="
                        );
                        println!(
                            "    --------this is the status at beginning of iteration {}------------",
                            iterations
                        );
                        println!("    -- items: {:?}", &items);
                        println!("    -- to_treat: {:?}", &to_treat);
                        println!("    -- already_treated: {:?}", &already_treated);
                        println!(
                            "    -------------------------------------------------------------------"
                        );
                    }
                }

                // at the beginning some conditions have to be reinitialized
                stop_condition = false;
                is_already_treated = false;
                length_temporal = 0;
                {
                    // clear the temporal queues
                    let mut items_temporal = items_temporal.lock().unwrap();
                    let mut already_treated_temporal = already_treated_temporal.lock().unwrap();

                    items_temporal.clear();
                    already_treated_temporal.clear();
                }

                /*
                first prepare the terrain, get an index, verify that the index as not been treated,
                if it is the case the pass this iteration, otherwise pick the item with the provided
                index and enter the for loop
                 */

                // get index from 'to_treat' and put it in current_index
                {
                    let mut to_treat = to_treat.lock().unwrap();
                    current_index = to_treat.pop_back().unwrap();
                }

                // here the test for 'already_treated'
                {
                    let already_treated = already_treated.lock().unwrap();
                    is_already_treated = already_treated.contains(&current_index)
                }

                // you got the value, now test
                if is_already_treated {
                    // if the item was already treated, pass to the next
                    continue;
                } else {
                    // otherwise continue with the loop, but the add the index to the already treated ones
                    let mut already_treated_temporal = already_treated_temporal.lock().unwrap();
                    already_treated_temporal.push_front(current_index);

                    // also udpate the current_item
                    let items = items.lock().unwrap();
                    current_item = items[current_index].clone();

                    // now current_item has the necessary item inside
                    if verbose {
                        println!(
                            " -- ABQ::complete: treating now {} (index {})",
                            &current_item, current_index
                        );
                    }
                }

                /*
                here is the main inner for loop
                compare with each item in 'items', see if you can apply some rule, if it is the case
                call 'tbox_complete_add_if_necessary_two' ('two' because all rules here use two items)
                 */
                {
                    // access the items
                    let items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    // current_length has to have the exact value
                    for index in 0..length {
                        let item = &items[index];

                        for rule_index in 0..number_of_rules {
                            let rule: &AbRule<T, A> = rules[rule_index];
                            let rule_ord = rule_ordinal[rule_index];

                            // use each item
                            for tbi in tbox.items() {
                                // three different vectors

                                if verbose {
                                    println!(" -- ABQ::complete: comparing with tbi: {}", tbi);
                                }

                                let new_item_vec3 = AbiqDllite::apply_rule(
                                    vec![&current_item, &item],
                                    vec![tbi],
                                    rule,
                                    deduction_tree,
                                );

                                for optional_vec in &[&new_item_vec3] {
                                    // if the rule succeeded

                                    // println!("--- in abox complete, optional vec is : {:?}", &optional_vec);

                                    if optional_vec.is_some() {
                                        let mut abis_to_add: Vec<AbiqDllite> = Vec::new();
                                        let iterator = optional_vec.as_ref().unwrap();

                                        let _abi_already_exits = false;

                                        for abi_to_be_added in iterator {
                                            abis_to_add.push(abi_to_be_added.clone());
                                        }

                                        // println!("--- optional vec will attempt to be added: {:?}", abis_to_add);

                                        length_temporal = complete_helper_add_if_necessary_general(
                                            &items,
                                            &mut items_temporal,
                                            vec![&current_item, &item],
                                            &abis_to_add, // always one element
                                            length_temporal,
                                            verbose,
                                            rule_ord,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }

                /*
                here is the last phase of the loop, put everything that 'items_temporal' has
                inside of 'items' and add every new index to 'to_treat' (or 'to_treat_temporal' ?)
                 */

                {
                    let mut items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    let mut to_treat = to_treat.lock().unwrap();

                    let mut already_treated = already_treated.lock().unwrap();
                    let mut already_treated_temporal = already_treated_temporal.lock().unwrap();

                    // this is for the items
                    length = complete_helper_dump_from_mutex_temporal_to_current(
                        &mut items,
                        &mut items_temporal,
                        length,
                        length_temporal,
                        Some(&mut to_treat),
                        verbose,
                    );

                    // put this also in a function
                    // this is for the index already treated
                    while !already_treated_temporal.is_empty() {
                        let already_treated_index = already_treated_temporal.pop_back().unwrap();
                        already_treated.push_front(already_treated_index);
                    }
                }

                // last line of the while loop
                // update the stop condition
                {
                    let to_treat = to_treat.lock().unwrap();
                    stop_condition = !to_treat.is_empty();
                }

                if verbose {
                    let items = items.lock().unwrap();
                    let to_treat = to_treat.lock().unwrap();
                    let already_treated = already_treated.lock().unwrap();

                    println!(
                        "    -------------------------------------------------------------------"
                    );
                    println!(
                        "    -----------this is the status at end of iteration {}----------------",
                        iterations
                    );
                    println!("    -- items: {:?}", &items);
                    println!("    -- to_treat: {:?}", &to_treat);
                    println!("    -- already_treated: {:?}", &already_treated);
                    println!(
                        "    ==================================================================="
                    );
                }

                // update the iteration counter
                iterations += 1;
            }

            let new_name = format!("{}_completed", self.name);
            let mut new_abq = AbqDllite::new(&new_name);
            {
                let mut items = items.lock().unwrap();
                while !items.is_empty() {
                    new_abq.add(items.pop_front().unwrap());
                }
            }

            // of course, set completed to 'true' in the new tbox
            new_abq.clean_duplicates();
            new_abq.completed = true;

            new_abq.sort();
            new_abq
        }
    }

    /// Checks if an ABox assertion is self-contradicting with
    /// respect to the TBox provided tb.
    pub fn abiq_is_self_contradicting(abiq: &AbiqDllite, tb: &TBDllite) -> bool {
        // build a new ABox with one sole assertion
        let mut new_ab = AbqDllite::new("temp");
        new_ab.add(abiq.clone());

        // complete the ABox
        new_ab = new_ab.complete(tb, false, false);

        // check for inconsistency
        new_ab.is_inconsistent(tb, false)
    }

    pub fn get_max_level(&self) -> usize {
        get_max_level_abstract(self.items())
    }

    pub fn get_abis_by_level(
        &self,
        _tb: &TBDllite,
        only_conflicts: bool,
        contradictions: &[(TbiDllite, Vec<AbiqDllite>)],
    ) -> Vec<usize> {
        let max_level = self.get_max_level();
        let mut levels: Vec<usize> = vec![0; max_level + 1];
        let mut lev: usize;
        let is_self_contradicting = false;

        for abiq in self.items() {
            lev = abiq.level();
            // is_self_contradicting =  ABQ_DLlite::abiq_is_self_contradicting(abiq, tb);

            // add only contradictions
            if is_self_contradicting
                || !only_conflicts
                || (abiq_in_vec_of_vec(abiq, contradictions))
            {
                levels[lev] += 1;
            }
        }

        levels
    }
}
