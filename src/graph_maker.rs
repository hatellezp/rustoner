
use crate::dl_lite::string_formatter::tbi_to_string;
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;
use crate::interface::format_constants::{
    UNICODE_EXISTS, UNICODE_NEG, UNICODE_SQSUBSETEQ, UNICODE_SUBSETEQ,
};
use crate::kb::knowledge_base::{Implier, Item, SymbolDict, TBox, TBoxItem};
use crate::kb::types::{DLType, CR};
use petgraph::graph::EdgeReference;
use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Graph};

use std::collections::HashMap;

// identifier for the rules
const RULE_IDS: [CR; 9] = [
    CR::Zero,
    CR::First,
    CR::Second,
    CR::Third,
    CR::Fourth,
    CR::Fifth,
    CR::Sixth,
    CR::Seventh,
    CR::Eight,
];

const RULE_STR_IDS: [&str; 9] = ["R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8"];

// first function: create graph from tbox with impliers
// in this function true for edges stand for tbi and false stand for rule
pub fn create_graph_for_tbox_unraveling(
    tbox: &TBDllite,
    symbols: &SymbolDict,
) -> Graph<String, (), Directed, u32> {
    let mut graph: Graph<String, ()> = Graph::new();
    let max_level = tbox.get_max_level();

    let only_conflicts = false;
    let tbis_by_level = tbox.get_tbis_by_level(only_conflicts);

    let mut added: HashMap<String, NodeIndex<u32>> = HashMap::new();

    let mut rules_added: Vec<usize> = Vec::new();
    let for_tbi = true;
    /*
    we wont add all rules for nothing, only rules that are being used
    // add all the rules
    let for_tbi = true;
    for item in &RULE_IDS {
        let rule_descrip = item.description(for_tbi);
        graph.add_node(rule_descrip);
    }

     */

    // I need two types of nodes
    // rond for tbis
    // square for rules
    for lev in 0..(max_level + 1) {
        let actual_level = max_level - lev;
        if tbis_by_level[actual_level] > 0 {
            for tbi in tbox.items() {
                if tbi.level() == actual_level && !tbi.is_trivial() {
                    let tbi_string = tbi_to_string(tbi, symbols).unwrap();

                    // here we substitute for the right symbol
                    let tbi_string = transform_tbi_for_graph(tbi, tbi_string);

                    let impliers = tbi.implied_by();

                    let tbi_index: NodeIndex<u32>;
                    if added.contains_key(&tbi_string) {
                        tbi_index = added.get(&tbi_string).unwrap().clone();
                    } else {
                        tbi_index = graph.add_node(tbi_string.clone());
                        added.insert(tbi_string, tbi_index);
                    }

                    for (r, v) in impliers {
                        let rule_usize = r.to_usize();

                        // add rule to the graph if it is not in
                        if !rules_added.contains(&rule_usize) {
                            graph.add_node(r.description(for_tbi));
                            rules_added.push(rule_usize);
                        }

                        let rule_string = format!("{}", r.identifier());
                        let index_rule = graph.add_node(rule_string);

                        let _ = graph.add_edge(index_rule, tbi_index, ());

                        for tbi_imp in v {
                            let tbi_string = tbi_to_string(tbi_imp, symbols).unwrap();

                            // here we substitute for the right symbol
                            let tbi_string = transform_tbi_for_graph(tbi_imp, tbi_string);

                            let imp_index: NodeIndex<u32>;
                            if added.contains_key(&tbi_string) {
                                imp_index = added.get(&tbi_string).unwrap().clone();
                            } else {
                                imp_index = graph.add_node(tbi_string.clone());
                                added.insert(tbi_string, imp_index);
                            }

                            let _ = graph.add_edge(imp_index, index_rule, ());
                        }
                    }
                }
            }
        }
    }

    graph
}

// create graph for abox with impliers

//////
// attributes and nodes formatting functions
// function for the creation of graphs
// create functions for att
pub fn edge_attr_tbox_unraveling(_g: &Graph<String, ()>, _e: EdgeReference<()>) -> String {
    String::from("color=\"black\"")
}

pub fn node_attr_tbox_unraveling(
    _g: &Graph<String, ()>,
    ni: (petgraph::prelude::NodeIndex, &String),
) -> String {
    let s = ni.1;

    for r_str in &RULE_STR_IDS {
        if s.contains(*r_str) {
            return String::from("shape=square color=red");
        }
    }

    String::from("")
}

pub fn transform_tbi_for_graph(tbi: &TbiDllite, tbi_string: String) -> String {
    let subset = if DLType::all_concepts(tbi.lside().t(), tbi.rside().t()) {
        UNICODE_SQSUBSETEQ
    } else {
        UNICODE_SUBSETEQ
    };

    let tbi_string = tbi_string.replace("EXISTS", UNICODE_EXISTS);
    let tbi_string = tbi_string.replace("<", subset);
    // let tbi_string = tbi_string.replace("-", UNICODE_NEG);
    let tbi_string = tbi_string.replace("NOT", UNICODE_NEG);

    let tbi_string = format!("LV{}: {}", tbi.level(), &tbi_string);
    tbi_string
}
