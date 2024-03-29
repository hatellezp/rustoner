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

/*
   helper is what its name says, a helper module, with several wraps for easy of use functions
*/

// =================================================================================================
// IMPORTS

use std::collections::HashMap;
use std::io::ErrorKind;
// error to detect absence of command
use std::ops::DivAssign;
// what is this ???
use std::process::Command;

use petgraph::Graph;
// creation of graphs
use petgraph::graph::EdgeReference;

// to the rankab task, which is rank abox assertion
use crate::alg_math::bounds::find_bound_complex_wrapper;
use crate::alg_math::utilities::{median, solve_system_wrapper_only_id_mod};
// Ontology and ABox (quantified) realizations for dl_lite
use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::ontology::OntologyDllite;
// abstract structs and widely use types
use crate::alg_math::bounds::Adjusters;
use crate::kb::knowledge_base::{ABox, AggrFn};
use crate::kb::types::ConflictType;

// execute a command

// END OF IMPORTS
// =================================================================================================

// the rank abox algorithm returns
type RankRemainder = (Vec<i8>, HashMap<usize, usize>, HashMap<usize, ConflictType>);

pub fn rank_abox(
    onto: &OntologyDllite,
    abq: &mut AbqDllite,
    _deduction_tree: bool,
    aggr: AggrFn,
    adjusters: Adjusters,
    verbose: bool,
    use_concurrency: bool,
) -> RankRemainder {
    // before everything we need to normalize

    // unpack the adjuster
    let (tolerance, m_scale, b_translate) = adjusters;

    let mut prevalues = abq
        .items()
        .iter()
        .map(|x| x.credibility())
        .collect::<Vec<f64>>();
    let normalization_scale = normalize_vector(&mut prevalues);

    for (i, abqi) in abq.items_mut().iter_mut().enumerate().take(prevalues.len()) {
        abqi.set_credibility(prevalues[i]);
    }

    let (before_matrix, real_to_virtual, virtual_to_real) =
        onto.conflict_matrix_refs_only(abq, verbose);

    let (done_matrix, before_to_done_matrix, _done_to_before_matrix, clean_index_tuple_op) =
        OntologyDllite::from_conflict_to_clean_matrix(&before_matrix).unwrap();

    let mut conflict_type: HashMap<usize, ConflictType> = HashMap::new();

    // this will be used independently of emptiness of done_matrix
    let abq_len = abq.len();

    // you can verify here done matrix already
    if done_matrix.is_empty() {
        for value in virtual_to_real.values() {
            // nothing else to do
            if *value < abq_len {
                // verify index out of bounds problems
                abq.get_mut(*value).unwrap().set_value(1.);
            }
        }

        // alg for creating viewer of what is a conflict and what not
        let mut virtual_index_op: Option<usize>;
        let _virtual_index: usize;
        for i in 0..abq_len {
            virtual_index_op = *real_to_virtual.get(&i).unwrap();

            if let Some(some_virtual_index) = virtual_index_op {
                if before_to_done_matrix
                    .get(&some_virtual_index)
                    .unwrap()
                    .is_none()
                {
                    // clean one
                    conflict_type.insert(i, ConflictType::Clean);
                } else {
                    // clean one
                    conflict_type.insert(i, ConflictType::Conflict);
                }
            } else {
                conflict_type.insert(i, ConflictType::SelfConflict);
            }
        }

        // return the conflict listing
        (before_matrix, virtual_to_real, conflict_type)
    } else {
        // the rank can be done on done matrix without differentiating cases
        let aggr_matrix = OntologyDllite::compute_aggregation_matrix(
            abq,
            &done_matrix,
            &virtual_to_real,
            aggr,
            verbose,
        );

        // compute the bound
        let bound_op = find_bound_complex_wrapper(
            aggr_matrix.clone(),
            tolerance,
            m_scale,
            b_translate,
            use_concurrency,
        );

        match bound_op {
            Option::None => (before_matrix, virtual_to_real, conflict_type),
            Some(bound) => {
                let dim = (done_matrix.len() as f64).sqrt() as usize;

                let mut rank: Vec<f64> = vec![0.; dim];

                // let aggr_matrix_clone = aggr_matrix.clone();
                solve_system_wrapper_only_id_mod(&aggr_matrix, &mut rank, bound);

                // now I have the rank, I can begin to put the information inside abq!!

                // we need to upscale in the case that some value is not conflicting, from it we can get the 1 value
                // you can upscale if the clean index is not none, simply as that
                if let Some(some_clean_index) = clean_index_tuple_op {
                    let (new_clean_index, _old_clean_index) = some_clean_index;

                    // get the rank of the clean fact
                    let clean_rank = rank[new_clean_index];
                    rank = rank.iter().map(|x| x / clean_rank).collect::<Vec<f64>>();
                    rank[new_clean_index] = 1.;
                } else {
                    // otherwise we done something else
                    // take the median value of the rank
                    // and normalize for that one
                    // TODO: come here and finish normalization

                    // first see if at least one value is clean inside
                    let mut some_clean_fact: Option<usize> = Option::None;

                    for i in 0..dim {
                        let mut is_clean = true;
                        for j in 0..dim {
                            is_clean = is_clean && (done_matrix[i * dim + j] == 0);
                            if !is_clean {
                                break;
                            }
                        }

                        if is_clean {
                            some_clean_fact = Some(i);
                            break;
                        }
                    }

                    // if we found some fact not implied nor contradict we take
                    if let Some(clean_fact_index) = some_clean_fact {
                        let clean_rank = rank[clean_fact_index];

                        for item in rank.iter_mut().take(dim) {
                            item.div_assign(clean_rank)
                        }

                        rank[clean_fact_index] = 1.;
                    } else {
                        // then all facts have some kind of implication or contradiction
                        let rank_for_median = rank.iter().copied().collect::<Vec<f64>>();
                        let median = median(&rank_for_median).unwrap_or(1.);

                        rank = rank.iter().map(|x| x / median).collect();
                    }
                }

                // now that we have upscale if possible, we put the value in the abox
                for (key, value_op) in &before_to_done_matrix {
                    if value_op.is_none() {
                        let real_index = *(virtual_to_real.get(&key).unwrap());
                        abq.get_mut(real_index).unwrap().set_value(1.);
                    } else {
                        let value = value_op.unwrap();
                        let real_index = *(virtual_to_real.get(&key).unwrap());
                        abq.get_mut(real_index).unwrap().set_value(rank[value]);
                    }
                }

                // once every value is in the abox, we upscale by the normalization factor
                for i in 0..abq.len() {
                    let abqi = abq.get_mut(i).unwrap();
                    let prevalue = abqi.credibility();

                    // TODO: verify why this is necessary, this value should have been updated in the block before
                    let value = abqi.value().unwrap_or(1_f64);

                    abqi.set_credibility(prevalue * normalization_scale);
                    abqi.set_value(value * normalization_scale);
                }

                // alg for creating viewer of what is a conflict and what not
                let mut virtual_index_op: Option<usize>;
                let mut clean_index_done = false;

                for i in 0..abq_len {
                    virtual_index_op = *real_to_virtual.get(&i).unwrap();

                    if let Some(some_virtual_index) = virtual_index_op {
                        if before_to_done_matrix
                            .get(&some_virtual_index)
                            .unwrap()
                            .is_none()
                        {
                            // clean one
                            conflict_type.insert(i, ConflictType::Clean);
                        } else {
                            // clean one
                            if !clean_index_done {
                                if let Some(some_clean_index_tuple) = clean_index_tuple_op {
                                    let (_new_clean_index, new_old_index) = some_clean_index_tuple;

                                    if new_old_index == i {
                                        conflict_type.insert(i, ConflictType::Clean);
                                        clean_index_done = true;
                                    } else {
                                        conflict_type.insert(i, ConflictType::Conflict);
                                    }
                                } else {
                                    conflict_type.insert(i, ConflictType::Conflict);
                                }
                            } else {
                                conflict_type.insert(i, ConflictType::Conflict);
                            }
                        }
                    } else {
                        conflict_type.insert(i, ConflictType::SelfConflict);
                    }
                }

                (before_matrix, virtual_to_real, conflict_type)
            }
        }
    }
}

// function for the creation of graphs
// create functions for att
pub fn edge_attr(_g: &Graph<String, bool>, e: EdgeReference<bool>) -> String {
    if *e.weight() {
        String::from("color=\"green\"")
    } else {
        String::from("color=\"red\"")
    }
}

pub fn node_attr(_g: &Graph<String, bool>, _ni: (petgraph::prelude::NodeIndex, &String)) -> String {
    String::from("")
}

/*
pub fn pretty_print_matrix<T: Display>(v: &[T]) {
    let n = (v.len() as f64).sqrt() as usize;

    for i in 0..n {
        for j in 0..n {
            print!("{}, ", v[n * i + j]);
        }
        println!();
    }
}

 */

pub fn normalize_vector(v: &mut Vec<f64>) -> f64 {
    if !v.is_empty() {
        let initial = v[0];
        let max_value = v.iter().fold(initial, |a, &b| f64::max(a, b));

        for item in v {
            item.div_assign(max_value)
        }

        max_value
    } else {
        1_f64
    }
}

pub fn command_exists(name: &str) -> bool {
    match Command::new(name).spawn() {
        Ok(_) => true,
        Err(e) => !(matches!(e.kind(), ErrorKind::NotFound)),
    }
}
