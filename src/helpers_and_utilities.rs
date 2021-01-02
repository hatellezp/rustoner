use crate::tbox_item::TBI;
use crate::types::CR;

use std::collections::VecDeque;
use std::sync::MutexGuard;

pub fn tbox_complete_helper_add_if_necessary_one(
    mu_all: &MutexGuard<VecDeque<TBI>>,
    mu: &mut MutexGuard<VecDeque<TBI>>,
    item: &TBI,
    new_item: TBI, // this one is consumed
    mut mu_length: usize,
    verbose: bool,
    rn: CR,
) -> usize {
    if mu_all.contains(&new_item) {
        if verbose {
            println!("---- {} rule applied here for {}, giving {}, but the item won't be added, it already exists", rn, item, &new_item);
        }
    } else {
        if verbose {
            println!(
                "---- {} rule applied here for {}, giving {}",
                rn, item, &new_item
            );
        }

        // add it to the items queue
        mu.insert(mu_length, new_item);

        // update the item's counter
        mu_length += 1;
    }

    mu_length
}

pub fn tbox_complete_helper_add_if_necessary_two(
    mu_all: &MutexGuard<VecDeque<TBI>>,
    mu: &mut MutexGuard<VecDeque<TBI>>,
    current_item: &TBI,
    item: &TBI,
    new_item: TBI,
    mut mu_length: usize,
    verbose: bool,
    rn: CR,
) -> usize {
    if mu_all.contains(&new_item) {
        if verbose {
            println!("---- {} rule applied here for {} and {}, giving {}, but the item won't be added, it already exists", rn, current_item, item, &new_item);
        }
    } else {
        if verbose {
            println!(
                "---- {} rule applied here for {} and {}, giving {}",
                rn, current_item, item, &new_item
            );
        }

        // add it to the items queue
        mu.insert(mu_length, new_item);

        // update the item's counter
        mu_length += 1;
    }

    mu_length
}

// find a way to avoid duplicate code here !!!
pub fn tbox_complete_helper_dump_from_mutex_temporal_to_current2(
    mu: &mut MutexGuard<VecDeque<TBI>>,
    mu_temp: &mut MutexGuard<VecDeque<TBI>>,
    mut mu_length: usize,
    mu_temp_length: usize,
    mu_to_treat: Option<&mut MutexGuard<VecDeque<usize>>>,
    verbose: bool,
) -> usize {
    // if to_treat is present then use it
    let to_treat_is_present = mu_to_treat.is_some();

    if to_treat_is_present {
        let to_treat = mu_to_treat.unwrap();
        for _ in 0..mu_temp_length {
            let new_item = (*mu_temp).pop_back().unwrap();

            // be careful here with the index
            if verbose {
                println!(
                    "---- adding {} to mutex with index {}",
                    &new_item, mu_length
                );
                println!("---- adding {} to 'to_treat' index", mu_length);
            }

            (*mu).insert(mu_length, new_item);

            // here to_treat is present
            (*to_treat).push_front(mu_length);

            mu_length += 1;
        }

        mu_length
    } else {
        for _ in 0..mu_temp_length {
            let new_item = (*mu_temp).pop_back().unwrap();

            // be careful here with the index
            if verbose {
                println!(
                    "---- adding {} to mutex with index {}",
                    &new_item, mu_length
                );
                println!("---- adding {} to 'to_treat' index", mu_length);
            }

            (*mu).insert(mu_length, new_item);
            mu_length += 1;
        }

        mu_length
    }
}
