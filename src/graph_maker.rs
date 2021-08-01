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

use crate::dl_lite::string_formatter::{abi_to_string, tbi_to_string};
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;
use crate::interface::format_constants::{
    UNICODE_EXISTS, UNICODE_NEG, UNICODE_SQSUBSETEQ, UNICODE_SUBSETEQ,
};
use crate::kb::knowledge_base::{ABox, Implier, Item, LeveledItem, SymbolDict, TBox, TBoxItem};
use crate::kb::types::{DLType, CR};
use petgraph::graph::EdgeReference;
use petgraph::prelude::NodeIndex;
use petgraph::{Directed, Graph};

use std::collections::HashMap;

use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;

// identifier for the rules
const RULE_IDS: [CR; 10] = [
    CR::First,
    CR::Second,
    CR::Third,
    CR::Fourth,
    CR::Fifth,
    CR::Sixth,
    CR::Seventh,
    CR::Eight,
    CR::Ninth,
    CR::Tenth,
];

const RULE_STR_IDS: [&str; 10] = ["R1", "R2", "R3", "R4", "R5", "R6", "R7", "R8", "R9", "10"];

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
    let is_for_abox = false;

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
                    let tbi_string = transform_tbi_for_graph(tbi, tbi_string, is_for_abox);

                    let impliers = tbi.implied_by();

                    let tbi_index: NodeIndex<u32> =
                        modify_hashmap_graph(tbi_string, &mut added, &mut graph);

                    for (r, v) in impliers {
                        let rule_usize = r.to_usize();

                        // add rule to the graph if it is not in
                        if !rules_added.contains(&rule_usize) {
                            graph.add_node(r.description(for_tbi));
                            rules_added.push(rule_usize);
                        }

                        let rule_string = r.identifier();
                        let index_rule = graph.add_node(rule_string);

                        let _ = graph.add_edge(index_rule, tbi_index, ());

                        for tbi_imp in v {
                            let tbi_string = tbi_to_string(tbi_imp, symbols).unwrap();

                            // here we substitute for the right symbol
                            let tbi_string =
                                transform_tbi_for_graph(tbi_imp, tbi_string, is_for_abox);

                            let imp_index: NodeIndex<u32> =
                                modify_hashmap_graph(tbi_string, &mut added, &mut graph);
                            let _ = graph.add_edge(imp_index, index_rule, ());
                        }
                    }
                }
            }
        }
    }

    graph
}

pub fn create_graph_for_aboxq_unraveling(
    abox: &AbqDllite,
    _tbox: &TBDllite,
    symbols: &SymbolDict,
) -> Graph<String, (), Directed, u32> {
    let mut graph: Graph<String, ()> = Graph::new();

    let max_level = abox.get_max_level();

    let only_conflicts = false;
    // let tbis_by_level = tbox.get_tbis_by_level(only_conflicts);
    let abiqs_by_level = abox.get_abis_by_level(only_conflicts, &[]);
    let mut added: HashMap<String, NodeIndex<u32>> = HashMap::new();

    let mut rules_added: Vec<usize> = Vec::new();
    let for_tbi = false;
    let is_for_abox = true;
    /*
    we wont add all rules for nothing, only rules that are being used
     */

    // I need two types of nodes
    // rond for tbis
    // square for rules
    for lev in 0..(max_level + 1) {
        let actual_level = max_level - lev;
        if abiqs_by_level[actual_level] > 0 {
            for abiq in abox.items() {
                if abiq.level() == actual_level && !abiq.is_trivial() {
                    let abi_string = abi_to_string(abiq.abi(), symbols).unwrap();

                    // here we substitute for the right symbol
                    let abi_string = transform_abiq_for_graph(abiq, abi_string);

                    let impliers = abiq.implied_by();

                    let abi_index: NodeIndex<u32> =
                        modify_hashmap_graph(abi_string, &mut added, &mut graph);

                    for (r, v_tbis, v_abiqs) in impliers {
                        let rule_usize = r.to_usize();

                        // add rule to the graph if it is not in
                        if !rules_added.contains(&rule_usize) {
                            graph.add_node(r.description(for_tbi));
                            rules_added.push(rule_usize);
                        }

                        let rule_string = r.identifier();
                        let index_rule = graph.add_node(rule_string);

                        let _ = graph.add_edge(index_rule, abi_index, ());

                        for tbi_imp in v_tbis {
                            let tbi_string = tbi_to_string(tbi_imp, symbols).unwrap();

                            // here we substitute for the right symbol
                            let tbi_string =
                                transform_tbi_for_graph(tbi_imp, tbi_string, is_for_abox);

                            let imp_index: NodeIndex<u32> =
                                modify_hashmap_graph(tbi_string, &mut added, &mut graph);

                            let _ = graph.add_edge(imp_index, index_rule, ());
                        }

                        for abi_imp in v_abiqs {
                            let abi_string = abi_to_string(abi_imp.abi(), symbols).unwrap();

                            // here we substitute for the right symbol
                            let abi_string = transform_abiq_for_graph(abi_imp, abi_string);

                            let imp_index: NodeIndex<u32> =
                                modify_hashmap_graph(abi_string, &mut added, &mut graph);
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

pub fn node_attr_abox_unraveling(
    _g: &Graph<String, ()>,
    ni: (petgraph::prelude::NodeIndex, &String),
) -> String {
    let s = ni.1;

    for r_str in &RULE_STR_IDS {
        if s.contains(*r_str) {
            return String::from("shape=rectangle color=red");
        }
    }

    // that's for the rules, now for tbox and abox
    let res: String;
    if s.contains("<tbi>") {
        res = String::from("color=blue")
    } else {
        res = String::from("color=green")
    }

    res
}

pub fn node_attr_tbox_unraveling(
    _g: &Graph<String, ()>,
    ni: (petgraph::prelude::NodeIndex, &String),
) -> String {
    let s = ni.1;

    for r_str in &RULE_STR_IDS {
        if s.contains(*r_str) {
            return String::from("shape=rectangle color=red");
        }
    }

    String::from("color=blue")
}

// quantum is used !!!!
pub fn transform_abiq_for_graph(abiq: &AbiqDllite, abiq_string: String) -> String {
    if !cfg!(target_os = "windows") {
        let abiq_string = abiq_string.replace("EXISTS", UNICODE_EXISTS);
        let _abiq_string = abiq_string.replace("NOT", UNICODE_NEG);
    }

    let abiq_string = format!("<abi>LV{}  {}", abiq.level(), &abiq_string);
    abiq_string
}

pub fn transform_tbi_for_graph(tbi: &TbiDllite, tbi_string: String, is_for_abox: bool) -> String {
    let subset = if DLType::all_concepts(tbi.lside().t(), tbi.rside().t()) {
        UNICODE_SQSUBSETEQ
    } else {
        UNICODE_SUBSETEQ
    };

    if !cfg!(target_os = "windows") {
        let tbi_string = tbi_string.replace("EXISTS", UNICODE_EXISTS);
        let tbi_string = tbi_string.replace("<", subset);
        // let tbi_string = tbi_string.replace("-", UNICODE_NEG);
        let _tbi_string = tbi_string.replace("NOT", UNICODE_NEG);
    }

    let tbi_string = match is_for_abox {
        true => format!("<tbi>LV{}  {}", tbi.level(), &tbi_string),
        _ => format!("LV{}  {}", tbi.level(), &tbi_string),
    };

    tbi_string
}

pub fn modify_hashmap_graph(
    abi_string: String,
    added: &mut HashMap<String, NodeIndex<u32>>,
    graph: &mut Graph<String, ()>,
) -> NodeIndex<u32> {
    let abi_index: NodeIndex;
    if added.contains_key(&abi_string) {
        abi_index = *added.get(&abi_string).unwrap();
    } else {
        abi_index = graph.add_node(abi_string.clone());
        added.insert(abi_string, abi_index);
    }

    abi_index
}
