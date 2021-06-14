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

use crate::alg_math::interface::{DataHolder, DataItem, Oracle};

use rustoner::kb::knowledge_base::AggrFn;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::Deref;

/// This file comprehends two main functions, computing the Indicator function,
/// which is a hashmap, and building a matrix for computing the abox ranking.

pub struct Builder {
    I: Box<Indicator>,
    C: Box<Credibility>,
    F: Box<Filter>,
    built: bool,
}

impl Builder {
    pub fn new(i: Indicator, c: Credibility, f: Filter) -> Builder {
        let I = Box::new(i);
        let C = Box::new(c);
        let F = Box::new(f);
        let built = false;

        Builder { I, C, F, built }
    }

    pub fn reset(&mut self) {
        self.I.reset();
        self.C.reset();
        self.F.reset();
        self.built = false;
    }

    pub fn build_matrix<
        DI: DataItem,
        DH: DataHolder + DataHolder<DI = DI>,
        O: Oracle + Oracle<DH = DH>,
    >(
        &mut self,
        dh: &DH,
        ora: &O,
        credibility_vector: &[f64],
        aggf: &AggrFn,
        conflict_limit: Option<usize>,
    ) -> Vec<f64> {
        /*
        this function supposes no assertion in the DataHolder structure is self-conflicting!
         */

        let length = dh.len(); // get the size
        let mut matrix: Vec<f64> = vec![0_f64; length * length]; // matrix with the required dimension

        // check if the builder has gathered information, otherwise reset and build
        if !self.built {
            self.reset();
            self.build_values(dh, ora, credibility_vector, aggf, conflict_limit);
        }

        // do not iterate over indexes, iterate on the key of the indicator function
        for (key, value) in self.I.deref().indicator() {
            let alpha_index = key.0;
            let b_index = key.1;

            let positive_or_negative = value.0;
            let beta_indices = &value.1;

            // get the credibility value
            let aggf_b_op = self.C.get(&b_index);

            match aggf_b_op {
                None => {
                    println!("data corrupted, the built of values failed!");
                    std::process::exit(exitcode::DATAERR)
                }
                Some(aggf_b) => {
                    // update all a_ij in the matrix (vector) where
                    // i = alpha_index
                    // j is in the beta_indices array

                    for beta_index in beta_indices {
                        matrix[length * alpha_index + *beta_index] +=
                            (positive_or_negative as f64) * (*aggf_b);
                    }
                }
            }
        }

        matrix
    }

    // TODO: check for subsets before checking inconsistency
    //       optimization: see the condition that allow for deduction of I values
    //                     before computation
    pub fn build_values<
        DI: DataItem,
        DH: DataHolder + DataHolder<DI = DI>,
        O: Oracle + Oracle<DH = DH>,
    >(
        &mut self,
        dh: &DH,
        ora: &O,
        credibility_vector: &[f64],
        aggf: &AggrFn,
        conflict_limit: Option<usize>,
    ) {
        // initialize or reinitialize the builder struct
        self.reset();

        let length = dh.len();
        let real_conflict_limit = if matches!(conflict_limit, Some(_)) {
            min(length, conflict_limit.unwrap())
        } else {
            length
        };

        // build the indicator function for each item in DataHolder
        for index in 0..length {
            let di_op: Option<&DI> = dh.get(index);

            match di_op {
                None => (),
                Some(alpha) => {
                    // now we have to build every entry in the indicator function and credibility function

                    // here we keep track of already done subsets
                    let mut subsets_done: Vec<Vec<usize>> = Vec::new();

                    // first reset the filter (and only the filter)
                    self.F.reset();

                    while self.F.noo() <= real_conflict_limit {
                        // get the indices if form of a filter of boolean
                        let filter = self.F.filter();

                        // this is the first thing to do, no need to analyze if index of
                        // alpha is present in in filter
                        // we need di not in this filter, otherwise we pass
                        if !self.F.filter()[index] {
                            // before everything I need a way to check no subset has been analysed yet
                            // TODO: find a way to solve the problem above

                            // present in B
                            let mut b_indices: Vec<usize> = Vec::new();
                            for (i, in_or_not) in filter.iter().enumerate().take(length) {
                                if *in_or_not {
                                    b_indices.push(i);
                                }
                            }
                            b_indices.sort_unstable();
                            // now B_indices has the index that are present in B (and sorted)

                            // we need to compare B_indices with the already computed subsets
                            // for the moment we will let it pass every time
                            let mut no_subset_of_filter_present: bool = true;

                            for subset in &subsets_done {
                                if is_superset(subset, &b_indices) {
                                    no_subset_of_filter_present = false;
                                    break;
                                }
                            }
                            // verification done

                            if no_subset_of_filter_present {
                                // two conditions passed for the moment:
                                // - di is not in B
                                // - B is minimal with respect to those done

                                // now we can add the new indices to the subsets_done witness
                                subsets_done.push(b_indices.clone());

                                // di is not in this filter
                                // create sub_dh
                                let b_subset: DH = dh.sub_data_holder(filter);

                                // check the third (and last) condition: B is consistent
                                if ora.is_consistent(&b_subset) {
                                    let _alpha_neg = alpha.negate();

                                    let b_alpha_positive = b_subset.clone();
                                    let b_alpha_negative = b_subset;

                                    b_alpha_positive.add_item(alpha.clone());
                                    b_alpha_negative.add_item(alpha.negate());

                                    let b_implies_not_alpha =
                                        ora.is_inconsistent(&b_alpha_positive);
                                    let b_implies_alpha = ora.is_inconsistent(&b_alpha_negative);

                                    // we filter each possibility
                                    // both true means B is inconsistent (which should not arrive)
                                    // and we let it pass
                                    // if both false, we also let it pass
                                    // only when one of and only one is true, there is information
                                    // that we need to store

                                    match (b_implies_not_alpha, b_implies_alpha) {
                                        (false, false) | (true, true) => (),
                                        (_, _) => {
                                            // find the value of B
                                            let aggf_b = compute_aggregation_from_filter(
                                                &aggf,
                                                credibility_vector,
                                                &filter,
                                            );

                                            // and put it in credibility
                                            // self.C.insert(index, aggf_b);

                                            // the line just above is erroneous, the indentifier
                                            // of aggf(B) is self.F.filter_index(),
                                            // that is, the index of B, and not the index of alpha
                                            // here I put it correctly
                                            self.C.insert(self.F.filter_index(), aggf_b);

                                            let ivalue: i8 = if b_implies_alpha { 1 } else { -1 };

                                            self.I.insert(
                                                (index, self.F.filter_index()),
                                                (ivalue, b_indices),
                                            );
                                        }
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }

        self.built = true;
    }
}

/// The indicator struct models the function Indicator.
/// It has
/// - an indicator field to store the result of the function,
/// - a conflict limit size (that will default to length size if none is provided) to
///   limit the search of conflict to a certain size
/// - the actual length of the knowledge base in question
/// - an id_generator, this function will generate a filter for subsets of the knowledge base
///   e.g. suppose length = 3, then the result of id_generator is the following
///   - id_generator(0) -> [false, false, false]
///   - id_generator(1) -> [true, false, false]
///   - id_generator(2) -> [false, true, false]
///   - id_generator(3) -> [false, false, true]
///   - ...
///   - id_generator(7) -> [true, true, true]
/// The id_generator function generates filter for the subsets of the knowledge base following
/// first an order of size and for a fixed size a lexicographic order.
#[derive(Debug)]
pub struct Indicator {
    // I(id of B, index of alpha) -> (value of I, index of beta in B)
    indicator: HashMap<(usize, usize), (i8, Vec<usize>)>,
}

impl Indicator {
    pub fn reset(&mut self) {
        self.indicator = HashMap::new();
    }

    pub fn insert(&mut self, k: (usize, usize), v: (i8, Vec<usize>)) -> Option<(i8, Vec<usize>)> {
        self.indicator.insert(k, v)
    }

    pub fn indicator(&self) -> &HashMap<(usize, usize), (i8, Vec<usize>)> {
        &self.indicator
    }
}

#[derive(Debug)]
pub struct Credibility {
    credibility_function: HashMap<usize, f64>,
}

impl Credibility {
    pub fn new() -> Credibility {
        let credibility_function: HashMap<usize, f64> = HashMap::new();

        Credibility {
            credibility_function,
        }
    }

    pub fn reset(&mut self) {
        self.credibility_function = HashMap::new();
    }

    pub fn insert(&mut self, k: usize, v: f64) -> Option<f64> {
        self.credibility_function.insert(k, v)
    }

    pub fn get(&self, k: &usize) -> Option<&f64> {
        self.credibility_function.get(k)
    }
}

pub struct Filter {
    length: usize,
    number_of_ones: usize,
    lower_one: usize,
    upper_one: usize,
    filter_index: usize,
    filter: Vec<bool>,
}

impl Filter {
    pub fn new(length: usize) -> Filter {
        let filter = vec![false; length];

        let number_of_ones: usize = 0;
        let lower_one: usize = 0;
        let upper_one: usize = 0;
        let filter_index: usize = 0;

        Filter {
            length,
            number_of_ones,
            lower_one,
            upper_one,
            filter_index,
            filter,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn noo(&self) -> usize {
        self.number_of_ones
    }

    pub fn lo(&self) -> usize {
        self.lower_one
    }

    pub fn uo(&self) -> usize {
        self.upper_one
    }

    pub fn filter_index(&self) -> usize {
        self.filter_index
    }

    pub fn filter(&self) -> &Vec<bool> {
        &self.filter
    }

    pub fn filter_mut(&mut self) -> &mut Vec<bool> {
        &mut self.filter
    }

    pub fn reset(&mut self) {
        self.filter = vec![false; self.length];
        self.number_of_ones = 0;
        self.lower_one = 0;
        self.upper_one = 0;
        self.filter_index = 0;
    }

    pub fn is_done(&self) -> bool {
        self.number_of_ones == self.length
    }

    pub fn next(&mut self) {
        // returns (new number of ones, new lower one, new upper one)
        // it also modifies the filter

        let total_size = self.length;
        let number_of_ones = self.number_of_ones;
        let lower_one = self.lower_one;
        let upper_one = self.upper_one;

        if total_size != self.number_of_ones {
            // update the filter
            self.filter_index += 1;

            if number_of_ones == 0 {
                self.filter[0] = true;

                self.number_of_ones = 1;
                self.lower_one = 0;
                self.upper_one = 0;
            } else if (total_size - number_of_ones) == lower_one {
                let new_number_of_ones = number_of_ones + 1;
                let new_lower_one: usize = 0;
                let new_upper_one: usize = number_of_ones;

                for i in 0..total_size {
                    if i <= new_upper_one {
                        self.filter[i] = true;
                    } else {
                        self.filter[i] = false;
                    }
                }

                self.number_of_ones = new_number_of_ones;
                self.lower_one = new_lower_one;
                self.upper_one = new_upper_one;
            } else if upper_one < (total_size - 1) {
                self.filter[upper_one] = false;
                self.filter[upper_one + 1] = true;

                let new_lower_one = if lower_one != upper_one {
                    lower_one
                } else {
                    lower_one + 1
                };
                let new_upper_one = upper_one + 1;

                self.number_of_ones = number_of_ones;
                self.lower_one = new_lower_one;
                self.upper_one = new_upper_one;
            } else {
                // we need to find the before to last one in the filter
                let mut zero_index: usize = total_size - 1;
                let mut one_index: usize = total_size - 2;
                let mut ones_before: usize = 2;

                for j in 0..(total_size - 1) {
                    let i = total_size - 2 - j;

                    // count all ones
                    if self.filter[i] {
                        ones_before += 1;
                    }

                    if (!self.filter[i]) && self.filter[i - 1] {
                        zero_index = i;
                        one_index = i - 1;
                        break;
                    }
                }

                // the one index must be set to false
                self.filter[one_index] = false;

                let new_lower_one = zero_index;
                let mut new_upper_one: usize = 0;

                for i in zero_index..total_size {
                    if ones_before > 0 {
                        ones_before -= 1;
                        self.filter[i] = true;

                        if ones_before == 0 {
                            new_upper_one = i;
                        }
                    } else {
                        self.filter[i] = false
                    }
                }

                self.lower_one = new_lower_one;
                self.upper_one = new_upper_one;
            }
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let ones: Vec<u8> = self.filter.iter().map(|x| if *x { 1 } else { 0 }).collect();

        let s = format!(
            "Filter<(l: {}, k: {}, d: {}, u: {}), {:?}>",
            self.length, self.number_of_ones, self.lower_one, self.upper_one, &ones
        );

        write!(f, "{}", s)
    }
}

// this function is not really associated to an specific struct, but to all, thus better
// to make it independent
pub fn compute_aggregation_from_filter(
    aggf: &AggrFn,
    credibility_vector: &[f64],
    filter: &[bool],
) -> f64 {
    if credibility_vector.len() != filter.len() {
        0_f64
    } else {
        let mut condensed: Vec<f64> = Vec::new();

        // collect values in a the condensed vector
        for (index, in_or_not) in filter.iter().enumerate().take(credibility_vector.len()) {
            if *in_or_not {
                condensed.push(credibility_vector[index])
            }
        }

        aggf(condensed)
    }
}

pub fn is_superset(subset: &[usize], superset: &[usize]) -> bool {
    if subset.len() > superset.len() {
        false
    } else {
        for index in subset {
            if !superset.contains(index) {
                return false;
            }
        }

        true
    }
}
