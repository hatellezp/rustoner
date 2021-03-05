mod dl_lite;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

// from kb


// from the dl_lite module
use crate::dl_lite::ontology::Ontology;

use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::native_filetype_utilities::{tbox_to_native_string};
use crate::dl_lite::string_formatter::{tbi_to_string};

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
    // let _path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;
    // let _ephemere: bool = args.ephemere;

    // now do what you are ask
    match task {
        Task::VERTB => {
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

                let new_tb = onto.complete_tbox(verbose);

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
            }
        },
        Task::GENCONTB => { println!("generate consequence tree for tbox not implemented yet"); },
        Task::CTB => {
            if path_tbox_op.is_none() {
                println!("you must provide a tbox file to complete");
                std::process::exit(exitcode::USAGE);
            } else {
                let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                let tb_ft = get_filetype(&path_tbox);
                let tb_name = parse_name_from_filename(&path_tbox);


                let mut onto = Ontology::new(String::from(tb_name));

                match path_symbols_op {
                    Some(path_symbols) => {
                        let path_symbols = path_symbols.to_str().unwrap();
                        let symbols_ft = get_filetype(path_symbols);

                        onto.add_symbols_from_file(path_symbols, symbols_ft, args.verbose);
                    }
                    Option::None => {
                        onto.add_symbols_from_file(&path_tbox, tb_ft, args.verbose);
                    }
                }


                onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);

                let new_tb = onto.complete_tbox(verbose);

                let question_print = "Do you want to see the output".to_string();
                let print_output = Question::new(&question_print)
                    .default(Answer::YES)
                    .show_defaults()
                    .confirm();

                if print_output == Answer::YES {
                    let new_tb_as_string = onto.tbox_to_string(&new_tb);

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
            }
        },
        Task::VERAB => { println!("verification of abox not implemented yet"); },
        Task::CAB => { println!("completion of abox not implemented yet"); },
        Task::GENCONAB => { println!("generate consequence tree for abox not implemented yet"); },
        Task::RNKAB => { println!("ranking of assertions not implemented yet"); },
        _ => println!("not implemented for the moment"),
    }
}
