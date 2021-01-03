use std::fmt;

use std::collections::HashSet;
use arrayvec::ArrayVec;
use crate::base::{BaseItem, Context};
use std::hash::{Hash, Hasher};

// only instances of base concepts and roles are allowed
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum ABoxItem<'a> {
    RoleAssertion(&'a BaseItem, &'a BaseItem, &'a BaseItem),
    ConceptAssertion(&'a BaseItem, &'a BaseItem)
}

#[derive(PartialEq, Eq, Debug)]
pub struct ABox<'a> {
    context: &'a Context,
    items: HashSet<ABoxItem<'a>>
}
// =================================================================================================
// here are implementation for known traits

impl<'a> Hash for ABox<'a> {
    /*
    I needed this, as deriving Hash automatically was not posssible for the rust compiler
     */
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut counter: usize = 0;

        self.context.hash(state);
        for b in &self.items {
            b.hash(state);
            counter.hash(state);
            counter = counter + 1;
        }
    }
}





