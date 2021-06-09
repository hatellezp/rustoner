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

use crate::kb::types::CR;

use crate::kb::knowledge_base::Implier;
use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::sync::MutexGuard;

/// ABox and TBox completion engines modify hashmaps in their inner workings.
/// Hashmaps usually cannot be modified and at the same time accessed, to avoid
/// corruption to the hash table.
/// Nevertheless we need to modify such hashmaps. We do this by wrapping these hashmaps
/// in a MutexGuard which ensures modification is done when not accessing is done.
/// Functions defined here modify these MutexGuards both in ABox and TBox engines.

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
    created_items: &[T],
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
                    if new_item == item {
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

/// Pretty printer for array of of tbis.
fn print_vector_of_tbi_references<T: Display + PartialEq + Eq>(vec: &[&T]) -> String {
    let mut s = String::from("[");

    for item in vec {
        s.push_str(format!("{}, ", &item).as_str());
    }

    s.push(']');
    s
}
