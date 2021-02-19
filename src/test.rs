mod dl_lite;
mod interface;
mod kb;

// for cli interface

use crate::kb::types::FileType;

use crate::dl_lite::ontology_quantum::Ontology;

fn main() {
    println!("hello there");

    let verbose = true;
    let native = FileType::NATIVE;
    let ontology_file = String::from("ontology1");
    let abox_file = String::from("abox1_quantum");

    let mut onto = Ontology::new(ontology_file.clone());

    onto.add_symbols_from_file(&ontology_file, native, verbose);
    onto.add_tbis_from_file(&ontology_file, native, verbose);
    onto.new_abox_from_file_quantum(&abox_file, native, verbose);

    println!("{}", &onto);

    let abox = onto.abox().unwrap();
    let tbox = onto.tbox();

    let abox_completed = abox.complete(tbox, false);

    println!("{}", &abox_completed);

    onto.add_abis_from_abox(&abox_completed);

    // println!("{}", &onto);
}
