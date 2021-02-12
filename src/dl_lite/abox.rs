use std::fmt;

use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::helpers_and_utilities::{
    complete_helper_add_if_necessary_general, complete_helper_dump_from_mutex_temporal_to_current2,
};
use crate::dl_lite::rule::{
    dl_lite_abox_rule_one, dl_lite_abox_rule_three, dl_lite_abox_rule_two, AbRule,
};
use crate::dl_lite::tbox::TB;
use crate::dl_lite::types::CR;
use crate::kb::knowledge_base::Data;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Debug, Clone)]
pub struct AB {
    name: String,
    items: Vec<ABI>,
    completed: bool,
    length: usize,
}

impl Data for AB {}

impl fmt::Display for AB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 {
            write!(f, "<AB>[]")
        } else {
            let mut s: String = format!("<AB({})>[", self.name);

            for item in &self.items {
                s.push_str(item.to_string().as_str());
                s.push_str(", ");
            }

            s.push_str("]");

            write!(f, "{}", s)
        }
    }
}

impl AB {
    pub fn new(name: &str) -> AB {
        AB {
            name: name.to_string(),
            items: vec![],
            length: 0,
            completed: false,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn add(&mut self, abi: ABI) -> bool {
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

    pub fn items(&self) -> &Vec<ABI> {
        &self.items
    }

    pub fn complete(&self, tbox: &TB, verbose: bool) -> AB {
        if self.items.len() == 0 {
            if verbose {
                println!("the abox is empty, nothing to complete");
            }

            let new_name = format!("{}_completion_not", &self.name);
            AB::new(&new_name)
        } else {
            /*
            the strategy is as follows, for each Vec or VecDeque keeps two, one that change during the
            loop, one that is updated at the end of each loop
             */

            // keep the items
            let items: Arc<Mutex<VecDeque<ABI>>> = Arc::new(Mutex::new(VecDeque::new()));
            let items_temporal: Arc<Mutex<VecDeque<ABI>>> = Arc::new(Mutex::new(VecDeque::new()));

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
            let mut current_item: ABI; // at each iteration keeps the item to be treated
            let mut is_already_treated: bool;
            let mut iterations: usize;

            // index trackers
            let mut length: usize; // updates whenever a phase is finished
            let mut length_temporal: usize; // the length of the 'items_temporal' queue

            /*
            I WILL PUT THE RULES HERE, WE CAN ADD OTHERS IF NEEDED
            */
            let rule_one: AbRule = dl_lite_abox_rule_one;
            let rule_two: AbRule = dl_lite_abox_rule_two;
            let rule_three: AbRule = dl_lite_abox_rule_three;

            let number_of_rules: usize = 3;
            let rules: [&AbRule; 3] = [&rule_one, &rule_two, &rule_three];

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
                            let rule: &AbRule = rules[rule_index];
                            let rule_ord = rule_ordinal[rule_index];

                            // use each item
                            for tbi in tbox.items() {
                                // three different vectors

                                if verbose {
                                    println!("---- comparing with tbi: {}", tbi);
                                }

                                let mut new_item_vec3 =
                                    ABI::apply_rule(vec![&current_item, &item], vec![tbi], rule);

                                for optional_vec in vec![&new_item_vec3] {
                                    // if the rule succeeded

                                    // println!("--- in abox complete, optional vec is : {:?}", &optional_vec);

                                    if optional_vec.is_some() {
                                        let mut abis_to_add: Vec<ABI> = Vec::new();
                                        let iterator = optional_vec.as_ref().unwrap();

                                        let mut abi_already_exits = false;

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
                    length = complete_helper_dump_from_mutex_temporal_to_current2(
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

            let new_name = format!("{}_completed", self.name);
            let mut new_tb = AB::new(&new_name);
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
}
