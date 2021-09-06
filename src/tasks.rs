use std::path::PathBuf;
use std::process::Command;

use petgraph::dot::{Config, Dot};
use question::{Answer, Question};
use tempfile::NamedTempFile;

use crate::alg_math::bounds::Adjusters;

use crate::alg_math::utilities::null_vector;
use crate::dl_lite::abox::AbqDllite;
use crate::dl_lite::native_filetype_utilities::abox_to_native_string_quantum;
use crate::dl_lite::ontology::OntologyDllite;
use crate::dl_lite::string_formatter::create_string_for_unravel_conflict_abox;
use crate::dl_lite::string_formatter::pretty_print_abiq_conflict;
use crate::dl_lite::string_formatter::{
    create_string_for_gencontb, create_string_for_unravel_conflict_tbox, tbi_to_string,
};
use crate::dl_lite::tbox_item::TbiDllite;
use crate::dl_lite::utilities::create_aboxq_graph_dot;
use crate::graph_maker::{
    create_graph_for_aboxq_unraveling, create_graph_for_tbox_unraveling, edge_attr_tbox_unraveling,
    node_attr_abox_unraveling, node_attr_tbox_unraveling,
};
use crate::helper::{command_exists, edge_attr, node_attr, rank_abox};
use crate::interface::cli::AggrName;
use crate::interface::cli::Task;
use crate::interface::utilities::{get_filetype, parse_name_from_filename, write_str_to_file};
use crate::kb::aggr_functions::{AGGR_COUNT, AGGR_MAX, AGGR_MEAN, AGGR_MIN, AGGR_SUM};
use crate::kb::knowledge_base::ABox;
use crate::kb::knowledge_base::{TBox, TBoxItem};
use crate::{
    ABoxRelatedPaths, TBoxRelatedPaths, B_TRANSLATE, COMMAND_SHELL_LINUX, COMMAND_SHELL_WINDOWS,
    DOT_COMMAND_LINUX, DOT_COMMAND_WINDOWS, M_SCALE, TOLERANCE,
};

// ===============================================================================================
// THESE ARE THE TASKS RELATED TO TBOXES

pub fn task_tbox_related(tbox_paths: TBoxRelatedPaths, task: Task, verbose: bool, silent: bool) {
    let (path_to_tbox_op, path_to_symbols_op, path_output_op) = tbox_paths;

    if let Some(path_tbox) = path_to_tbox_op {
        // get the information from the file: name and bla bla
        let path_tbox = path_tbox.to_str().unwrap().to_string();
        let tb_ft = get_filetype(&path_tbox);
        let tb_name = parse_name_from_filename(&path_tbox);

        // create a temporal ontology
        let mut onto = OntologyDllite::new(String::from(tb_name));

        // get symbols from this file if possible
        match path_to_symbols_op {
            Some(path_symbols) => {
                let path_symbols = path_symbols.to_str().unwrap();
                let symbols_ft = get_filetype(path_symbols);

                onto.add_symbols_from_file(path_symbols, symbols_ft, verbose);
            }
            Option::None => {
                // else add from the tbox file
                onto.add_symbols_from_file(&path_tbox, tb_ft, verbose);
            }
        }

        // now add tbis from the tbox file
        onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);

        // now we can pass the necessary information to each function
        match task {
            Task::VerTB => task_verify_tbox(&mut onto, verbose, silent),
            Task::GenConTB => {
                task_generate_consequences_tbox(&mut onto, path_output_op, tb_name, verbose, silent)
            }
            _ => {
                println!("ERROR: you must provide a tbox related task: 'verify' or  'generate consequences'");
                std::process::exit(exitcode::USAGE);
            }
        }
    } else {
        println!("ERROR: you must provide a tbox file");
        std::process::exit(exitcode::USAGE);
    }
}

// this function seem good to me
pub fn task_verify_tbox(onto: &mut OntologyDllite, verbose: bool, silent: bool) {
    let mut deduction_tree = false;

    /*
       to find possible contradictions in the tbox we need the full closure
    */

    let negative_only = 1_i8;
    let which_closure = true;

    // do the negative closure
    onto.generate_cln(deduction_tree, verbose, negative_only);
    let mut full_closure = onto.cln(which_closure);

    let mut contradictions: Vec<&TbiDllite> = Vec::new();

    for tbi in full_closure.items() {
        if tbi.is_contradiction() && !tbi.is_trivial() {
            contradictions.push(tbi);
        }
    }

    match contradictions.len() {
        0 => {
            println!(" -- no contradictions nor possible contradictions were found");
            std::process::exit(exitcode::OK);
        }
        _ => {
            println!(" -- possible contradictions were found");

            if !silent {
                // show contradictions
                let question_print = " -- do you want to see the contradictions?";

                let print_output = ask_question(question_print);

                if print_output == Answer::YES {
                    let mut current_tbi_op: Option<String>;

                    println!("{{");

                    for tbi in contradictions {
                        current_tbi_op = tbi_to_string(tbi, onto.symbols());

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
            let question_print = " -- do you want unravel for conflicts?";
            let print_output = ask_question(question_print);

            if print_output == Answer::YES {
                deduction_tree = true;

                onto.generate_cln(deduction_tree, verbose, negative_only);
                full_closure = onto.cln(which_closure);

                let only_conflicts = true;
                let pretty_string = create_string_for_unravel_conflict_tbox(
                    full_closure,
                    onto.symbols(),
                    only_conflicts,
                );

                if !silent {
                    println!("{}", &pretty_string);
                }
            }

            std::process::exit(exitcode::OK);
        }
    }
}

pub fn task_generate_consequences_tbox(
    onto: &mut OntologyDllite,
    path_output_op: &Option<PathBuf>,
    tbox_name: &str,
    verbose: bool,
    silent: bool,
) {
    // complete by deduction
    let deduction_tree = true;
    let dont_write_trivial = true;
    let negative_only = 1_i8;
    let which_closure = true;

    // closure
    onto.generate_cln(deduction_tree, verbose, negative_only);
    let full_closure = onto.cln(which_closure);

    let pretty_string =
        create_string_for_gencontb(full_closure, onto.symbols(), dont_write_trivial);

    if !silent {
        println!("{}", &pretty_string);
    }

    // here we put the generation of the graph
    // here to dot notation and graph stuff
    let question_print = " -- do you want to create a deduction graph by dot notation?";

    let print_output = ask_question(question_print);

    if print_output == Answer::YES {
        // TODO: I'm here
        let graph = create_graph_for_tbox_unraveling(full_closure, onto.symbols());

        let get_edge = edge_attr_tbox_unraveling;
        let get_node = node_attr_tbox_unraveling;

        let dot_notation =
            Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel], &get_edge, &get_node);

        let dot_notation_output = format!("{:?}", dot_notation);

        let filename = format!("{}_consequences.dot", tbox_name);
        write_str_to_file(&dot_notation_output, &filename);

        if !silent {
            println!(" -- dot file created: {}", &filename);
        }

        //create graph here
        // here verify that the command exists
        let dot_command_name = find_and_verify_dot_command();

        // now show graph
        let question_print = " -- do you want to generate a visual output?";

        let print_output = ask_question(question_print);

        if print_output == Answer::YES {
            generate_visual_and_dot_output(
                &dot_notation_output,
                dot_command_name,
                tbox_name,
                silent,
                false,
            );
        }
    }

    // consequences to file if presented
    write_output_op_to_file(path_output_op, &pretty_string);

    std::process::exit(exitcode::OK);
}

// this function is not needed at the moment
// pub fn task_complete_tbox(_onto: &mut OntologyDllite) {}

// ==============================================================================================
// HERE I WILL PUT THE ABOX RELATED TASKS

pub fn task_abox_related(
    abox_paths: ABoxRelatedPaths,
    aggr_name_op: &Option<AggrName>,
    task: Task,
    verbose: bool,
    silent: bool,
) {
    let (path_abox_op, path_tbox_op, path_symbols_op, path_output_op) = abox_paths;

    if let (Some(path_abox), Some(path_tbox)) = (path_abox_op, path_tbox_op) {
        println!(" -- be sure to have an abox without self conflicting facts before going further, you can use the 'cleanab' task for this");

        // get information for the tbox
        let path_tbox = path_tbox.to_str().unwrap().to_string();
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
        let path_abox = path_abox.to_str().unwrap().to_string();
        let ab_name = parse_name_from_filename(&path_abox).trim().to_string();
        let ab_ft = get_filetype(&path_abox);

        // reserved name
        if ab_name == "temp_abox" {
            println!("ERROR: the name 'temp_abox' is reserved, please use another one");
            std::process::exit(exitcode::USAGE);
        }

        // add abox
        onto.new_abox_from_file_quantum(&path_abox, ab_ft, verbose);

        match task {
            Task::VerAB => task_verify_abox(&mut onto, path_output_op, verbose, silent),
            Task::CleanAB => task_clean_abox(&mut onto, &ab_name, verbose, silent),
            Task::GenConAB => task_generate_consequences_abox(
                &mut onto,
                path_output_op,
                &ab_name,
                verbose,
                silent,
            ),
            Task::RankAB => task_rank_abox(
                &mut onto,
                path_output_op,
                aggr_name_op,
                &ab_name,
                verbose,
                silent,
            ),
            _ => {
                println!("ERROR: you must provide a abox related task: 'verify', 'clean', 'generate consequences' or 'rank'");
                std::process::exit(exitcode::USAGE);
            }
        }
    } else {
        println!("ERROR: you must provide a file containing a tbox and a file containing the abox");
        std::process::exit(exitcode::USAGE);
    }
}

pub fn task_verify_abox(
    onto: &mut OntologyDllite,
    path_output_op: &Option<PathBuf>,
    verbose: bool,
    silent: bool,
) {
    // first create the negative closure
    let deduction_tree = false;
    let negative_only = -1_i8;
    let which_closure = false;
    let detailed = false;

    onto.generate_cln(deduction_tree, verbose, negative_only);
    let negative_closure = onto.cln(which_closure);

    let abox_op = onto.abox();

    if let Some(abox) = abox_op {
        let (the_abox_is_inconsistent, _details) =
            AbqDllite::is_inconsistent_refs_only(abox.items_by_ref(), negative_closure, detailed);

        if !the_abox_is_inconsistent {
            println!(" -- no contradictions were found");
        } else {
            println!(" -- contradictions were found");

            if !silent {
                // show contradictions
                let question_print = " -- do you want to see them";
                let print_output = ask_question(question_print);

                if print_output == Answer::YES {
                    let _current_tbi_op: Option<String>;

                    let (_the_abox_is_inconsistent, contradictions_op) =
                        AbqDllite::is_inconsistent_refs_only(
                            abox.items_by_ref(),
                            negative_closure,
                            true,
                        );

                    if let Some(contradictions) = contradictions_op {
                        println!("[");

                        for tuple in contradictions.iter() {
                            let (tbi_op, abi_vec) = tuple;

                            let v_abiq_s =
                                pretty_print_abiq_conflict(tbi_op, abi_vec, onto.symbols());

                            println!("{}", &v_abiq_s);
                        }

                        println!("]");

                        // now unravel for conflicts
                        let question_print = " -- do you want unravel for conflicts?";
                        let print_output = ask_question(question_print);

                        if print_output == Answer::YES {
                            let only_conflicts = true;
                            let pretty_string = create_string_for_unravel_conflict_abox(
                                &abox,
                                onto.symbols(),
                                only_conflicts,
                                &contradictions,
                            );

                            if !silent {
                                println!("{}", &pretty_string);
                                println!(" -- if you see that one sole element might be sprouting conflicts, use the 'cleanab' task to clean the abox from self conflicting facts");
                            }

                            // don't forget to copy to file if output is specified
                            write_output_op_to_file(path_output_op, &pretty_string);
                        }
                    }
                }
            }
        }

        std::process::exit(exitcode::OK);
    } else {
        println!(
            "ERROR: the abox was not created, maybe run with 'verbose' option to see what happened"
        );
        std::process::exit(exitcode::CANTCREAT);
    }
}

pub fn task_clean_abox(onto: &mut OntologyDllite, ab_name: &str, verbose: bool, silent: bool) {
    let clean_name = format!("{}_clean", ab_name);
    let dirty_name = format!("{}_dirty", ab_name);
    let mut clean_ab = AbqDllite::new(&clean_name);
    let mut dirty_ab = AbqDllite::new(&dirty_name);

    // for formatting
    let dont_write_trivial = true;
    let mut pretty_string: String;

    // create the closure
    let deduction_tree = false;
    let negative_only = -1_i8;
    let which_closure = false;

    onto.generate_cln(deduction_tree, verbose, negative_only);
    let negative_closure = onto.cln(which_closure);

    let orig_ab_op = onto.abox();
    match orig_ab_op {
        Option::None => {
            println!("ERROR: the original abox is not present, maybe run with 'verbose' for more information");
            std::process::exit(exitcode::DATAERR);
        }
        Some(orig_ab) => {
            // remember that onto has the completed tbox
            for abiq in orig_ab.items() {
                let (is_self_conflict, _) =
                    AbqDllite::is_inconsistent_refs_only(vec![abiq], negative_closure, false);

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
                    pretty_string = onto.abox_to_string_quantum(&clean_ab);
                    println!("{}", &pretty_string);
                } else {
                    println!(" -- all assertions seems to be self conflicting");
                }

                if !dirty_is_empty {
                    println!(" -- dirty abox:");
                    pretty_string = onto.abox_to_string_quantum(&dirty_ab);
                    println!("{}", &pretty_string);
                } else {
                    println!(" -- seems that no self conflicting assertions where found");
                }
            }

            // now write to file
            let clean_name = clean_ab.name();
            let dirty_name = dirty_ab.name();

            // write to both files
            if !clean_is_empty {
                let clean_output_ab =
                    abox_to_native_string_quantum(&clean_ab, onto.symbols(), dont_write_trivial);

                match clean_output_ab {
                    Option::None => {
                        if !silent {
                            println!(" -- couldn't create clean abox!");
                        }
                    }
                    Some(output_ab) => {
                        write_str_to_file(&output_ab, &clean_name);

                        if !silent {
                            println!(" -- wrote clean abox to {}", &clean_name);
                        }
                    }
                }
            } else if !silent {
                println!(" -- no clean abox was written, as there is nothing to be written");
            }

            if !dirty_is_empty {
                let dirty_output_ab =
                    abox_to_native_string_quantum(&dirty_ab, onto.symbols(), dont_write_trivial);

                match dirty_output_ab {
                    Option::None => {
                        if !silent {
                            println!(" -- couldn't create dirty abox!");
                        }
                    }
                    Some(output_ab) => {
                        write_str_to_file(&output_ab, &dirty_name);

                        if !silent {
                            println!(" -- wrote dirty abox to {}", &dirty_name);
                        }
                    }
                }
            } else if !silent {
                println!(" -- no dirty element found to be written");
            }
        }
    }

    std::process::exit(exitcode::OK);
}

pub fn task_generate_consequences_abox(
    onto: &mut OntologyDllite,
    path_output_op: &Option<PathBuf>,
    ab_name: &str,
    verbose: bool,
    silent: bool,
) {
    let only_conflicts = false;

    // I will need the full closure here
    let deduction_tree = true;
    let only_negative = 1_i8;
    let which_closure = true;

    onto.generate_cln(deduction_tree, verbose, only_negative);
    let full_closure = onto.cln(which_closure);

    // complete abox
    let abox_completed = onto
        .abox()
        .unwrap()
        .complete(full_closure, deduction_tree, verbose);

    let ab_output = create_string_for_unravel_conflict_abox(
        &abox_completed,
        onto.symbols(),
        only_conflicts,
        &[],
    );

    if !silent {
        println!("{}", &ab_output);
    }

    // here we put the generation of the graph
    // here to dot notation and graph stuff
    let question_print = " -- do you want to create a deduction graph by dot notation?";
    let print_output = ask_question(question_print);

    if print_output == Answer::YES {
        // TODO: I'm here
        let new_tb = onto.tbox();

        let graph = create_graph_for_aboxq_unraveling(&abox_completed, new_tb, onto.symbols());

        let get_edge = edge_attr_tbox_unraveling;
        let get_node = node_attr_abox_unraveling;

        let dot_notation =
            Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel], &get_edge, &get_node);

        let dot_notation_output = format!("{:?}", dot_notation);

        let filename = format!("{}_consequences.dot", &ab_name);
        write_str_to_file(&dot_notation_output, &filename);

        if !silent {
            println!(" -- dot file created: {}", &filename);
        }

        let dot_command_name = find_and_verify_dot_command();

        // now show graph
        let question_print = " -- do you want see a generate a visual output?";
        let print_output = ask_question(question_print);

        if print_output == Answer::YES {
            generate_visual_and_dot_output(
                &dot_notation_output,
                dot_command_name,
                ab_name,
                silent,
                false,
            );
        }
    }

    // consequences to file if presented
    write_output_op_to_file(path_output_op, &ab_output);

    std::process::exit(exitcode::OK);
}

pub fn task_rank_abox(
    onto: &mut OntologyDllite,
    path_output_op: &Option<PathBuf>,
    aggr_name_op: &Option<AggrName>,
    ab_name: &str,
    verbose: bool,
    silent: bool,
) {
    // the current abox is not the completed one
    let mut abox = onto.abox().unwrap().clone();
    let deduction_tree = false;

    // find aggregation function
    let aggr = match aggr_name_op {
        Option::None => AGGR_SUM,
        Some(aggr_name) => match aggr_name {
            AggrName::Undefined => AGGR_SUM,
            AggrName::Sum => AGGR_SUM,
            AggrName::Max => AGGR_MAX,
            AggrName::Min => AGGR_MIN,
            AggrName::Mean => AGGR_MEAN,
            AggrName::Count => AGGR_COUNT,
        },
    };

    // generate the closure
    let _which_closure = false;
    let negative_only = 0_i8;

    onto.generate_cln(deduction_tree, verbose, negative_only);

    // defined the adjusters
    let adjusters: Adjusters = (TOLERANCE, M_SCALE, B_TRANSLATE);

    // TODO: after the test of concurrency come back here and use concurrency if it is better
    let use_concurrency = false;
    let use_concurrency = true;

    let (before_matrix, virtual_to_real, conflict_type) = rank_abox(
        &onto,
        &mut abox,
        deduction_tree,
        aggr,
        adjusters,
        verbose,
        use_concurrency,
    );
    // now the abox is ranked

    if !silent {
        let question_print = " -- do you want to see the output?";
        let print_output = ask_question(question_print);

        if print_output == Answer::YES {
            let abox_string = onto.abox_to_string_quantum(&abox);
            println!("{}", &abox_string);
        }
    }

    // save to file the new abox
    if path_output_op.is_some() {
        let dont_write_trivial = true;
        let abox_ranked_string_op =
            abox_to_native_string_quantum(&abox, onto.symbols(), dont_write_trivial);

        if let Some(abox_ranked_string) = &abox_ranked_string_op {
            write_output_op_to_file(path_output_op, abox_ranked_string);
        }
    }

    // now create graph if necessary
    let question_print = " -- do you want to create a conflict graph?";
    let print_output = ask_question(question_print);

    if print_output == Answer::YES {
        if null_vector(&before_matrix) {
            println!(" -- no conflicts where found, the conflict graph will be empty, passing");
            std::process::exit(exitcode::OK)
        }

        // here ask later for only conflicts or not
        let only_conflicts_graph = true;

        let graph = create_aboxq_graph_dot(
            &abox,
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

        let dot_notation =
            Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel], &get_edge, &get_node);

        let dot_notation_output = format!("{}", dot_notation);

        // two things: first save dot notation, second save graph to pdf
        let question_print = " -- do you want to save to dot notation?";
        let print_output = ask_question(question_print);

        if print_output == Answer::YES {
            let filename = format!("{}_conflict_graph.dot", abox.name());
            write_str_to_file(&dot_notation_output, &filename);

            if !silent {
                println!(" -- dot file created: {}", &filename);
            }
        }

        // here verify that the command exists
        let dot_command_name = find_and_verify_dot_command();

        // now show graph
        let question_print = " -- do you want see a generate a visual output?";
        let print_output = ask_question(question_print);

        if print_output == Answer::YES {
            generate_visual_and_dot_output(
                &dot_notation_output,
                dot_command_name,
                ab_name,
                silent,
                true,
            );
        }
    }

    std::process::exit(exitcode::OK);
}

// ===============================================================================================
// these are utitlies for every task

pub fn ask_question(question: &str) -> Answer {
    let answer = Question::new(question)
        .default(Answer::YES)
        .show_defaults()
        .confirm();

    answer
}

pub fn write_output_op_to_file(output_path: &Option<PathBuf>, text: &str) {
    match output_path {
        Some(path_output) => {
            let filename = path_output.to_str().unwrap().to_string();

            write_str_to_file(text, &filename);
        }
        Option::None => (),
    }
}

pub fn generate_visual_and_dot_output(
    dot_notation_output: &str,
    dot_command_name: &str,
    tbox_name: &str,
    silent: bool,
    ranking: bool,
) {
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
                    println!("path is not valid: {:?}", &path_to_temp_dot);
                }
                Some(path_to_temp) => {
                    // write to temporary file
                    write_str_to_file(dot_notation_output, path_to_temp);

                    let dot_output_format = "pdf";

                    let name_output_file = if ranking {
                        format!("{}_ranking.{}", tbox_name, dot_output_format)
                    } else {
                        format!("{}_consequences.{}", tbox_name, dot_output_format)
                    };
                    let command = format!(
                        "{} -T{} {} -o {}",
                        dot_command_name, dot_output_format, path_to_temp, &name_output_file
                    );

                    // execute dot command
                    // TODO: change this to be platform independent

                    let output = if cfg!(target_os = "windows") {
                        Command::new(COMMAND_SHELL_WINDOWS)
                            .arg("/C")
                            .arg(&command)
                            .output()
                    } else {
                        Command::new(COMMAND_SHELL_LINUX)
                            .arg("-c")
                            .arg(&command)
                            .output()
                    };

                    match output {
                        Err(e) => {
                            println!("couldn't create output: {}", &e);
                        }
                        Ok(o) => {
                            if !silent {
                                let _std_out = std::str::from_utf8(&o.stdout).unwrap();
                                println!(" -- file generated: {}", &name_output_file);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn find_and_verify_dot_command() -> &'static str {
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
        std::process::exit(exitcode::OSERR);
    }
    dot_command_name
}
