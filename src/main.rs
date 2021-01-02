mod helpers_and_utilities;
mod node;
mod rule;
mod tbox;
mod tbox_item;
mod types;

//mod scratch;

use crate::node::Node;
use crate::tbox::TB;
use crate::tbox_item::TBI;
use crate::types::DLType;
use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");

    /*
    let a2 = Node::new(Some(2), DLType::BaseRole).unwrap();
    let b2 = Node::new(Some(3), DLType::BaseConcept).unwrap();
    let c2 = Node::new(Some(4), DLType::Nominal).unwrap();

    println!("a2: {}, b2: {}, c2: {}", a2, b2, c2);
     */

    let mut q: VecDeque<usize> = VecDeque::new();
    let mut index = 0;
    for i in 1..10 {
        q.insert(index, i);
        index += 1;
    }

    let mut v: usize;
    while !q.is_empty() {
        v = q.pop_front().unwrap();

        if v % 4 == 0 {
            q.push_back((v / 4) * 13);
        }
        println!("{}", v);
    }

    // test, hyper stupid test
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
}
