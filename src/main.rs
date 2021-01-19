mod dl_lite;
mod kb;

// use crate::dl_lite::node::Node;
// use crate::dl_lite::tbox::TB;
// use crate::dl_lite::tbox_item::TBI;
// use crate::dl_lite::types::DLType;
use crate::kb::types::FileType;

// use crate::dl_lite::json_utilities::{parse_symbols_from_json, parse_tbox_from_json};
// use std::collections::HashMap;
use crate::dl_lite::ontology::Ontology;
// use std::fs::File;

fn main() {
    println!("=================================================================");

    println!("Hello, world!");

    let mut onto = Ontology::new(String::from("test2"));

    let fntb1 = "src/dl_lite/examples/tbox1.json";
    let fntb2 = "src/dl_lite/examples/tbox2.json";
    let fnsb1 = "src/dl_lite/examples/symbols1.json";
    let fnsb2 = "src/dl_lite/examples/symbols2.json";
    let fntb3 = "src/dl_lite/examples/tbox3.json";
    let fntb4 = "src/dl_lite/examples/tbox4.json";

    onto.add_symbols(fnsb1, FileType::JSON);
    onto.add_symbols(fnsb2, FileType::JSON);

    // onto.add_tbis(fntb1, FileType::JSON, false);
    // onto.add_tbis(fntb2, FileType::JSON, false);
    // onto.add_tbis(fntb3, FileType::JSON, false);
    onto.add_tbis(fntb4, FileType::JSON, false);

    // println!("before completion:\n{}", &onto);

    onto.auto_complete(false);
    onto.sort();

    // println!("onto sorted:\n{}", &onto);

    let fntb_dumped1 = "src/dl_lite/examples/tbox_dumped1.json";
    onto.tbox_to_file(fntb_dumped1, FileType::JSON, true);
}
