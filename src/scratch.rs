use crate::tbox::TB;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::tbox_item::TBI;
use crate::types::DLType;
use crate::node::{Node, Mod};
use std::ops::Deref;

pub fn complete(&mut old_tb: TB, verbose: bool) -> TB {
    /*
    the strategy is as follows, for each Vec or VecDeque keeps two, one that change during the
    loop, one that is updated at the end of each loop
     */

    // keep the items
    let mut items: Arc<Mutex<VecDeque<TBI>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut items_temporal: Arc<Mutex<VecDeque<TBI>>> = Arc::new(Mutex::new(VecDeque::new()));

    // keep the index to be treated
    let mut to_treat: Arc<Mutex<VecDeque<usize>>> = Arc::new(Mutex::new(VecDeque::new()));
    let mut to_treat_temporal: Arc<Mutex<VecDeque<usize>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    // keep the index that have already been treated
    let mut already_treated: Arc<Mutex<VecDeque<usize>>> =
        Arc::new(Mutex::new(VecDeque::new()));
    let mut already_treated_temporal: Arc<Mutex<VecDeque<usize>>> =
        Arc::new(Mutex::new(VecDeque::new()));

    // indicators for the while main loop
    let mut stop_condition: bool; // stop condition for the loop, see if to_treat is 'empty' or not
    let mut current_index: usize; // at each iteration keeps the index to be treated
    let mut current_item: TBI; // at each iteration keeps the item to be treated
    let mut items_temporal_length: usize; // the length of the 'items_temporal' queue
    let mut look_out_for_roles: bool; // witness for treating role types
    let mut is_already_treated: bool;
    let mut iterations: usize;

    // index trackers
    let mut item_counter: usize; // keep track of number of items
    let mut current_length: usize; // updates whenever a phase is finished

    // if some index have to kept, this vectors works in that
    let mut important_index: VecDeque<usize> = VecDeque::new();

    // begin by putting every element in the items queue
    item_counter = 0;
    for item in &old_tb.items {
        let mut items = items.lock().unwrap();
        items.insert(item_counter, item.clone());

        // keep track of negated items
        if item.rside().is_negated() {
            important_index.push_front(item_counter);
        }

        // update the item counter index
        item_counter += 1;
    }

    // update the current_length
    current_length = item_counter;

    // add negations
    // first rule: A=>notB then B=>notA
    {
        // unwrap items
        let mut items = items.lock().unwrap();

        for index in 0..current_length {
            // TODO: maybe I will have to wrap 'important_index' in a 'Arc(Mutex)' strategy too...
            if important_index.contains(&index) {
                // this index is one of a negated item
                // create the new item
                let item = &items[index];

                // whenever an item has a negation on the right side, it successfully
                // inverts the negation
                let new_item = item.reverse_negation().unwrap();

                if items.contains(&new_item) {
                    if verbose {
                        println!("---- first rule (negation rule) for {}, giving {}, but the item won't be added, it already exists", item, &new_item);
                    }
                } else {
                    if verbose {
                        println!(
                            "---- first rule (negation rule) for {}, giving {}",
                            item, &new_item
                        );
                    }

                    // add it to the items queue
                    items.insert(item_counter, new_item);

                    // update the item's counter
                    item_counter += 1
                }
            }
        }
    }

    // maybe empty the negated_index ?
    important_index.clear();

    // update the current length
    current_length = item_counter;

    // this queue keeps track of what elements have yet to be treated
    {
        let mut to_treat = to_treat.lock().unwrap();
        for index in 0..current_length {
            to_treat.push_front(index);
        }
    }

    /*
    begin main loop:
        - it pops an index from 'to_treat' and gets the element from 'items'
        - because we go in an orderly fashion, we don't have to verify that the items have
          been treated or not
        - this item, 'current_item', is the compared to every other item in 'items',
        - if a rule can be applied first add the new created item to 'items_temporal'
        - after the loop over 'items', put everything 'items_temporal' has in 'items'

     */
    stop_condition = true;
    iterations = 0;
    while stop_condition {
        {
            if verbose {
                let mut items = items.lock().unwrap();
                let mut to_treat = to_treat.lock().unwrap();
                let mut already_treated = already_treated.lock().unwrap();

                println!("===================================================================");
                println!(
                    "--------this is the status at beginning of iteration {}------------",
                    iterations
                );
                println!("-- items: {:?}", &items);
                println!("-- to_treat: {:?}", &to_treat);
                println!("-- already_treated: {:?}", &already_treated);
                println!("-------------------------------------------------------------------");
            }
        }

        // at the beginning some conditions have to be reinitialized
        stop_condition = false;
        is_already_treated = false;
        look_out_for_roles = false;
        items_temporal_length = 0;
        {
            // clear the temporal queues
            let mut items_temporal = items_temporal.lock().unwrap();
            let mut already_treated_temporal = already_treated_temporal.lock().unwrap();

            items_temporal.clear();
            already_treated_temporal.clear();
        }

        // get index from 'to_treat' and put it in current_index
        {
            let mut to_treat = to_treat.lock().unwrap();
            current_index = to_treat.pop_back().unwrap();
        }

        // here the test for 'already_treated'
        {
            let mut already_treated = already_treated.lock().unwrap();
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
        }

        // get item from 'items' thanks to current_index and put it in current_item
        {
            let mut items = items.lock().unwrap();
            current_item = items[current_index].clone();
        }

        if verbose {
            println!(
                "---- treating now: {} (index {})",
                &current_item, current_index
            );
        }

        look_out_for_roles =
            DLType::all_roles(current_item.lside().t(), current_item.rside().t());

        // this is the main for loop, where once current_item fixed, items pass and are
        // compared and see if one of the rules can be applied to it
        {
            // access the items
            let mut items = items.lock().unwrap();
            let mut items_temporal = items_temporal.lock().unwrap();

            // current_length has to have the exact value
            for index in 0..current_length {
                let item = &items[index];

                // second rule: A=>B and B=>C then A=>C
                if current_item.rside() == item.lside() {
                    let new_item =
                        TBI::new(current_item.lside().clone(), item.rside().clone()).unwrap();

                    // always use print before pushing
                    // 'new_item' will be added only if it is not present already in 'items'
                    if items.contains(&new_item) {
                        if verbose {
                            println!("     * second rule applied for {} and {}, giving {}, but won't be added, it already exists", &current_item, &item, &new_item);
                        }
                    } else {
                        if verbose {
                            println!(
                                "     * second rule applied for {} and {}, giving {}",
                                &current_item, &item, &new_item
                            );
                            items_temporal.push_front(new_item);
                            items_temporal_length += 1;
                        }
                    }
                }

                // the roles rules
                if look_out_for_roles {
                    // third rule r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB (because negations won't add themselves)
                    // fourth rule r1=>r2 and B=>notExists_r2_inv then B=>notExists_r1_inv
                    match &current_item.rside() {
                        Node::R(_) => {
                            // HAVE YOU SEEN THE DIFFERENCE IN LINES ???
                            match &item.rside() {
                                Node::X(Mod::N, bn) => {
                                    match bn.deref() {
                                        // third rule r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB (because negations won't add themselves)
                                        Node::X(Mod::E, bn2) => {
                                            match bn2.deref() {
                                                // third rule r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB (because negations won't add themselves)
                                                Node::R(_) => {
                                                    if &bn2.deref() == &current_item.rside() {
                                                        // we get r2 == rX
                                                        println!("hello there");
                                                        // I have to create two different tbi
                                                        let exists_r1 = (*(&current_item
                                                            .lside()))
                                                            .clone()
                                                            .exists()
                                                            .unwrap();
                                                        let new_item1 = TBI::new(
                                                            (*(&item.lside())).clone(),
                                                            (exists_r1.clone()).negate(),
                                                        )
                                                            .unwrap();
                                                        let new_item2 = TBI::new(
                                                            exists_r1,
                                                            (*(&item.lside())).clone(),
                                                        )
                                                            .unwrap();

                                                        // now I can add the values
                                                        // but before verbose time
                                                        if items.contains(&new_item1) {
                                                            if verbose {
                                                                println!("     * third rule applied here for {} and {} giving {}, but won't be added, it already exists", &current_item, &item, &new_item1);
                                                            }
                                                        } else {
                                                            if verbose {
                                                                println!("     * third rule applied here for {} and {} giving {}", &current_item, &item, &new_item1);

                                                                items_temporal
                                                                    .push_front(new_item1);
                                                                items_temporal_length += 1;
                                                            }
                                                        }

                                                        // second item
                                                        if items.contains(&new_item2) {
                                                            if verbose {
                                                                println!("     * third rule applied here for {} and {} giving {}, but won't be added, it already exists", &current_item, &item, &new_item2);
                                                            }
                                                        } else {
                                                            if verbose {
                                                                println!("     * third rule applied here for {} and {} giving {}", &current_item, &item, &new_item2);

                                                                items_temporal
                                                                    .push_front(new_item2);
                                                                items_temporal_length += 1;
                                                            }
                                                        }
                                                    }
                                                }
                                                // fourth rule r1=>r2 and B=>notExists_r2_inv then B=>notExists_r1_inv
                                                Node::X(Mod::I, bn3) => {
                                                    if &bn3.deref() == &current_item.rside() {
                                                        // awful stuff
                                                        let new_item = TBI::new(
                                                            (*(&item.lside())).clone(),
                                                            (*(&current_item.lside()))
                                                                .clone()
                                                                .inverse()
                                                                .exists()
                                                                .unwrap()
                                                                .negate(),
                                                        )
                                                            .unwrap();

                                                        if items.contains(&new_item) {
                                                            if verbose {
                                                                println!("     * fourth rule applied here for {} and {} giving {}, but won't be added, it already exists", &current_item, &item, &new_item);
                                                            }
                                                        } else {
                                                            if verbose {
                                                                println!("     * fourth rule applied here for {} and {} giving {}", &current_item, &item, &new_item);

                                                                items_temporal
                                                                    .push_front(new_item);
                                                                items_temporal_length += 1;
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => (),
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                    // end of third rule
                    // end of fourth rule
                }
                // end of 'look_out_for_roles' first 'if' condition

                // fifth rule: Exists_r=>notExists_r then r=>not_r and Exists_r_inv=>notExists_r_inv

                // sixth rule: Exists_r_inv=>notExists_r_inv then r=>not_r and Exists_r=>notExists_r

                if look_out_for_roles {
                    // seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv

                    // eight rule: r1=>r2 then r1_inv=>r2_inv and Exists_r1=>Exists_r2
                }
            }
        }
        // end of rules application

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
            for index in 0..items_temporal_length {
                let new_item = items_temporal.pop_back().unwrap();

                // be careful here with the index
                if verbose {
                    println!(
                        "---- adding {} to 'items' with index {}",
                        &new_item, current_length
                    );
                    println!("---- adding {} to 'to_treat' index", current_length);
                }

                items.insert(current_length, new_item);
                to_treat.push_front(current_length);
                current_length += 1;
            }

            // this is for the index already treated
            while !already_treated_temporal.is_empty() {
                let already_treated_index = already_treated_temporal.pop_back().unwrap();
                already_treated.push_front(already_treated_index);
            }
        }

        // update the stop condition
        {
            let mut to_treat = to_treat.lock().unwrap();
            stop_condition = !to_treat.is_empty();
        }

        if verbose {
            let mut items = items.lock().unwrap();
            let mut to_treat = to_treat.lock().unwrap();
            let mut already_treated = already_treated.lock().unwrap();

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

    let mut new_tb = TB::new();
    {
        let mut items = items.lock().unwrap();
        while !items.is_empty() {
            new_tb.add(items.pop_front().unwrap());
        }
    }

    // of course, set completed to 'true' in the new tbox
    // *new_tb.completed() = true;
    new_tb
}