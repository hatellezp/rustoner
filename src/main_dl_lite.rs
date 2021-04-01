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

mod alg_math;
mod dl_lite;
mod graph_maker;
mod helper;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

// from alg_math

// from kb
use crate::kb::knowledge_base::{ABox, SymbolDict, TBox, TBoxItem};
use crate::kb::types::FileType;

// from the dl_lite module
use crate::dl_lite::ontology::OntologyDllite;

use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::abox_item_quantum::AbiqDllite;
use crate::dl_lite::native_filetype_utilities::{
    abox_to_native_string_quantum, tbox_to_native_string,
};
use crate::dl_lite::string_formatter::{
    create_string_for_gencontb, create_string_for_unravel_conflict_abox,
    create_string_for_unravel_conflict_tbox, pretty_print_abiq_conflict, tbi_to_string,
};
use crate::dl_lite::tbox::TBDllite;
use crate::dl_lite::tbox_item::TbiDllite;

use crate::kb::aggr_functions::{AGGR_COUNT, AGGR_MAX, AGGR_MEAN, AGGR_MIN, AGGR_SUM};

// from the interface module
use crate::interface::cli::{AggrName, Cli, Task};

use crate::interface::utilities::{get_filetype, parse_name_from_filename, write_str_to_file};

// to ask basic questions
use crate::helper::rank_abox;
use crate::helper::{command_exists, edge_attr, node_attr};
use petgraph::dot::{Config, Dot};

use question::{Answer, Question};

use crate::alg_math::utilities::null_vector;
use crate::graph_maker::{create_graph_for_tbox_unraveling, edge_attr_tbox_unraveling, node_attr_tbox_unraveling, create_graph_for_aboxq_unraveling, node_attr_abox_unraveling};
use std::process::Command;
use tempfile::NamedTempFile;

// constants for the bound computing
const TOLERANCE: f64 = 0.0000000000000001;
const M_SCALER: f64 = 1.1;
const B_TRANSLATE: f64 = 1.;
const DOT_COMMAND_LINUX: &str = "dot";
const DOT_COMMAND_WINDOWS: &str = "dot.exe";

// the main function
pub fn main() {
    let args = Cli::from_args();

    // get all arguments regardless of the task
    let task: Task = args.task;
    // let path_db_op: Option<std::path::PathBuf> = args.path_db;
    let path_tbox_op: Option<std::path::PathBuf> = args.path_tbox;
    let path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;
    let silent: bool = args.silent;
    let aggr_name_op: Option<AggrName> = args.aggr;
    // let _ephemere: bool = args.ephemere;

    // ---------------------------------
    // this are variables used in several places

    // to avoid trivial tbi written to file
    let _dont_write_trivial = true;

    // whenever we need a pretty string
    let mut pretty_string: String;

    // moving tb
    let mut new_tb: TBDllite;

    // symbols:
    let symbols: &SymbolDict;

    // for the deduction tree
    let mut deduction_tree = false;

    let path_tbox: String;
    let tb_ft: FileType;
    let tb_name: &str;
    let mut onto: OntologyDllite;
    let symbols_ft: FileType;
    let only_conflicts: bool;
    let dont_write_trivial: bool;

    // now do what you are ask
    match task {
        Task::VERTB | Task::GENCONTB | Task::CTB => {
            if let Some(some_path_tbox) = path_tbox_op {
                // get the information from the file: name and bla bla
                path_tbox = some_path_tbox.to_str().unwrap().to_string();
                tb_ft = get_filetype(&path_tbox);
                tb_name = parse_name_from_filename(&path_tbox);

                // create a temporal ontology
                onto = OntologyDllite::new(String::from(tb_name));

                // get symbols from this file if possible
                match path_symbols_op {
                    Some(path_symbols) => {
                        let path_symbols = path_symbols.to_str().unwrap();
                        symbols_ft = get_filetype(path_symbols);

                        onto.add_symbols_from_file(path_symbols, symbols_ft, args.verbose);
                    }
                    Option::None => {
                        // else add from the tbox file
                        onto.add_symbols_from_file(&path_tbox, tb_ft, args.verbose);
                    }
                }

                // now add tbis from the tbox file
                onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);

                // here we bound symbols
                symbols = onto.symbols();

                match task {
                    Task::VERTB => {
                        deduction_tree = false;
                        new_tb = onto.complete_tbox(deduction_tree, verbose);

                        let mut contradictions: Vec<&TbiDllite> = Vec::new();

                        for tbi in new_tb.items() {
                            if tbi.is_contradiction() && !tbi.is_trivial() {
                                contradictions.push(tbi);
                            }
                        }

                        match contradictions.len() {
                            0 => {
                                println!(
                                    " -- no contradictions nor possible contradictions were found"
                                );
                                std::process::exit(exitcode::OK);
                            }
                            _ => {
                                println!(" -- possible contradictions were found");

                                if !silent {
                                    // show contradictions
                                    let question_print =
                                        " -- do you want to see the contradictions?".to_string();

                                    let print_output = Question::new(&question_print)
                                        .default(Answer::YES)
                                        .show_defaults()
                                        .confirm();

                                    if print_output == Answer::YES {
                                        let mut current_tbi_op: Option<String>;

                                        println!("{{");

                                        for tbi in contradictions {
                                            current_tbi_op = tbi_to_string(tbi, symbols);

                                            if current_tbi_op.is_some() {
                                                println!("  {},", &(current_tbi_op.unwrap()));
                                            } else {
                                                println!("  passing,");
                                            }
                                        }

                                        println!("}}");
                                    }
                                }

                                // now ask if they want to see the tree for the contradictions
                                let question_print =
                                    " -- do you want unravel for conflicts?".to_string();

                                let print_output = Question::new(&question_print)
                                    .default(Answer::YES)
                                    .show_defaults()
                                    .confirm();

                                if print_output == Answer::YES {
                                    deduction_tree = true;
                                    new_tb = onto.complete_tbox(deduction_tree, verbose);

                                    only_conflicts = true;
                                    pretty_string = create_string_for_unravel_conflict_tbox(
                                        &new_tb,
                                        symbols,
                                        only_conflicts,
                                    );

                                    if !silent {
                                        println!("{}", &pretty_string);
                                    }

                                    // here to dot notation and graph stuff
                                    let question_print =
                                        " -- do you want to create a deduction graph by dot notation?".to_string();

                                    let print_output = Question::new(&question_print)
                                        .default(Answer::YES)
                                        .show_defaults()
                                        .confirm();

                                    if print_output == Answer::YES {
                                        // TODO: I'm here
                                        let graph = create_graph_for_tbox_unraveling(
                                            &new_tb,
                                            onto.symbols(),
                                        );

                                        let get_edge = edge_attr_tbox_unraveling;
                                        let get_node = node_attr_tbox_unraveling;

                                        let dot_notation = Dot::with_attr_getters(
                                            &graph,
                                            &[Config::EdgeNoLabel],
                                            &get_edge,
                                            &get_node,
                                        );

                                        let dot_notation_output = format!("{:?}", dot_notation);

                                        let filename = format!("{}_consequences.dot", tb_name);
                                        write_str_to_file(&dot_notation_output, &filename);

                                        if !silent {
                                            println!(" -- dot file created: {}", &filename);
                                        }

                                        //create graph here
                                        // here verify that the command exists
                                        let dot_command_name = if cfg!(windows) {
                                            DOT_COMMAND_WINDOWS
                                        } else {
                                            DOT_COMMAND_LINUX
                                        };
                                        let dot_exists = command_exists(dot_command_name);

                                        if !dot_exists {
                                            println!(
                                                " -- {} not found, can't generate visual output",
                                                dot_command_name
                                            );
                                            std::process::exit(exitcode::OK)
                                        }

                                        // now show graph
                                        let question_print =
                                            " -- do you want see a generate a visual output?"
                                                .to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            // here create a temporary file
                                            let temp_dot_file_res = NamedTempFile::new();

                                            match temp_dot_file_res {
                                                Err(e) => {
                                                    if !silent {
                                                        println!(
                                                            "could not generate output: {}",
                                                            e
                                                        );
                                                    }
                                                }
                                                Ok(temp_dot) => {
                                                    let path_to_temp_dot =
                                                        (&temp_dot).path().to_str();

                                                    match path_to_temp_dot {
                                                        Option::None => {
                                                            println!(
                                                                "path is not valid: {:?}",
                                                                &path_to_temp_dot
                                                            );
                                                        }
                                                        Some(path_to_temp) => {
                                                            // write to temporary file
                                                            write_str_to_file(
                                                                &dot_notation_output,
                                                                path_to_temp,
                                                            );

                                                            let dot_output_format = "pdf";

                                                            let name_output_file = format!(
                                                                "{}_consequences.{}",
                                                                tb_name, dot_output_format
                                                            );
                                                            let command = format!(
                                                                "dot -T{} {} -o {}",
                                                                dot_output_format,
                                                                path_to_temp,
                                                                &name_output_file
                                                            );

                                                            // execute dot command
                                                            // TODO: change this to be platform independent
                                                            let output = Command::new("sh")
                                                                .arg("-c")
                                                                .arg(&command)
                                                                .output();

                                                            match output {
                                                                Err(e) => {
                                                                    println!(
                                                                        "couldn't create output: {}",
                                                                        &e
                                                                    );
                                                                }
                                                                Ok(o) => {
                                                                    if !silent {
                                                                        let _std_out =
                                                                            std::str::from_utf8(
                                                                                &o.stdout,
                                                                            )
                                                                            .unwrap();
                                                                        println!(
                                                                            " -- file generated: {}",
                                                                            &name_output_file
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // don't forget to copy to file if output is specified

                                    match path_output_op {
                                        Some(path_output) => {
                                            let filename =
                                                path_output.to_str().unwrap().to_string();
                                            write_str_to_file(&pretty_string, &filename);

                                            std::process::exit(exitcode::OK);
                                        }
                                        Option::None => std::process::exit(exitcode::OK),
                                    }
                                }
                            }
                        }
                    }
                    Task::GENCONTB => {
                        // complete by deduction
                        deduction_tree = true;
                        dont_write_trivial = true;
                        new_tb = onto.complete_tbox(deduction_tree, verbose);
                        pretty_string = create_string_for_gencontb(
                            &new_tb,
                            symbols,
                            dont_write_trivial,
                            verbose,
                        );

                        if !silent {
                            println!("{}", &pretty_string);
                        }

                        // here we put the generation of the graph
                        // here to dot notation and graph stuff
                        let question_print =
                            " -- do you want to create a deduction graph by dot notation?"
                                .to_string();

                        let print_output = Question::new(&question_print)
                            .default(Answer::YES)
                            .show_defaults()
                            .confirm();

                        if print_output == Answer::YES {
                            // TODO: I'm here
                            let graph = create_graph_for_tbox_unraveling(&new_tb, onto.symbols());

                            let get_edge = edge_attr_tbox_unraveling;
                            let get_node = node_attr_tbox_unraveling;

                            let dot_notation = Dot::with_attr_getters(
                                &graph,
                                &[Config::EdgeNoLabel],
                                &get_edge,
                                &get_node,
                            );

                            let dot_notation_output = format!("{:?}", dot_notation);

                            let filename = format!("{}_consequences.dot", tb_name);
                            write_str_to_file(&dot_notation_output, &filename);

                            if !silent {
                                println!(" -- dot file created: {}", &filename);
                            }

                            //create graph here
                            // here verify that the command exists
                            let dot_command_name = if cfg!(windows) {
                                DOT_COMMAND_WINDOWS
                            } else {
                                DOT_COMMAND_LINUX
                            };
                            let dot_exists = command_exists(dot_command_name);

                            if !dot_exists {
                                println!(
                                    " -- {} not found, can't generate visual output",
                                    dot_command_name
                                );
                                std::process::exit(exitcode::OK)
                            }

                            // now show graph
                            let question_print =
                                " -- do you want see a generate a visual output?".to_string();

                            let print_output = Question::new(&question_print)
                                .default(Answer::YES)
                                .show_defaults()
                                .confirm();

                            if print_output == Answer::YES {
                                // here create a temporary file
                                let temp_dot_file_res = NamedTempFile::new();

                                match temp_dot_file_res {
                                    Err(e) => {
                                        if !silent {
                                            println!("could not generate output: {}", e);
                                        }
                                    }
                                    Ok(temp_dot) => {
                                        let path_to_temp_dot = (&temp_dot).path().to_str();

                                        match path_to_temp_dot {
                                            Option::None => {
                                                println!(
                                                    "path is not valid: {:?}",
                                                    &path_to_temp_dot
                                                );
                                            }
                                            Some(path_to_temp) => {
                                                // write to temporary file
                                                write_str_to_file(
                                                    &dot_notation_output,
                                                    path_to_temp,
                                                );

                                                let dot_output_format = "pdf";

                                                let name_output_file = format!(
                                                    "{}_consequences.{}",
                                                    tb_name, dot_output_format
                                                );
                                                let command = format!(
                                                    "dot -T{} {} -o {}",
                                                    dot_output_format,
                                                    path_to_temp,
                                                    &name_output_file
                                                );

                                                // execute dot command
                                                // TODO: change this to be platform independent
                                                let output = Command::new("sh")
                                                    .arg("-c")
                                                    .arg(&command)
                                                    .output();

                                                match output {
                                                    Err(e) => {
                                                        println!("couldn't create output: {}", &e);
                                                    }
                                                    Ok(o) => {
                                                        if !silent {
                                                            let _std_out =
                                                                std::str::from_utf8(&o.stdout)
                                                                    .unwrap();
                                                            println!(
                                                                " -- file generated: {}",
                                                                &name_output_file
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // consequences to file if presented
                        match path_output_op {
                            Some(path_output) => {
                                let filename = path_output.to_str().unwrap().to_string();

                                write_str_to_file(&pretty_string, &filename);
                            }
                            Option::None => (),
                        }
                    }
                    Task::CTB => {
                        // deduction tree is activated only in generate consequence tree mode
                        deduction_tree = false;
                        dont_write_trivial = true;

                        new_tb = onto.complete_tbox(deduction_tree, verbose);

                        if !silent {
                            let question_print = " -- do you want to see the output?".to_string();
                            let print_output = Question::new(&question_print)
                                .default(Answer::YES)
                                .show_defaults()
                                .confirm();

                            if print_output == Answer::YES {
                                let new_tb_as_string =
                                    onto.tbox_to_string(&new_tb, dont_write_trivial);

                                println!(" -- <TBox>\n{}", &new_tb_as_string);
                            }
                        }

                        match path_output_op {
                            Some(path_output) => {
                                let new_tb_as_string_op =
                                    tbox_to_native_string(&new_tb, symbols, dont_write_trivial);

                                match new_tb_as_string_op {
                                    Option::None => println!("ERROR: couldn't create output, maybe run with 'verbose' to see more"),
                                    Some(new_tb_as_string) => {
                                        let filename = path_output.to_str().unwrap().to_string();

                                        write_str_to_file(&new_tb_as_string, &filename);
                                    },
                                }
                            }
                            Option::None => (),
                        }
                    }
                    _ => println!("impossible to be here"),
                }
            } else {
                println!("ERROR: you must provide a tbox file");
                std::process::exit(exitcode::USAGE);
            }
        }
        Task::VERAB | Task::CLEANAB | Task::CAB | Task::GENCONAB | Task::RNKAB => {
            // some common stuff
            match (path_tbox_op, path_abox_op) {
                (Some(some_tbox_path), Some(some_abox_path)) => {
                    println!(" -- be sure to have an abox without self conflicting facts before going further, you can use the 'cleanab' task for this");

                    // get information for the tbox
                    let path_tbox = some_tbox_path.to_str().unwrap().to_string();
                    let onto_name = parse_name_from_filename(&path_tbox);
                    let tb_filetype = get_filetype(&path_tbox);

                    let mut onto = OntologyDllite::new(onto_name.to_string());

                    // add symbols from where you can
                    if let Some(some_symbols_path) = path_symbols_op {
                        let path_symbols = some_symbols_path.to_str().unwrap().to_string();
                        let symbols_filetype = get_filetype(&path_symbols);

                        onto.add_symbols_from_file(&path_symbols, symbols_filetype, verbose);
                    } else {
                        onto.add_symbols_from_file(&path_tbox, tb_filetype, verbose);
                    }

                    // add tbis
                    onto.add_tbis_from_file(&path_tbox, tb_filetype, verbose);

                    // path and name of the abox
                    let path_abox = some_abox_path.to_str().unwrap().to_string();
                    let ab_name = parse_name_from_filename(&path_abox).trim().to_string();
                    let ab_ft = get_filetype(&path_abox);

                    // reserved name
                    if ab_name == "temp_abox" {
                        println!("ERROR: the name 'temp_abox' is reserved, please use another one");
                        std::process::exit(exitcode::USAGE);
                    }

                    // add abox
                    onto.new_abox_from_file_quantum(&path_abox, ab_ft, verbose);

                    // we continue with the completion
                    // before completion of abox you need to autocomplete the tbox in onto
                    deduction_tree = true;
                    new_tb = onto.complete_tbox(false, verbose); // I put false here because I only need deduction for abox
                    onto.add_tbis_from_vec(new_tb.items()); // add what you don't have

                    let abox_completed_op = onto.complete_abox(deduction_tree, verbose);

                    symbols = onto.symbols();

                    match abox_completed_op {
                        Some(abox_completed) => {
                            match task {
                                Task::VERAB => {
                                    //change current abox
                                    let contradictions: Vec<(TbiDllite, Vec<AbiqDllite>)> =
                                        abox_completed
                                            .is_inconsistent_detailed(onto.tbox(), verbose);
                                    // let is_abox_consistent = abox_completed.is_inconsistent(onto.tbox(), verbose);

                                    match contradictions.len() {
                                        0 => println!(" -- no contradictions  were found"),
                                        _ => {
                                            println!(" -- contradictions were found");

                                            if !silent {
                                                // show contradictions
                                                let question_print =
                                                    " -- do you want to see them".to_string();

                                                let print_output = Question::new(&question_print)
                                                    .default(Answer::YES)
                                                    .show_defaults()
                                                    .confirm();

                                                if print_output == Answer::YES {
                                                    let _current_tbi_op: Option<String>;

                                                    println!("[");

                                                    for tuple in contradictions.iter() {
                                                        let container = tuple.clone();

                                                        let v_abiq_s = pretty_print_abiq_conflict(
                                                            (&container.0, &container.1),
                                                            symbols,
                                                        );

                                                        println!("{}", &v_abiq_s);
                                                    }

                                                    println!("]");
                                                }
                                            }

                                            // now unravel for conflicts
                                            let question_print =
                                                " -- do you want unravel for conflicts?"
                                                    .to_string();

                                            let print_output = Question::new(&question_print)
                                                .default(Answer::YES)
                                                .show_defaults()
                                                .confirm();

                                            if print_output == Answer::YES {
                                                only_conflicts = true;
                                                pretty_string =
                                                    create_string_for_unravel_conflict_abox(
                                                        &new_tb,
                                                        &abox_completed,
                                                        symbols,
                                                        only_conflicts,
                                                        &contradictions,
                                                    );

                                                if !silent {
                                                    println!("{}", &pretty_string);
                                                    println!(" -- if you see that one sole element might be sprouting conflicts, use the 'cleanab' task to clean the abox from self conflicting facts");
                                                }

                                                // don't forget to copy to file if output is specified

                                                match path_output_op {
                                                    Some(path_output) => {
                                                        let filename = path_output
                                                            .to_str()
                                                            .unwrap()
                                                            .to_string();
                                                        write_str_to_file(
                                                            &pretty_string,
                                                            &filename,
                                                        );
                                                    }
                                                    Option::None => (),
                                                }
                                            }
                                        }
                                    }
                                }
                                Task::CLEANAB => {
                                    let clean_name = format!("{}_clean", &ab_name);
                                    let dirty_name = format!("{}_dirty", &ab_name);
                                    let mut clean_ab = AbqDllite::new(&clean_name);
                                    let mut dirty_ab = AbqDllite::new(&dirty_name);

                                    let orig_ab_op = onto.abox();

                                    match orig_ab_op {
                                        Option::None => {
                                            println!("ERROR: the original abox is not present, maybe run with 'verbose' for more information");
                                            std::process::exit(exitcode::DATAERR);
                                        }
                                        Some(orig_ab) => {
                                            let mut is_self_conflict: bool;

                                            // remember that onto has the completed tbox
                                            for abiq in orig_ab.items() {
                                                is_self_conflict =
                                                    AbqDllite::abiq_is_self_contradicting(
                                                        abiq,
                                                        onto.tbox(),
                                                    );

                                                if is_self_conflict {
                                                    dirty_ab.add(abiq.clone());
                                                } else {
                                                    clean_ab.add(abiq.clone());
                                                }
                                            }

                                            let clean_is_empty = clean_ab.is_empty();
                                            let dirty_is_empty = dirty_ab.is_empty();

                                            if !silent {
                                                if !clean_is_empty {
                                                    println!(" -- clean abox:");
                                                    pretty_string =
                                                        onto.abox_to_string_quantum(&clean_ab);
                                                    println!("{}", &pretty_string);
                                                } else {
                                                    println!(
                                                        " -- all elements seems to be self conflicting"
                                                    );
                                                }

                                                if !dirty_is_empty {
                                                    println!(" -- dirty abox:");
                                                    pretty_string =
                                                        onto.abox_to_string_quantum(&dirty_ab);
                                                    println!("{}", &pretty_string);
                                                } else {
                                                    println!(" -- seems that no self conflicting facts where found");
                                                }
                                            }

                                            // now write to file
                                            let clean_name = clean_ab.name();
                                            let dirty_name = dirty_ab.name();
                                            dont_write_trivial = true;

                                            // write to both files
                                            if !clean_is_empty {
                                                let clean_output_ab = abox_to_native_string_quantum(
                                                    &clean_ab,
                                                    onto.symbols(),
                                                    dont_write_trivial,
                                                );

                                                match clean_output_ab {
                                                    Option::None => {
                                                        if !silent {
                                                            println!(
                                                                " -- couldn't create clean abox!"
                                                            );
                                                        }
                                                    }
                                                    Some(output_ab) => {
                                                        write_str_to_file(&output_ab, &clean_name);

                                                        if !silent {
                                                            println!(
                                                                " -- wrote clean abox to {}",
                                                                &clean_name
                                                            );
                                                        }
                                                    }
                                                }
                                            } else if !silent {
                                                println!(" -- no clean abox was written, as there is nothing to wrote");
                                            }

                                            if !dirty_is_empty {
                                                let dirty_output_ab = abox_to_native_string_quantum(
                                                    &dirty_ab,
                                                    onto.symbols(),
                                                    dont_write_trivial,
                                                );

                                                match dirty_output_ab {
                                                    Option::None => {
                                                        if !silent {
                                                            println!(
                                                                " -- couldn't create dirty abox!"
                                                            );
                                                        }
                                                    }
                                                    Some(output_ab) => {
                                                        write_str_to_file(&output_ab, &dirty_name);

                                                        if !silent {
                                                            println!(
                                                                " -- wrote clean abox to {}",
                                                                &dirty_name
                                                            );
                                                        }
                                                    }
                                                }
                                            } else if !silent {
                                                println!(
                                                    " -- no dirty element found to be written"
                                                );
                                            }
                                        }
                                    }
                                }
                                Task::CAB => {
                                    let abox_completed_string =
                                        onto.abox_to_string_quantum(&abox_completed);

                                    if !silent {
                                        println!(" -- abox:\n");
                                        println!("{}", &abox_completed_string);
                                    }

                                    match path_output_op {
                                        Some(path_output) => {
                                            let filename =
                                                path_output.to_str().unwrap().to_string();
                                            dont_write_trivial = true;

                                            // add the abis
                                            onto.add_abis_from_abox(&abox_completed);
                                            onto.abox_to_file(
                                                &filename,
                                                FileType::NATIVE,
                                                dont_write_trivial,
                                            );

                                            if !silent {
                                                println!(" -- abox written to {}", &filename);
                                            }
                                        }
                                        Option::None => (),
                                    }
                                }
                                Task::GENCONAB => {
                                    let only_conflicts = false;
                                    let tb_output = create_string_for_unravel_conflict_tbox(&new_tb, onto.symbols(), only_conflicts);
                                    let ab_output = create_string_for_unravel_conflict_abox(&new_tb, &abox_completed, onto.symbols(), only_conflicts, &[]);

                                    if !silent {
                                        println!("{}", ab_output);
                                    }

                                    // here we put the generation of the graph
                                    // here to dot notation and graph stuff
                                    let question_print =
                                        " -- do you want to create a deduction graph by dot notation?"
                                            .to_string();

                                    let print_output = Question::new(&question_print)
                                        .default(Answer::YES)
                                        .show_defaults()
                                        .confirm();

                                    if print_output == Answer::YES {
                                        // TODO: I'm here
                                        let graph = create_graph_for_aboxq_unraveling(&abox_completed, &new_tb, onto.symbols());

                                        let get_edge = edge_attr_tbox_unraveling;
                                        let get_node = node_attr_abox_unraveling;

                                        let dot_notation = Dot::with_attr_getters(
                                            &graph,
                                            &[Config::EdgeNoLabel],
                                            &get_edge,
                                            &get_node,
                                        );

                                        let dot_notation_output = format!("{:?}", dot_notation);

                                        let filename = format!("{}_consequences.dot", &ab_name);
                                        write_str_to_file(&dot_notation_output, &filename);

                                        if !silent {
                                            println!(" -- dot file created: {}", &filename);
                                        }

                                        //create graph here
                                        // here verify that the command exists
                                        let dot_command_name = if cfg!(windows) {
                                            DOT_COMMAND_WINDOWS
                                        } else {
                                            DOT_COMMAND_LINUX
                                        };
                                        let dot_exists = command_exists(dot_command_name);

                                        if !dot_exists {
                                            println!(
                                                " -- {} not found, can't generate visual output",
                                                dot_command_name
                                            );
                                            std::process::exit(exitcode::OK)
                                        }

                                        // now show graph
                                        let question_print =
                                            " -- do you want see a generate a visual output?".to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            // here create a temporary file
                                            let temp_dot_file_res = NamedTempFile::new();

                                            match temp_dot_file_res {
                                                Err(e) => {
                                                    if !silent {
                                                        println!("could not generate output: {}", e);
                                                    }
                                                }
                                                Ok(temp_dot) => {
                                                    let path_to_temp_dot = (&temp_dot).path().to_str();

                                                    match path_to_temp_dot {
                                                        Option::None => {
                                                            println!(
                                                                "path is not valid: {:?}",
                                                                &path_to_temp_dot
                                                            );
                                                        }
                                                        Some(path_to_temp) => {
                                                            // write to temporary file
                                                            write_str_to_file(
                                                                &dot_notation_output,
                                                                path_to_temp,
                                                            );

                                                            let dot_output_format = "pdf";

                                                            let name_output_file = format!(
                                                                "{}_consequences.{}",
                                                                ab_name, dot_output_format
                                                            );
                                                            let command = format!(
                                                                "dot -T{} {} -o {}",
                                                                dot_output_format,
                                                                path_to_temp,
                                                                &name_output_file
                                                            );

                                                            // execute dot command
                                                            // TODO: change this to be platform independent
                                                            let output = Command::new("sh")
                                                                .arg("-c")
                                                                .arg(&command)
                                                                .output();

                                                            match output {
                                                                Err(e) => {
                                                                    println!("couldn't create output: {}", &e);
                                                                }
                                                                Ok(o) => {
                                                                    if !silent {
                                                                        let _std_out =
                                                                            std::str::from_utf8(&o.stdout)
                                                                                .unwrap();
                                                                        println!(
                                                                            " -- file generated: {}",
                                                                            &name_output_file
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // consequences to file if presented
                                    match path_output_op {
                                        Some(path_output) => {
                                            let filename = path_output.to_str().unwrap().to_string();

                                            write_str_to_file(&ab_output, &filename);
                                        }
                                        Option::None => (),
                                    }
                                }
                                Task::RNKAB => {
                                    // the current abox is not the completed one
                                    let mut abox = onto.abox().unwrap().clone();
                                    deduction_tree = false;

                                    // find aggregation function
                                    let aggr = match aggr_name_op {
                                        Option::None => AGGR_SUM,
                                        Some(aggr_name) => match aggr_name {
                                            AggrName::UNDEFINED => AGGR_SUM,
                                            AggrName::SUM => AGGR_SUM,
                                            AggrName::MAX => AGGR_MAX,
                                            AggrName::MIN => AGGR_MIN,
                                            AggrName::MEAN => AGGR_MEAN,
                                            AggrName::COUNT => AGGR_COUNT,
                                        },
                                    };

                                    let (before_matrix, virtual_to_real, conflict_type) = rank_abox(
                                        &onto,
                                        &mut abox,
                                        deduction_tree,
                                        aggr,
                                        TOLERANCE,
                                        M_SCALER,
                                        B_TRANSLATE,
                                        verbose,
                                    );
                                    // now the abox is ranked

                                    if !silent {
                                        let question_print =
                                            " -- do you want to see the output?".to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            let abox_string = onto.abox_to_string_quantum(&abox);
                                            println!("{}", &abox_string);
                                        }
                                    }

                                    // save to file the new abox
                                    match path_output_op {
                                        Some(path_output) => {
                                            let filename =
                                                path_output.to_str().unwrap().to_string();
                                            dont_write_trivial = true;

                                            // add the abis
                                            let abox_ranked_string_op =
                                                abox_to_native_string_quantum(
                                                    &abox,
                                                    onto.symbols(),
                                                    dont_write_trivial,
                                                );

                                            match abox_ranked_string_op {
                                                Option::None => {
                                                    println!(
                                                        "  -- couldn't create an output string!"
                                                    );
                                                }
                                                Some(ranked_string) => {
                                                    let wrote = write_str_to_file(
                                                        &ranked_string,
                                                        &filename,
                                                    );
                                                    if wrote && !silent {
                                                        println!(
                                                            " -- ranked abox written to {}",
                                                            &filename
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        Option::None => (),
                                    }

                                    // now create graph if necessary
                                    let question_print =
                                        " -- do you want to create a conflict graph?".to_string();

                                    let print_output = Question::new(&question_print)
                                        .default(Answer::YES)
                                        .show_defaults()
                                        .confirm();

                                    if print_output == Answer::YES {
                                        if null_vector(&before_matrix) {
                                            println!(" -- no conflicts where found, the conflict graph will be empty, passing");
                                            std::process::exit(exitcode::OK)
                                        }

                                        // here ask later for only conflicts or not
                                        let only_conflicts_graph = true;

                                        let graph = abox.create_graph_dot(
                                            onto.symbols(),
                                            &before_matrix,
                                            &virtual_to_real,
                                            &conflict_type,
                                            only_conflicts_graph,
                                        );

                                        if !silent {
                                            println!(" -- conflict graph created");
                                        }

                                        let get_edge = edge_attr;
                                        let get_node = node_attr;

                                        let dot_notation = Dot::with_attr_getters(
                                            &graph,
                                            &[Config::EdgeNoLabel],
                                            &get_edge,
                                            &get_node,
                                        );

                                        let output = format!("{}", dot_notation);

                                        // two things: first save dot notation, second save graph to pdf
                                        let question_print =
                                            " -- do you want to save to dot notation?".to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            let filename =
                                                format!("{}_conflict_graph.dot", abox.name());
                                            write_str_to_file(&output, &filename);

                                            if !silent {
                                                println!(" -- dot file created: {}", &filename);
                                            }
                                        }

                                        // here verify that the command exists
                                        let dot_command_name = if cfg!(windows) {
                                            DOT_COMMAND_WINDOWS
                                        } else {
                                            DOT_COMMAND_LINUX
                                        };
                                        let dot_exists = command_exists(dot_command_name);

                                        if !dot_exists {
                                            println!(
                                                " -- {} not found, can't generate visual output",
                                                dot_command_name
                                            );
                                            std::process::exit(exitcode::OK)
                                        }

                                        // now show graph
                                        let question_print =
                                            " -- do you want see a generate a visual output?"
                                                .to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            // here create a temporary file
                                            let temp_dot_file_res = NamedTempFile::new();

                                            match temp_dot_file_res {
                                                Err(e) => {
                                                    if !silent {
                                                        println!(
                                                            "could not generate output: {}",
                                                            e
                                                        );
                                                    }
                                                }
                                                Ok(temp_dot) => {
                                                    let path_to_temp_dot =
                                                        (&temp_dot).path().to_str();

                                                    match path_to_temp_dot {
                                                        Option::None => {
                                                            println!(
                                                                "path is not valid: {:?}",
                                                                &path_to_temp_dot
                                                            );
                                                        }
                                                        Some(path_to_temp) => {
                                                            // write to temporary file
                                                            write_str_to_file(
                                                                &output,
                                                                path_to_temp,
                                                            );

                                                            let dot_output_format = "pdf";

                                                            let name_output_file = format!(
                                                                "{}_conflict_graph.{}",
                                                                &abox.name(),
                                                                dot_output_format
                                                            );
                                                            let command = format!(
                                                                "dot -T{} {} -o {}",
                                                                dot_output_format,
                                                                path_to_temp,
                                                                &name_output_file
                                                            );

                                                            // execute dot command
                                                            // TODO: change this to be platform independent
                                                            let output = Command::new("sh")
                                                                .arg("-c")
                                                                .arg(&command)
                                                                .output();

                                                            match output {
                                                                Err(e) => {
                                                                    println!(
                                                                        "couldn't create output: {}",
                                                                        &e
                                                                    );
                                                                }
                                                                Ok(o) => {
                                                                    if !silent {
                                                                        let _std_out =
                                                                            std::str::from_utf8(
                                                                                &o.stdout,
                                                                            )
                                                                            .unwrap();
                                                                        println!(
                                                                            " -- file generated: {}",
                                                                            &name_output_file
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    println!("not sure how you arrived here...");
                                }
                            }
                        }
                        Option::None => {
                            println!("WARNING: could not create abox, maybe try to run with '--verbose' to see the errors");
                        }
                    }
                }
                (_, _) => {
                    println!(
                    "ERROR: you must provide a file containing a tbox and a file containing the abox"
                    );
                    std::process::exit(exitcode::USAGE);
                }
            }
        }
        _ => println!("NOT IMPLEMENTED"),
    }
}
