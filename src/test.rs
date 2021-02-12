mod dl_lite;
mod interface;
mod kb;

// for cli interface
use structopt::StructOpt;

use crate::kb::types::FileType;

use crate::dl_lite::ontology::Ontology;
use crate::interface::cli::{Cli, Task};
use crate::interface::utilities::{get_filetype, parse_name_from_filename};

use crate::dl_lite::sqlite_interface::{add_abis_to_db, connect_to_db, update_symbols_to_db};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

fn main() {
    println!("hello there");

    let verbose = false;
    let native = FileType::NATIVE;
    let ontology_file = String::from("ontology1");
    let abox_file = String::from("abox1");

    let mut onto = Ontology::new(ontology_file.clone());

    onto.add_symbols_from_file(&ontology_file, native, verbose);
    onto.add_tbis_from_file(&ontology_file, native, verbose);
    onto.new_abox_from_file(&abox_file, native, verbose);

    println!("{}", &onto);

    let abox = onto.abox().unwrap();
    let tbox = onto.tbox();

    let abox_completed = abox.complete(tbox, false);

    println!("{}", &abox_completed);

    onto.add_abis_from_abox(&abox_completed);

    // println!("{}", &onto);
}
