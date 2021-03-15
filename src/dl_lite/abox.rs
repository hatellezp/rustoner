use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::sync::{Arc, Mutex};

use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::helpers_and_utilities::{
    complete_helper_add_if_necessary_general, complete_helper_dump_from_mutex_temporal_to_current,
};

use crate::dl_lite::node::NodeDllite;
use crate::dl_lite::rule::{dl_lite_abox_rule_one, dl_lite_abox_rule_three, dl_lite_abox_rule_two};
use crate::dl_lite::string_formatter::{abi_to_string, abiq_in_vec_of_vec};
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TBI_DLlite;
use crate::kb::knowledge_base::{ABox, ABoxItem, AbRule, Item, SymbolDict, TBox, TBoxItem};
use crate::kb::types::{ConflictType, DLType, CR};

use petgraph::{Directed, Graph};

#[derive(PartialEq, Debug, Clone)]
pub struct AbqDllite {
    name: String,
    items: Vec<AbiqDllite>,
    completed: bool,
    length: usize,
}

impl ABox for AbqDllite {
    type AbiItem = AbiqDllite;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn len(&self) -> usize {
        // i'm changing this for the time being
        // self.length
        self.items.len()
    }

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

    fn items(&self) -> &Vec<AbiqDllite> {
        &self.items
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

            s.push_str("]");

            write!(f, "{}", s)
        }
    }
}

impl AbqDllite {
    pub fn new(name: &str) -> AbqDllite {
        AbqDllite {
            name: name.to_string(),
            items: vec![],
            length: 0,
            completed: false,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut AbiqDllite> {
        self.items.get_mut(index)
    }

    pub fn from_vec(name: &str, mut v: Vec<AbiqDllite>) -> AbqDllite {
        let mut abq = AbqDllite::new(name);

        while !v.is_empty() {
            let abiq = v.pop().unwrap();

            abq.add(abiq);
        }

        abq
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    // create an abox from a vec of index, will help when finding conflicts in a database
    pub fn sub_abox(&self, index: Vec<usize>, name: Option<&str>) -> Option<AbqDllite> {
        let name = match name {
            Option::None => "tmp",
            Some(s) => s,
        };

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

    pub fn is_inconsistent_detailed(
        &self,
        tb: &TBDllite,
        verbose: bool,
    ) -> Vec<(TBI_DLlite, Vec<AbiqDllite>)> {
        let tbis = tb.items();

        // we can have a:A and A < (-A)
        // also a:A a:B and A < (-B) the first case is an special case, so we test for the second
        // only

        let mut contradictions: Vec<(TBI_DLlite, Vec<AbiqDllite>)> = Vec::new();

        let mut combs_added_one: Vec<&AbiqDllite> = Vec::new();
        let mut combs_added_two: Vec<(&AbiqDllite, &AbiqDllite)> = Vec::new();

        let self_length = self.length;

        for tbi in tbis {
            if verbose {
                println!(" -- ABQ::is_inconsistent: comparing against {}", &tbi);
            }

            let left = tbi.lside();
            let right = tbi.rside();
            let right_negated = right.clone().negate();

            for i in 0..self_length {
                let abq_i = self.items.get(i).unwrap();
                let node_i = abq_i.abi().symbol();

                if verbose {
                    println!(
                        " -- ABQ::is_inconsistent: analysing {} with index {}",
                        abq_i, i
                    );
                }

                if node_i == left {
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

                            // verify that elements has not yer been added
                            let only_one = abq_i == abq_j;
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
        let mut lside: &NodeDllite;
        let mut rside: &NodeDllite;

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

                    // println!("------------------comparing\n               tbi: {}\n               abiq_{}: {}\n               abiq_{}: {}", tbi, i, abiq_i, j, abiq_j);

                    if abiq_i.same_nominal(abiq_j) {
                        // println!("--------------------they have same nominal");

                        is_match = (abiq_i.item() == lside && abiq_j.item().is_negation(rside))
                            || (abiq_i.item() == rside && abiq_j.item().is_negation(lside));

                        // there a second test:
                        if i != j {
                            // let negated_j = abiq_j.item().clone().negate();
                            // println!("------------------negated_{} is {}", j, &negated_j);

                            if abiq_j.item().is_negation(abiq_i.item()) {
                                // println!("  --  found contradiction without tbi: abiq_{}: {}, abiq_{}: {}", i, abiq_i, j, abiq_j);
                                return true;
                            }
                        }

                        if is_match {
                            // println!("  --  found contradiction: abiq_i: {}, abiq_j: {}, tbi: {}", abiq_i, abiq_j, tbi);
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

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
        type T = TBI_DLlite;
        type A = AbiqDllite;

        if self.items.len() == 0 {
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

                                for optional_vec in vec![&new_item_vec3] {
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
            new_abq
        }
    }

    pub fn abiq_is_self_contradicting(abiq: &AbiqDllite, tb: &TBDllite) -> bool {
        let mut new_ab = AbqDllite::new("temp");
        new_ab.add(abiq.clone());

        new_ab = new_ab.complete(tb, false, false);

        new_ab.is_inconsistent(tb, false)
    }

    pub fn get_max_level(&self) -> usize {
        let mut max_level: usize = 0;

        for tbi in &self.items {
            max_level = max_level.max(tbi.level());
        }

        max_level
    }

    pub fn get_abis_by_level(
        &self,
        _tb: &TBDllite,
        only_conflicts: bool,
        contradictions: &Vec<(TBI_DLlite, Vec<AbiqDllite>)>,
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

    pub fn create_graph_dot<'a>(
        &self,
        symbols: &SymbolDict,
        conflict_matrix: &Vec<i8>,
        real_to_virtual: &HashMap<usize, usize>,
        conflict_type: &HashMap<usize, ConflictType>,
    ) -> Graph<String, bool, Directed, u32> {
        let mut graph: Graph<String, bool> = Graph::new();
        let mut string_op: Option<String>;
        let mut string_node: String;
        let mut abiq: &AbiqDllite;
        let mut conft: ConflictType;

        // dict for index
        let mut index_dict = HashMap::new();

        // first add all the nodes
        for i in 0..self.len() {
            abiq = self.items.get(i).unwrap();
            conft = *(conflict_type.get(&i).unwrap());

            match conft {
                ConflictType::SelfConflict => (),
                ConflictType::Clean => (),
                ConflictType::Conflict => {
                    string_op = abi_to_string(abiq.abi(), symbols);

                    string_node = match string_op {
                        Option::None => {
                            format!("{}, v: {}", abiq.abi(), abiq.value().unwrap_or(1.))
                        }
                        Some(s) => {
                            format!("{}, v: {}", s, abiq.value().unwrap_or(1.))
                        }
                    };

                    let index = graph.add_node(string_node);
                    index_dict.insert(i, index);
                }
            }
        }

        // now add the edges
        let dim = (conflict_matrix.len() as f64).sqrt() as usize;
        let mut real_i: usize;
        let mut real_j: usize;
        let mut w_ij: i8;
        for virtual_i in 0..dim {
            for virtual_j in 0..dim {
                w_ij = conflict_matrix[virtual_i * dim + virtual_j];
                if w_ij != 0 {
                    real_i = *(real_to_virtual.get(&virtual_i).unwrap());
                    real_j = *(real_to_virtual.get(&virtual_j).unwrap());

                    let index_i_op = index_dict.get(&real_i);
                    let index_j_op = index_dict.get(&real_j);

                    match (index_i_op, index_j_op) {
                        (Some(index_i), Some(index_j)) => {
                            if w_ij > 0 {
                                // i is implied by j
                                graph.add_edge(*index_j, *index_i, true); // an arrow from j to i
                            } else {
                                // i is refuted by j
                                graph.add_edge(*index_j, *index_i, false); // an arrow from j to i
                            }
                        }
                        (_, _) => (), // passing
                    }
                }

                if conflict_matrix[virtual_i * dim + virtual_j] > 0 {
                    // i is implied by j
                } else if conflict_matrix[virtual_i * dim + virtual_j] < 0 {
                    // i is refuted by j
                }
            }
        }

        graph
    }
}
