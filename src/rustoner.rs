mod dl_lite;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

// from kb


// from the dl_lite module
use crate::dl_lite::ontology::Ontology;
use crate::dl_lite::abox_item::ABI;
use crate::dl_lite::abox_item_quantum::ABIQ;
use crate::dl_lite::abox::ABQ;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::native_filetype_utilities::{tbox_to_native_string};
use crate::dl_lite::string_formatter::{tbi_to_string, pretty_print_abiq_conflict};

// from the interface module
use crate::interface::cli::{Cli, Task};

use crate::interface::utilities::{get_filetype, parse_name_from_filename, write_str_to_file};

// to ask basic questions
use question::{Answer, Question};

// the main function
pub fn main() {
    // to avoid trivial tbi written to file
    let dont_write_trivial = true;

    let args = Cli::from_args();

    // get all arguments regardless of the task
    let task: Task = args.task;
    // let path_db_op: Option<std::path::PathBuf> = args.path_db;
    let path_tbox_op: Option<std::path::PathBuf> = args.path_tbox;
    let path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;
    // let _ephemere: bool = args.ephemere;

    // now do what you are ask
    match task {
        Task::VERTB | Task::GENCONTB | Task::CTB => {
            if path_tbox_op.is_none() {
                println!("you must provide a tbox file to complete");
                std::process::exit(exitcode::USAGE);
            } else {
                // get the information from the file: name and bla bla
                let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                let tb_ft = get_filetype(&path_tbox);
                let tb_name = parse_name_from_filename(&path_tbox);

                // create a temporal ontology
                let mut onto = Ontology::new(String::from(tb_name));

                // get symbols from this file if possible
                match path_symbols_op {
                    Some(path_symbols) => {
                        let path_symbols = path_symbols.to_str().unwrap();
                        let symbols_ft = get_filetype(path_symbols);

                        onto.add_symbols_from_file(path_symbols, symbols_ft, args.verbose);
                    }
                    Option::None => {
                        // else add from the tbox file
                        onto.add_symbols_from_file(&path_tbox, tb_ft, args.verbose);
                    }
                }

                // now add tbis from the tbox file
                onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);

                match task {
                    Task::VERTB => {
                        let deduction_tree = false;
                        let new_tb = onto.complete_tbox(deduction_tree, verbose);

                        let mut contradictions: Vec<&TBI> = Vec::new();

                        for tbi in new_tb.items() {
                            if tbi.is_contradiction() && !tbi.is_trivial() {
                                contradictions.push(tbi);
                            }
                        }

                        match contradictions.len() {
                            0 => println!(" -- no contradictions nor possible contradictions were found"),
                            _ => {
                                println!(" -- possible contradictions were found");

                                // show contradictions
                                let question_print = "Do you want to see them".to_string();

                                let print_output = Question::new(&question_print)
                                    .default(Answer::YES)
                                    .show_defaults()
                                    .confirm();

                                if print_output == Answer::YES {
                                    let mut current_tbi_op: Option<String>;

                                    for tbi in contradictions {
                                        current_tbi_op = tbi_to_string(tbi, onto.symbols());

                                        if current_tbi_op.is_some() {
                                            println!("{}", &(current_tbi_op.unwrap()));
                                        } else {
                                            println!("passing");
                                        }
                                    }
                                }
                            },
                        }
                    },
                    Task::GENCONTB => { println!("not implemented yet"); },
                    Task::CTB => {
                        // deduction tree is activated only in generate consequence tree mode
                        let deduction_tree = false;

                        let new_tb = onto.complete_tbox(deduction_tree, verbose);

                        let question_print = "Do you want to see the output".to_string();
                        let print_output = Question::new(&question_print)
                            .default(Answer::YES)
                            .show_defaults()
                            .confirm();

                        if print_output == Answer::YES {
                            let dont_write_trivial = true;
                            let new_tb_as_string = onto.tbox_to_string(&new_tb, dont_write_trivial);

                            println!(" -- <TBox>\n{}", &new_tb_as_string);
                        }

                        match path_output_op {
                            Some(path_output) => {
                                let new_tb_as_string_op = tbox_to_native_string(&new_tb, onto.symbols(), dont_write_trivial);

                                match new_tb_as_string_op {
                                    Option::None => println!("couldn't create output, maybe run with 'verbose' to see more"),
                                    Some(new_tb_as_string) => {
                                        let filename = path_output.to_str().unwrap().to_string();

                                        write_str_to_file(&new_tb_as_string, &filename);
                                    },
                                }
                            }
                            Option::None => (),
                        }
                    },
                    _ => println!("impossible to be here"),
                }
            }
        },
        Task::VERAB | Task::CAB | Task::GENCONAB | Task::RNKAB => {
            // some common stuff
            if (path_tbox_op.is_none()) || path_abox_op.is_none() {
                println!("you must provide a file containing a tbox and a file containing the abox");
                std::process::exit(exitcode::USAGE);
            } else {
                // get information for the tbox
                let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                let onto_name = parse_name_from_filename(&path_tbox);
                let tb_filetype = get_filetype(&path_tbox);

                let mut onto = Ontology::new(onto_name.to_string());

                // add symbols from where you can
                if path_symbols_op.is_some() {
                    let path_symbols = path_symbols_op.unwrap().to_str().unwrap().to_string();
                    let symbols_filetype = get_filetype(&path_symbols);

                    onto.add_symbols_from_file(&path_symbols, symbols_filetype, verbose);
                } else {
                    onto.add_symbols_from_file(&path_tbox, tb_filetype, verbose);
                }

                // add tbis
                onto.add_tbis_from_file(&path_tbox, tb_filetype, verbose);

                // path and name of the abox
                let path_abox = path_abox_op.unwrap().to_str().unwrap().to_string();
                let ab_name = parse_name_from_filename(&path_abox).trim().to_string();
                let ab_ft = get_filetype(&path_abox);

                // reserved name
                if ab_name == "temp_abox" {
                    println!("the name 'temp_abox' is reserved, please use another one");
                    std::process::exit(exitcode::USAGE);
                }

                // add abox
                onto.new_abox_from_file_quantum(&path_abox, ab_ft, verbose);

                // we continue with the completion
                // we continue here after
                // before completion of abox you need to autocomplete the tbox in onto
                let deduction_tree = false;
                onto.auto_complete(deduction_tree, verbose);

                let option_ref_to_current_abox = onto.abox();
                let abox_completed_op = onto.complete_abox(verbose);

                match abox_completed_op {
                    Some(abox_completed) => {
                        match task {
                            Task::VERAB => {
                                //change current abox
                                let contradictions: Vec<(TBI, Vec<ABIQ>)> = abox_completed.is_inconsistent_detailed(onto.tbox(), verbose);
                                // let is_abox_consistent = abox_completed.is_inconsistent(onto.tbox(), verbose);

                                match contradictions.len() {
                                    0 => println!(" -- no contradictions nor possible contradictions were found"),
                                    _ => {
                                        println!(" -- possible contradictions were found");

                                        // show contradictions
                                        let question_print = "Do you want to see them".to_string();

                                        let print_output = Question::new(&question_print)
                                            .default(Answer::YES)
                                            .show_defaults()
                                            .confirm();

                                        if print_output == Answer::YES {
                                            let mut current_tbi_op: Option<String>;

                                            println!("[");

                                            for tuple in contradictions.iter() {
                                                let container = tuple.clone();

                                                let v_abiq_s = pretty_print_abiq_conflict((&container.0, &container.1), onto.symbols());

                                                println!("{}", &v_abiq_s);
                                            }

                                            println!("]");
                                        }
                                    },
                                }
                            },
                            Task::CAB => {},
                            Task::GENCONAB => {},
                            Task::RNKAB => {},
                            _ => { println!("not sure how you arrived here..."); }
                        }
                    },
                    Option::None =>  {
                        println!("the completion output nothing, maybe try to run with '--verbose' to see the errors");
                    }
                }
            }
        },
        _ => println!("not implemented for the moment"),
    }
}
