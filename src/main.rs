mod dl_lite;
mod kb;

// use crate::dl_lite::node::Node;
// use crate::dl_lite::tbox::TB;
// use crate::dl_lite::tbox_item::TBI;
// use crate::dl_lite::types::DLType;
use crate::kb::types::FileType;

// use crate::dl_lite::json_utilities::{parse_symbols_from_json, parse_tbox_from_json};
// use std::collections::HashMap;
use crate::dl_lite::node::Node;
use crate::dl_lite::ontology::Ontology;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::mem;
use std::iter::Filter;
use std::process::id;
// use std::fs::File;

fn main() {
    println!("=================================================================");

    println!("Hello, world!");

    let json = FileType::JSON;
    let native = FileType::NATIVE;
    let mut onto = Ontology::new("Ontology1".to_string());

    let ontology1 = "src/dl_lite/examples/ontology1.dllite";

    onto.add_symbols(ontology1, native);
    onto.add_tbis(ontology1, native, false);

    println!("{}", &onto);

    println!("--------------------------------------");
    onto.auto_complete(false);
    println!("{}", &onto);

}

