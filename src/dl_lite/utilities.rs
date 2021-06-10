use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::string_formatter::abi_to_string;
use crate::kb::knowledge_base::{ABox, LeveledItem, SymbolDict};
use crate::kb::types::ConflictType;
use petgraph::{Directed, Graph};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Helps ordering size of impliers both in abox items and tbox items.
pub fn ordering_cmp_helper(len1: usize, len2: usize) -> (usize, Ordering) {
    match len1.cmp(&len2) {
        Ordering::Less => (len1, Ordering::Less),
        Ordering::Equal => (len1, Ordering::Equal),
        Ordering::Greater => (len2, Ordering::Greater),
    }
}

pub fn get_max_level_abstract<T: LeveledItem>(items: &[T]) -> usize {
    let mut max_level: usize = 0;

    for item in items {
        max_level = max_level.max(item.level());
    }

    max_level
}

pub fn create_aboxq_graph_dot(
    abq: &AbqDllite,
    symbols: &SymbolDict,
    conflict_matrix: &[i8],
    real_to_virtual: &HashMap<usize, usize>,
    conflict_type: &HashMap<usize, ConflictType>,
    only_conflicts: bool,
) -> Graph<String, bool, Directed, u32> {
    let mut graph: Graph<String, bool> = Graph::new();
    let mut string_op: Option<String>;
    let mut string_node: String;
    let mut abiq: &AbiqDllite;
    let _conft: ConflictType;

    // dict for index
    let mut index_dict = HashMap::new();

    // first add all the nodes
    for i in 0..abq.len() {
        abiq = abq.items().get(i).unwrap();
        let conft = conflict_type.get(&i).unwrap_or(&ConflictType::Conflict);

        match conft {
            ConflictType::SelfConflict => (),
            ConflictType::Clean => {
                if !only_conflicts {
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
                        match w_ij.cmp(&0) {
                            Ordering::Less => {
                                // i is refuted by j
                                graph.add_edge(*index_j, *index_i, false); // an arrow from j to i
                            }
                            Ordering::Greater => {
                                // i is implied by j
                                graph.add_edge(*index_j, *index_i, true); // an arrow from j to i
                            }
                            _ => (),
                        }
                    }
                    (_, _) => (), // passing
                }
            }
        }
    }

    graph
}
