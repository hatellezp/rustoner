mod dl_lite;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

use crate::kb::types::FileType;

use crate::dl_lite::ontology::Ontology;

use crate::interface::cli::{Cli, Task};
use crate::interface::utilities::{get_filetype, parse_name_from_filename};

use crate::dl_lite::sqlite_interface::{
    add_abis_to_db_quantum, connect_to_db, drop_tables_from_database, get_table_names,
    update_symbols_to_db,
};
use question::{Answer, Question};
use rusqlite::Connection;

fn main() {
    let args = Cli::from_args();

    // get all arguments regardless of the task
    let path_db_op: Option<std::path::PathBuf> = args.path_db;
    let path_tbox_op: Option<std::path::PathBuf> = args.path_tbox;
    let path_abox_op: Option<std::path::PathBuf> = args.path_abox;
    let path_symbols_op: Option<std::path::PathBuf> = args.path_symbols;
    let path_output_op: Option<std::path::PathBuf> = args.path_output;
    let verbose: bool = args.verbose;
    let _ephemere: bool = args.ephemere;

    // for the deduction tree
    let deduction_tree = false;

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

                        onto.add_symbols_from_file(path_symbols, symbols_ft, verbose);
                    }
                    Option::None => {
                        if verbose {
                            // what is this ??
                            // println!("here: {:?} and {:?}", &path_tbox, &tb_ft);
                        }

                        onto.add_symbols_from_file(&path_tbox, tb_ft, verbose);
                    }
                }

                // add tbis and complete the tbox
                onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);
                onto.auto_complete(deduction_tree, verbose);

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

                        onto.add_symbols_from_file(path_symbols, symbols_ft, args.verbose);
                    }
                    Option::None => {
                        // println!("here: {:?} and {:?}", &path_tbox, &tb_ft);
                        onto.add_symbols_from_file(&path_tbox, tb_ft, args.verbose);
                    }
                }

                if verbose {
                    println!("attempting to add tbox items and complete");
                }

                onto.add_tbis_from_file(&path_tbox, tb_ft, verbose);
                onto.auto_complete(deduction_tree, verbose);

                match path_output_op {
                    Some(path_output) => {
                        onto.tbox_to_file(path_output.to_str().unwrap(), FileType::NATIVE, true);
                    }
                    Option::None => {
                        let mut s = String::new();
                        let formatted =
                            format!("----<TBox>\n{}\n", &onto.tbox_to_string(&onto.tbox(), false));
                        s.push_str(formatted.as_str());

                        println!("{}", s);
                    }
                }
            }
        }
        Task::CAB => {
            if (path_db_op.is_none() && path_tbox_op.is_none()) || path_abox_op.is_none() {
                println!("you must provide a database or a file containing an ontology, if you don't have a database maybe create one with the 'init' task");
                std::process::exit(exitcode::USAGE);
            } else {
                let _isfile = path_tbox_op.is_some();
                let isdb = path_tbox_op.is_none(); // only use the database if no tbox file is specified
                let conn = Connection::open_in_memory();

                let mut conn = match conn {
                    Err(e) => {
                        if verbose {
                            println!("an error ocurred: {}", &e);
                        }
                        std::process::exit(exitcode::TEMPFAIL);
                    }
                    Ok(c) => c,
                };

                //the files is always preferred
                let mut onto: Ontology;

                if path_tbox_op.is_some() {
                    let path_tbox = path_tbox_op.unwrap().to_str().unwrap().to_string();
                    let onto_name = parse_name_from_filename(&path_tbox);
                    let tb_filetype = get_filetype(&path_tbox);

                    onto = Ontology::new(onto_name.to_string());

                    // add symbols
                    if path_symbols_op.is_some() {
                        let path_symbols = path_symbols_op.unwrap().to_str().unwrap().to_string();
                        let symbols_filetype = get_filetype(&path_symbols);

                        onto.add_symbols_from_file(&path_symbols, symbols_filetype, verbose);
                    } else {
                        onto.add_symbols_from_file(&path_tbox, tb_filetype, verbose);
                    }

                    // add tbis
                    onto.add_tbis_from_file(&path_tbox, tb_filetype, verbose);
                } else {
                    // read the database into an ontology
                    let path_db = path_db_op.clone().unwrap().to_str().unwrap().to_string();

                    // establish connection it should be fine
                    let conn: Connection;
                    conn = connect_to_db(&path_db, verbose);

                    // initiate won't add existent aboxes, only symbols and the tbox
                    let onto_res = Ontology::initiate_from_db(&path_db, verbose);

                    onto = match onto_res {
                        Err(e) => {
                            println!("an error ocurred: {}", &e);
                            std::process::exit(exitcode::IOERR);
                        }
                        Ok(o) => {
                            if verbose {
                                println!(
                                    "succeed on connection to database: {} with connection: {:?}",
                                    &path_db, &conn
                                );
                            }
                            o
                        }
                    };
                }
                // up here we defined onto

                // path and name of the abox
                let path_abox = path_abox_op.unwrap().to_str().unwrap().to_string();
                let ab_name = parse_name_from_filename(&path_abox).trim().to_string();
                let ab_ft = get_filetype(&path_abox);

                // reserved name
                if ab_name == "temp_abox" {
                    println!("the name 'temp_abox' is reserved, please use another one");
                    std::process::exit(exitcode::USAGE);
                }

                // real work

                // add abox
                onto.new_abox_from_file_quantum(&path_abox, ab_ft, verbose);

                if isdb {
                    let path_db = path_db_op.unwrap().to_str().unwrap().to_string();
                    conn = connect_to_db(&path_db, verbose);

                    // udpate the new symbols
                    update_symbols_to_db(onto.symbols(), &conn, verbose);

                    // add this abox to the database if it don't exists
                    let table_names_res = get_table_names(&conn, verbose);
                    let ab_table_name_c = format!("{}_abox_concept", &ab_name);
                    let ab_table_name_r = format!("{}_abox_role", &ab_name);

                    let mut table_exists = false;
                    match table_names_res {
                        Ok(v) => {
                            for s in &v {
                                table_exists = table_exists
                                    || s.contains(&ab_table_name_c)
                                    || s.contains(&ab_table_name_r);
                            }
                        }
                        _ => (),
                    };

                    // if table exists ask if you want to drop it
                    if table_exists {
                        let question_drop_table = format!("ABox {} already exists in database, drop it ? if this is a new abox better change the name and restart", &ab_name);
                        let drop_table = Question::new(&question_drop_table)
                            .default(Answer::NO)
                            .show_defaults()
                            .confirm();

                        if drop_table == Answer::YES {
                            println!("dropping abox...");

                            let tb_c = format!("{}_abox_concept", &ab_name);
                            let tb_r = format!("{}_abox_role", &ab_name);
                            let tables_to_drop = vec![tb_c.as_str(), tb_r.as_str()];

                            // drop the tables
                            drop_tables_from_database(&conn, tables_to_drop, verbose);
                        } else {
                            println!("aborting");
                            std::process::exit(exitcode::OK)
                        }
                    }

                    // first put the original abox in the database
                    add_abis_to_db_quantum(
                        onto.symbols(),
                        onto.abox().unwrap().items(),
                        &onto.abox_name(),
                        &conn,
                        verbose,
                    );
                }

                // we continue with the completion
                // we continue here after
                let abox_completed_op = onto.complete_abox(verbose);

                match abox_completed_op {
                    Some(abox_completed) => {
                        //change current abox
                        onto.new_abox_from_aboxq(abox_completed);

                        if isdb {
                            add_abis_to_db_quantum(
                                onto.symbols(),
                                onto.abox().unwrap().items(),
                                &onto.abox_name(),
                                &conn,
                                verbose,
                            );
                        }

                        match path_output_op {
                            Some(path_output) => {
                                let path_as_string = path_output.to_str().unwrap().to_string();
                                let _abox_completed_name =
                                    parse_name_from_filename(&path_as_string);
                                let abox_completed_filetype = get_filetype(&path_as_string);

                                onto.abox_to_file(&path_as_string, abox_completed_filetype, true);
                            }
                            _ => {
                                let question_print = "Do you want to see the output".to_string();
                                let print_output = Question::new(&question_print)
                                    .default(Answer::YES)
                                    .show_defaults()
                                    .confirm();

                                if print_output == Answer::YES {
                                    let abox_as_string =
                                        onto.abox_to_string_quantum(onto.abox().unwrap());

                                    println!("{}", &abox_as_string);
                                }
                            }
                        }
                    }
                    _ => {
                        println!("the completion output nothing, maybe try to run with '--verbose' to see the errors");
                    }
                }
            }
        },
        Task::RNKAB => println!("not implemented"),
        Task::UNDEFINED => println!("unrecognized task!"),
        _ => println!("not implemented!")
    }
}
