/*
The idea of a context is to avoid making copies of every item.
A context shall be provided whenever computing tboxes and aboxes.
A context is nothing else than a bag of items
 */

use crate::item::DLType::{UNDEFINED, RoleBase, RoleInverse, RoleNegated, ConceptBottom, ConceptBase, ConceptExists, ConceptNegated, Constant};
use std::fmt;

use crate::item::{Item, Nominal, DLType};
use std::collections::HashSet;
use arrayvec::ArrayVec;

use std::collections::VecDeque;
use std::hash::{Hash, Hasher};


// I prefer a hashset to avoid adding new elements
#[derive(PartialEq, Eq, Debug)]
pub struct Context {
    pub bag: HashSet<Item>,  // TODO: verify that this is a good idea,
                            //        otherwise I have to implement all methods of Context
}

impl Context {
    pub fn new(vec: Vec<Item>) -> Context {
        let mut bag: HashSet<Item> = HashSet::new();

        for item in vec {
            bag.insert(item);
        }

        Context { bag }
    }
}

impl Hash for Context {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut counter: usize = 0;

        for b in &self.bag {
            b.hash(state);
            counter.hash(state);
            counter = counter + 1;
        }
    }
}
