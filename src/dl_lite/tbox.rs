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

// external imports
use std::collections::VecDeque;
use std::fmt;
use std::sync::{Arc, Mutex};

// internal imports
use crate::dl_lite::helpers_and_utilities::{
    complete_helper_add_if_necessary_general, complete_helper_dump_from_mutex_temporal_to_current,
};
use crate::dl_lite::rule::{
    dl_lite_rule_eight, dl_lite_rule_five, dl_lite_rule_four, dl_lite_rule_one, dl_lite_rule_seven,
    dl_lite_rule_six, dl_lite_rule_three, dl_lite_rule_two, dl_lite_rule_zero,
};
use crate::dl_lite::tbox_item::TbiDllite;
use crate::dl_lite::utilities::get_max_level_abstract;
use crate::kb::knowledge_base::{LeveledItem, TBox, TBoxItem, TbRule};
use crate::kb::types::CR;

#[derive(PartialEq, Debug, Clone)]
pub struct TBDllite {
    items: Vec<TbiDllite>,
    length: usize,
    completed: bool,
}

impl TBox for TBDllite {
    type TbiItem = TbiDllite;

    fn len(&self) -> usize {
        self.length
    }

    fn add(&mut self, tbi: TbiDllite) -> bool {
        /*
        returns true if the item was successfully inserted, false otherwise
         */
        if !self.items.contains(&tbi) {
            self.items.push(tbi);
            self.length += 1;
            true
        } else {
            false
        }
    }

    fn items(&self) -> &Vec<TbiDllite> {
        &(self.items)
    }

    fn get(&self, index: usize) -> Option<&TbiDllite> {
        self.items.get(index)
    }

    fn sort(&mut self) {
        self.items.sort();
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains(&self, tbi: &TbiDllite) -> bool {
        self.items.contains(tbi)
    }
}

/*
for the moment it is empty, but after I have to add functionality here
 */

impl fmt::Display for TBDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 {
            write!(f, "<TB>[]")
        } else {
            let mut s: String = String::from("<TB>[");

            for item in &self.items {
                s.push_str(item.to_string().as_str());
                s.push(',');
            }

            s.push(']');

            write!(f, "{}", s)
        }
    }
}

// adding Default Implementation: thanks Clippy
impl Default for TBDllite {
    fn default() -> Self {
        TBDllite::new()
    }
}

impl TBDllite {
    pub fn new() -> TBDllite {
        let items: Vec<TbiDllite> = Vec::new();
        TBDllite {
            items,
            length: 0,
            completed: false,
        }
    }

    /*
    pub fn new_from_iter<I>(it: I) -> TB_DLlite
    where
        I: Iterator<Item = TBI_DLlite>,
    {
        let mut tb = TB_DLlite::new();

        for item in it {
            tb.add(item);
        }

        tb
    }

     */

    pub fn levels(&self) -> Vec<usize> {
        let levels: Vec<usize> = self.items.iter().map(|x| (&x).level()).collect();

        levels
    }

    /*
    pub fn completed(&self) -> &bool {
        &(self.completed)
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn remove_trivial(&mut self) {
        if !self.items.is_empty() {
            let mut new_items: Vec<TBI_DLlite> = Vec::new();

            while !self.items.is_empty() {
                let tbi = self.items.pop().unwrap();

                if !(&tbi).is_trivial() {
                    new_items.push(tbi);
                }
            }

            self.items = new_items;
        }
    }

     */

    // get a list to negative inclusions
    pub fn negative_inclusions(&self, take_trivial: bool) -> Vec<&TbiDllite> {
        let mut neg_tbi: Vec<&TbiDllite> = Vec::new();

        for tbi in &self.items {
            if tbi.is_negative_inclusion() && (!tbi.is_trivial() || take_trivial) {
                neg_tbi.push(tbi);
            }
        }

        neg_tbi
    }

    // TODO: this is maybe the most important function, to update !!!!
    pub fn cln_completion(&self, negative_closure: bool, deduction_tree: bool, verbose: bool) -> TBDllite {

        let mut cln_tbox = TBDllite::new();

        cln_tbox
    }

    pub fn complete(&self, deduction_tree: bool, verbose: bool) -> TBDllite {
        // TESTING: for type constriction
        type T = TbiDllite;

        if self.items.is_empty() {
            if verbose {
                println!("the tbox is empty, nothing to complete");
            }

            TBDllite::new()
        } else {
            /*
            the strategy is as follows, for each Vec or VecDeque keeps two, one that change during the
            loop, one that is updated at the end of each loop
             */

            // keep the items
            let items: Arc<Mutex<VecDeque<TbiDllite>>> = Arc::new(Mutex::new(VecDeque::new()));
            let items_temporal: Arc<Mutex<VecDeque<TbiDllite>>> =
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
            let mut current_item: TbiDllite; // at each iteration keeps the item to be treated
            let mut is_already_treated: bool;
            let mut iterations: usize;

            // index trackers
            let mut length: usize; // updates whenever a phase is finished
            let mut length_temporal: usize; // the length of the 'items_temporal' queue

            /*
            I WILL PUT THE RULES HERE, WE CAN ADD OTHERS IF NEEDED
            */
            let rule_zero: TbRule<T> = dl_lite_rule_zero;
            let rule_one: TbRule<T> = dl_lite_rule_one;
            let rule_two: TbRule<T> = dl_lite_rule_two;
            let rule_three: TbRule<T> = dl_lite_rule_three;
            let rule_four: TbRule<T> = dl_lite_rule_four;
            let rule_five: TbRule<T> = dl_lite_rule_five;
            let rule_six: TbRule<T> = dl_lite_rule_six;
            let rule_seven: TbRule<T> = dl_lite_rule_seven;
            let rule_eight: TbRule<T> = dl_lite_rule_eight;

            let number_of_rules: usize = 7;
            let rules: [&TbRule<T>; 7] = [
                &rule_two,
                &rule_three,
                &rule_four,
                &rule_five,
                &rule_six,
                &rule_seven,
                &rule_eight,
            ];
            let rule_ordinal = [
                CR::Second,
                CR::Third,
                CR::Fourth,
                CR::Fifth,
                CR::Sixth,
                CR::Seventh,
                CR::Eight,
            ];

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

            // apply zero rule before the main loop
            // X=>Top and Bottom=>Y
            {
                // first put everything in 'items_temporal'
                {
                    length_temporal = 0;
                    let items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    for index in 0..length {
                        let item = &items[index];

                        // here we add the deduction tree switch
                        let new_item_vec =
                            TbiDllite::apply_rule(vec![item], &rule_zero, deduction_tree);

                        // here there is some unnecessary clone stuff
                        if let Some(some_vec) = new_item_vec {
                            length_temporal = complete_helper_add_if_necessary_general(
                                &items,
                                &mut items_temporal,
                                vec![item],
                                &some_vec, // always one element
                                length_temporal,
                                verbose,
                                CR::Zero,
                            );
                        }
                    }
                }

                // then dump everything in 'items' from 'items_temporal'
                {
                    let mut items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    length = complete_helper_dump_from_mutex_temporal_to_current(
                        &mut items,
                        &mut items_temporal,
                        length,
                        length_temporal,
                        Option::None,
                        verbose,
                    );
                }
            }
            // end of zero rule

            // apply first rule before the main loop
            // A=>notB then B=>notA
            {
                // first put everything in 'items_temporal'
                {
                    length_temporal = 0;
                    let items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    for index in 0..length {
                        let item = &items[index];

                        // added deduction tree here
                        let new_item_vec =
                            TbiDllite::apply_rule(vec![item], &rule_one, deduction_tree);

                        // here there is some unnecessary clone stuff
                        if let Some(some_vec) = new_item_vec {
                            length_temporal = complete_helper_add_if_necessary_general(
                                &items,
                                &mut items_temporal,
                                vec![item],
                                &some_vec, // always one element
                                length_temporal,
                                verbose,
                                CR::First,
                            );
                        }
                    }
                }

                // then dump everything in 'items' from 'items_temporal'
                {
                    let mut items = items.lock().unwrap();
                    let mut items_temporal = items_temporal.lock().unwrap();

                    length = complete_helper_dump_from_mutex_temporal_to_current(
                        &mut items,
                        &mut items_temporal,
                        length,
                        length_temporal,
                        Option::None,
                        verbose,
                    );
                }
            }
            // end of first rule

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
                            "==================================================================="
                        );
                        println!(
                            "--------this is the status at beginning of iteration {}------------",
                            iterations
                        );
                        println!("-- items: {:?}", &items);
                        println!("-- to_treat: {:?}", &to_treat);
                        println!("-- already_treated: {:?}", &already_treated);
                        println!(
                            "-------------------------------------------------------------------"
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
                            "---- treating now {} (index {})",
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
                            let rule: &TbRule<T> = rules[rule_index];
                            let rule_ord = rule_ordinal[rule_index];

                            // three different vectors
                            // added deduction tree
                            let new_item_vec3 = TbiDllite::apply_rule(
                                vec![&current_item, &item],
                                rule,
                                deduction_tree,
                            );

                            for optional_vec in &[&new_item_vec3] {
                                // if the rule succeeded

                                if optional_vec.is_some() {
                                    let mut tbis_to_add: Vec<TbiDllite> = Vec::new();
                                    let iterator = optional_vec.as_ref().unwrap();
                                    // try to apply rule zero and one
                                    for tbi in iterator {
                                        // added deduction tree
                                        let zero_tbi = TbiDllite::apply_rule(
                                            vec![tbi],
                                            &rule_zero,
                                            deduction_tree,
                                        );
                                        let one_tbi = TbiDllite::apply_rule(
                                            vec![tbi],
                                            &rule_one,
                                            deduction_tree,
                                        );

                                        tbis_to_add.push(tbi.clone());

                                        if let Some(some_tbi) = zero_tbi {
                                            tbis_to_add.push(some_tbi[0].clone());
                                        }

                                        if let Some(some_tbi) = one_tbi {
                                            tbis_to_add.push(some_tbi[0].clone());
                                        }
                                    }

                                    length_temporal = complete_helper_add_if_necessary_general(
                                        &items,
                                        &mut items_temporal,
                                        vec![&current_item, &item],
                                        &tbis_to_add, // always one element
                                        length_temporal,
                                        verbose,
                                        rule_ord,
                                    );
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

                    println!("-------------------------------------------------------------------");
                    println!(
                        "-----------this is the status at end of iteration {}----------------",
                        iterations
                    );
                    println!("-- items: {:?}", &items);
                    println!("-- to_treat: {:?}", &to_treat);
                    println!("-- already_treated: {:?}", &already_treated);
                    println!("===================================================================");
                }

                // update the iteration counter
                iterations += 1;
            }

            let mut new_tb = TBDllite::new();
            {
                let mut items = items.lock().unwrap();
                while !items.is_empty() {
                    new_tb.add(items.pop_front().unwrap());
                }
            }

            // of course, set completed to 'true' in the new tbox
            new_tb.completed = true;
            new_tb
        }
    }

    /*
    pub fn is_satisfiable(&self, deduction_tree: bool, verbose: bool) -> bool {
        let new_tb = self.complete(deduction_tree, verbose);

        for tbi in new_tb.items {
            if tbi.is_contradiction() && !tbi.is_trivial() {
                return false;
            }
        }

        return true;
    }

     */

    pub fn get_tbis_by_level(&self, only_conflicts: bool) -> Vec<usize> {
        let max_level = self.get_max_level();
        let mut levels: Vec<usize> = vec![0; max_level + 1];
        let mut lev: usize;

        for tbi in self.items() {
            lev = tbi.level();

            // add only contradictions
            if tbi.is_contradiction() || !only_conflicts {
                levels[lev] += 1;
            }
        }

        levels
    }

    pub fn get_max_level(&self) -> usize {
        get_max_level_abstract(self.items())
    }
}
