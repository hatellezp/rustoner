mod dl_lite;
mod kb;

use crate::dl_lite::interpreter::Ontology;
use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let c1 = Node::new(Some(10), DLType::BaseConcept).unwrap();
    let c2 = Node::new(Some(11), DLType::BaseConcept).unwrap();
    let r1 = Node::new(Some(20), DLType::BaseRole).unwrap();
    let r2 = Node::new(Some(21), DLType::BaseRole).unwrap();

    let r3 = Node::new(Some(21), DLType::BaseRole).unwrap();
    let r4 = Node::new(Some(22), DLType::BaseRole).unwrap();
    let r5 = Node::new(Some(20), DLType::BaseRole).unwrap();
    let r6 = Node::new(Some(22), DLType::BaseRole).unwrap();

    let c3 = c2.clone();
    let c4 = Node::new(Some(12), DLType::BaseConcept).unwrap();

    let c5 = Node::new(Some(13), DLType::BaseConcept).unwrap();
    let c6 = Node::new(Some(14), DLType::BaseConcept).unwrap();
    let c7 = c6.negate();

    let tbi_c = TBI::new(c1, c2).unwrap();
    let tbi_c2 = TBI::new(c3, c4).unwrap();
    let tbi_c3 = TBI::new(c5, c7).unwrap();
    let tbi_r = TBI::new(r1, r2).unwrap();
    let tbi_r2 = TBI::new(r3, r4).unwrap();
    let tbi_r3 = TBI::new(r5, r6).unwrap();

    let tbi_c3_reversed = (&tbi_c3).reverse_negation().unwrap();

    let mut tbox = TB::new();

    tbox.add(tbi_c2);
    tbox.add(tbi_c);
    tbox.add(tbi_c3);
    tbox.add(tbi_c3_reversed);

    tbox.add(tbi_r);
    tbox.add(tbi_r2);
    tbox.add(tbi_r3);

    println!("{}", &tbox);

    let new_tbox = tbox.complete2(true);

    println!("{}", &new_tbox);

    println!("=================================================================");

    let onto = Ontology::new(); // creates new ontology

    let p = Path::new("src");
    let p = p.join("dl_lite").join("examples").join("tbox1.json");
    let filename = p.to_str().unwrap();

    let tb = Ontology::parse_tbox_from_file_json(filename);

}
