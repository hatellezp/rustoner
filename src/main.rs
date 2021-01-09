use std::path::Path;

mod dl_lite;
mod kb;

use crate::dl_lite::interpreter::Ontology;
use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;

use crate::dl_lite::json_utilities::{parse_symbols_from_json, parse_tbox_from_json};
use std::collections::HashMap;

fn main() {
    println!("=================================================================");

    println!("Hello, world!");

    let mut onto = Ontology::new(String::from("test1")); // creates new ontology

    let p = Path::new("src");
    let p_tbox = p.join("dl_lite").join("examples").join("tbox1.json");
    let p_symb = p.join("dl_lite").join("examples").join("symbols1.json");

    let filename_tbox = p_tbox.to_str().unwrap();
    let filename_symb = p_symb.to_str().unwrap();

    onto.initialize_from_json(filename_tbox, filename_symb, true);
    println!("{}", &onto);

    println!("==============\ntbox:\n{:?}\n=======================", &onto.tbox());

    let tbox_completed = onto.complete_tbox(false);
    println!("tbox complete:\n{}", &tbox_completed);



}
