mod dl_lite;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

use crate::kb::types::FileType;

use crate::dl_lite::ontology::Ontology;
use crate::interface::cli::{Cli, Task};
use crate::interface::utilities::{get_filetype, parse_name_from_filename};

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use crate::dl_lite::sqlite_interface::{connect_to_db, update_symbols_to_db, add_abis_to_db};

fn main() {
    let args = Cli::from_args();

    // get all arguments regardless of the task
    let path_db_op: Option<std::path::PathBuf> = args.path_db;
    let path_tbox_op: Option<std::path::PathBuf> = args.path_tbox;
    let path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;

    match args.task {
        Task::INIT => {
            // to initialize the database you need tbox items
            if path_tbox_op.is_none() {
                println!("you must provide a tbox file to initialize the database");
                std::process::exit(exitcode::USAGE);
            } else {
                // get info from the tbox file
                let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                let tb_ft = get_filetype(&path_tbox);
                let tb_name = parse_name_from_filename(&path_tbox);

                // create a new ontology
                let mut onto = Ontology::new(String::from(tb_name));

                // get symbols from a symbols file if specified else from the tbox file
                match path_symbols_op {
                    Some(path_symbols) => {
                        let path_symbols = path_symbols.to_str().unwrap();
                        let symbols_ft = get_filetype(path_symbols);

                        onto.add_symbols(path_symbols, symbols_ft, verbose);
                    }
                    Option::None => {
                        if verbose {
                            // what is this ??
                            println!("here: {:?} and {:?}", &path_tbox, &tb_ft);
                        }

                        onto.add_symbols(&path_tbox, tb_ft, verbose);
                    }
                }

                // add tbis and complete the tbox
                onto.add_tbis(&path_tbox, tb_ft, verbose);
                onto.auto_complete(verbose);

                // create db name
                let mut path_to_db = match path_db_op {
                    Option::None => String::from(tb_name),
                    Some(pdb) => String::from(pdb.to_str().unwrap()),
                };
                // let mut path_to_db = String::from(tb_name);
                path_to_db.push_str(".db");

                // attempt to connect
                let conn = connect_to_db(&path_to_db, verbose);

                // populate
                onto.populate_db(&conn, verbose);
            }
        }
        Task::CTB => {
            if path_tbox_op.is_none() {
                println!("you must provide a tbox file to complete");
                std::process::exit(exitcode::USAGE);
            } else {
                let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                let tb_ft = get_filetype(&path_tbox);
                let tb_name = parse_name_from_filename(&path_tbox);

                // let path_output_op = args.path_output;
                // let path_symbols_op = args.path_symbols;

                let mut onto = Ontology::new(String::from(tb_name));

                match path_symbols_op {
                    Some(path_symbols) => {
                        let path_symbols = path_symbols.to_str().unwrap();
                        let symbols_ft = get_filetype(path_symbols);

                        onto.add_symbols(path_symbols, symbols_ft, args.verbose);
                    }
                    Option::None => {
                        println!("here: {:?} and {:?}", &path_tbox, &tb_ft);
                        onto.add_symbols(&path_tbox, tb_ft, args.verbose);
                    }
                }

                if verbose {
                    println!("attempting to add tbox items and complete");
                }

                onto.add_tbis(&path_tbox, tb_ft, verbose);
                onto.auto_complete(verbose);

                match path_output_op {
                    Some(path_output) => {
                        onto.tbox_to_file(path_output.to_str().unwrap(), FileType::NATIVE, true);
                    }
                    Option::None => {
                        let mut s = String::new();
                        let formatted =
                            format!("----<TBox>\n{}\n", &onto.tbox_to_string(&onto.tbox()));
                        s.push_str(formatted.as_str());

                        println!("{}", s);
                    }
                }
            }
        }
        Task::CAB => {
            println!("test!!!");

            if path_db_op.is_none() || path_abox_op.is_none() {
                println!("you must provide a database containing an ontology, if you don't have one maybe create one with the 'init' task");
                std::process::exit(exitcode::USAGE);
            } else {
                // read the database into an ontology
                let path_db = path_db_op.unwrap().to_str().unwrap().to_string();
                let onto_res = Ontology::initiate_from_db(&path_db, verbose);

                // path and name of the abox
                let path_abox = path_abox_op.unwrap().to_str().unwrap().to_string();
                let ab_name = parse_name_from_filename(&path_abox).trim().to_string();
                let ab_ft = get_filetype(&path_abox);

                // reserved name
                if ab_name == "temp_abox" {
                    println!("the name 'temp_abox' is reserved, please another one");
                    std::process::exit(exitcode::USAGE);
                }

                // real work
                match onto_res {
                    Err(e) => {
                        println!("an error ocurred: {}", &e);
                        std::process::exit(exitcode::IOERR);
                    }
                    Ok(mut onto) => {
                        println!("{}", &onto);

                        // establish connection it should be fine
                        let conn = connect_to_db(&path_db, verbose);

                        // add abox
                        onto.new_abox(&path_abox, ab_ft, verbose);

                        // udpate the new symbols
                        update_symbols_to_db(onto.symbols(), &conn, verbose);

                        // add abis and nodes that come with
                        add_abis_to_db(onto.symbols(), onto.abox().unwrap().items(), &onto.abox_name(),  &conn, verbose);
                    }
                }
            }
        }
        Task::RNK => println!("not implemented"),
        Task::UNDEFINED => println!("unrecognized task!"),
    }
}
