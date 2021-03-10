use crate::kb::types::CR;

use crate::kb::knowledge_base::Implier;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::sync::MutexGuard;

pub fn complete_helper_dump_from_mutex_temporal_to_current<
    T: Display + PartialEq + Eq + Clone + Debug + Implier,
>(
    mu: &mut MutexGuard<VecDeque<T>>,
    mu_temp: &mut MutexGuard<VecDeque<T>>,
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

            if !mu.contains(&new_item) {
                // be careful here with the index
                if verbose {
                    println!(
                        " -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: adding {} to mutex with index {}",
                        &new_item, mu_length
                    );
                    println!(" -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: adding {} to 'to_treat' index", mu_length);
                }

                (*mu).insert(mu_length, new_item);

                // here to_treat is present
                (*to_treat).push_front(mu_length);

                mu_length += 1;
            } else {
                // already treated by adding impliers
                for item in mu.iter_mut() {
                    if item == &new_item {
                        let impliers = new_item.implied_by();
                        for implier in impliers {
                            item.add_to_implied_by(implier.clone());
                        }
                    }
                }

                if verbose {
                    println!(" -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: item {} already in mutex", &new_item);
                }
            }
        }

        mu_length
    } else {
        for _ in 0..mu_temp_length {
            let new_item = (*mu_temp).pop_back().unwrap();

            if !mu.contains(&new_item) {
                // be careful here with the index
                if verbose {
                    println!(
                        " -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: adding {} to mutex with index {}",
                        &new_item, mu_length
                    );
                    // println!(" -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: adding {} to 'to_treat' index", mu_length);
                }

                (*mu).insert(mu_length, new_item);

                // here to_treat is present
                // (*to_treat).push_front(mu_length);

                mu_length += 1;
            } else {
                // already treated by adding impliers
                for item in mu.iter_mut() {
                    if item == &new_item {
                        let impliers = new_item.implied_by();
                        for implier in impliers {
                            item.add_to_implied_by(implier.clone());
                        }
                    }
                }

                if verbose {
                    println!(" -- helpers_and_utilities::complete_helper_dump_from_mutex_temporal_to_current2: item {} already in mutex", &new_item);
                }
            }
        }

        mu_length
    }
}

pub fn complete_helper_add_if_necessary_general<T: Display + PartialEq + Eq + Clone + Implier>(
    mu_all: &MutexGuard<VecDeque<T>>,
    mu: &mut MutexGuard<VecDeque<T>>,
    applied_items: Vec<&T>,
    created_items: &Vec<T>,
    mut mu_length: usize,
    verbose: bool,
    rn: CR,
) -> usize {
    for new_item in created_items {
        if mu_all.contains(&new_item) || mu.contains(&new_item) {
            // if new_item is present then we need to update the implied by
            if mu_all.contains(&new_item) && !mu.contains(&new_item) {
                // this should be silent because the item already exists
                // add it to the items queue
                mu.insert(mu_length, new_item.clone());

                // update the item's counter
                mu_length += 1;

                // and now verify in the dump function to add impliers
            }
            if mu.contains(&new_item) {
                for item in mu.iter_mut() {
                    if &new_item == &item {
                        let impliers = new_item.implied_by();
                        for implier in impliers {
                            item.add_to_implied_by(implier.clone());
                        }
                    }
                }
            }

            if verbose {
                println!(" -- helpers_and_utilities::complete_helper_add_if_necessary_general: {} rule applied here for {}, giving {}, but the item won't be added, it already exists", rn, print_vector_of_tbi_references(&applied_items), &new_item);
            }
        } else {
            if verbose {
                println!(
                    " -- helpers_and_utilities::complete_helper_add_if_necessary_general: {} rule applied here for {}, giving {}",
                    rn,
                    print_vector_of_tbi_references(&applied_items),
                    &new_item
                );
            }

            // add it to the items queue
            mu.insert(mu_length, new_item.clone());

            // update the item's counter
            mu_length += 1;
        }
    }

    mu_length
}

fn print_vector_of_tbi<T: Display + PartialEq + Eq>(vec: &Vec<T>) -> String {
    let mut s = String::from("[");

    for item in vec {
        s.push_str(format!("{}, ", &item).as_str());
    }

    s.push_str("]");

    s
}

fn print_vector_of_tbi_references<T: Display + PartialEq + Eq>(vec: &Vec<&T>) -> String {
    let mut s = String::from("[");

    for item in vec {
        s.push_str(format!("{}, ", &item).as_str());
    }

    s.push_str("]");

    s
}

pub fn print_matrix<T: Display>(v: Vec<T>) {
    let mlength = v.len();
    let msize = (mlength as f64).sqrt() as usize;

    let mut s = String::from("[");

    for i in 0..msize {
        for j in 0..msize {
            let to_add = format!("{}", v.get(i * msize + j).unwrap());

            s.push_str(to_add.as_str());

            if j != msize - 1 {
                s.push_str(", ");
            }
        }

        if i != msize - 1 {
            s.push_str("\n");
        }
    }
    s.push_str("]");

    println!("{}", s);
}
