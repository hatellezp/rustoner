use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::ontology::OntologyDllite;

use crate::kb::knowledge_base::{ABox, AggrFn};
use crate::kb::types::ConflictType;

use crate::alg_math::bounds::find_bound_complex_wrapper;
use crate::alg_math::utilities::solve_system_wrapper_only_id_mod;

use petgraph::graph::EdgeReference;
use petgraph::Graph;
use std::collections::HashMap;

pub fn rank_abox(
    onto: &OntologyDllite,
    abq: &mut AbqDllite,
    deduction_tree: bool,
    aggr: AggrFn,
    tolerance: f64,
    m_scaler: f64,
    b_translate: f64,
    verbose: bool,
) -> (Vec<i8>, HashMap<usize, usize>, HashMap<usize, ConflictType>) {
    let (before_matrix, real_to_virtual, virtual_to_real) =
        onto.conflict_matrix(abq, deduction_tree, verbose);

    let (done_matrix, before_to_done_matrix, _done_to_before_matrix, clean_index_tuple_op) =
        OntologyDllite::from_conflict_to_clean_matrix(&before_matrix, verbose).unwrap();

    pretty_print_matrix(&done_matrix);
    println!(
        "from rank abox {:?}\n{:?}\n{:?}",
        &before_to_done_matrix, &_done_to_before_matrix, &clean_index_tuple_op
    );

    let mut conflict_type: HashMap<usize, ConflictType> = HashMap::new();

    // this will be used independently of emptiness of done_matrix
    let abq_len = abq.len();

    // you can verify here done matrix already
    if done_matrix.is_empty() {
        for (_key, value) in &virtual_to_real {
            // nothing else to do
            if *value < abq_len {
                // verify index out of bounds problems
                abq.get_mut(*value).unwrap().set_value(1.);
            }
        }

        // alg for creating viewer of what is a conflict and what not
        let mut virtual_index_op: Option<usize>;
        let mut virtual_index: usize;
        for i in 0..abq_len {
            virtual_index_op = *real_to_virtual.get(&i).unwrap();

            if virtual_index_op.is_none() {
                conflict_type.insert(i, ConflictType::SelfConflict);
            } else {
                virtual_index = virtual_index_op.unwrap();

                if before_to_done_matrix.get(&virtual_index).unwrap().is_none() {
                    // clean one
                    conflict_type.insert(i, ConflictType::Clean);
                } else {
                    // clean one
                    conflict_type.insert(i, ConflictType::Conflict);
                }
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
        let bound_op =
            find_bound_complex_wrapper(aggr_matrix.clone(), tolerance, m_scaler, b_translate);

        match bound_op {
            Option::None => (before_matrix, virtual_to_real, conflict_type),
            Some(bound) => {
                let dim = (done_matrix.len() as f64).sqrt() as usize;

                let mut rank: Vec<f64> = vec![0.; dim];

                solve_system_wrapper_only_id_mod(aggr_matrix, &mut rank, bound);

                // now I have the rank, I can begin to put the information inside abq!!

                // we need to upscale in the case that some value is not conflicting, from it we can get the 1 value
                // you can upscale if the clean index is not none, simply as that
                if clean_index_tuple_op.is_some() {
                    let (new_clean_index, _old_clean_index) = clean_index_tuple_op.unwrap();

                    // get the rank of the clean fact
                    let clean_rank = rank[new_clean_index];
                    rank = rank
                        .iter()
                        .map(|x| x * (1. / clean_rank))
                        .collect::<Vec<f64>>();
                    rank[new_clean_index] = 1.;
                } else {
                    // otherwise we done something else
                    // take the median value of the rank
                    // and normalize for that one
                    ()
                }

                // now that we have upscale if possible, we put the value in the abox
                let mut real_index: usize;
                let mut value: usize;
                for (key, value_op) in &before_to_done_matrix {
                    if value_op.is_none() {
                        real_index = *(virtual_to_real.get(&key).unwrap());
                        abq.get_mut(real_index).unwrap().set_value(1.);
                    } else {
                        value = value_op.unwrap();
                        real_index = *(virtual_to_real.get(&key).unwrap());
                        abq.get_mut(real_index).unwrap().set_value(rank[value]);
                    }
                }

                // alg for creating viewer of what is a conflict and what not
                let mut virtual_index_op: Option<usize>;
                let mut virtual_index: usize;
                let mut clean_index_done = false;

                for i in 0..abq_len {
                    virtual_index_op = *real_to_virtual.get(&i).unwrap();

                    if virtual_index_op.is_none() {
                        conflict_type.insert(i, ConflictType::SelfConflict);
                    } else {
                        virtual_index = virtual_index_op.unwrap();

                        if before_to_done_matrix.get(&virtual_index).unwrap().is_none() {
                            // clean one
                            conflict_type.insert(i, ConflictType::Clean);
                        } else {
                            // clean one
                            if !clean_index_done {
                                if clean_index_tuple_op.is_some() {
                                    let (_new_clean_index, new_old_index) =
                                        clean_index_tuple_op.unwrap();

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
        String::from("color=\"blue\"")
    } else {
        String::from("color=\"red\"")
    }
}

pub fn node_attr(_g: &Graph<String, bool>, _ni: (petgraph::prelude::NodeIndex, &String)) -> String {
    String::from("")
}

pub fn pretty_print_matrix(v: &Vec<i8>) {
    let n = (v.len() as f64).sqrt() as usize;

    for i in 0..n {
        for j in 0..n {
            print!("{}, ", v[n * i + j]);
        }
        println!();
    }
}
