use std::path::Path;

mod dl_lite;
mod kb;

use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::{DLType, FileType};

use crate::dl_lite::json_utilities::{parse_symbols_from_json, parse_tbox_from_json};
use std::collections::HashMap;
use crate::dl_lite::ontology::Ontology;
use std::fs::File;

fn main() {
    println!("=================================================================");

    println!("Hello, world!");

    let mut onto = Ontology::new(String::from("test2"));

    let fntb1 = "src/dl_lite/examples/tbox1.json";
    let fntb2 = "src/dl_lite/examples/tbox2.json";
    let fnsb1 = "src/dl_lite/examples/symbols1.json";
    let fnsb2 = "src/dl_lite/examples/symbols2.json";

    println!("before everything:\n{}", &onto);

    onto.add_symbols(fnsb1, FileType::JSON);
    onto.add_tbis(fntb1, FileType::JSON, true);

    println!("after first add:\n{}", &onto);

    onto.add_symbols(fnsb2, FileType::JSON);
    onto.add_tbis(fntb1, FileType::JSON, true);
    onto.add_tbis(fntb2, FileType::JSON, true);

    println!("after second add:\n{}", &onto);
}
