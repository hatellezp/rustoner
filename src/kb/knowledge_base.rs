/*
The framework defined in 'dl_lite' can be generalized here using traits.

What is what define 'dl_lite':
    * types for symbol constructors
    * nodes for the structure of complex symbols
    * an axiom struct for the rules
    * an axioms struct to store several rules
    * an assertion struct for data
    * a data struct for
 */

use std::collections::HashMap;
use std::io;
use std::fmt::{Display, Debug};
use std::hash::Hash;

pub trait LType: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    fn same_type(&self, other: &Self) -> bool;
}

pub trait Expression: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized {
    fn string_to_expression<LT: LType>(s: &str, symbols: &HashMap<String, (usize, LT)>) -> io::Result<Self>;

    // this method is different of 'Display', it is for writing to file
    fn expression_to_string(&self) -> String;
}

pub trait DataItem: PartialOrd + Ord + PartialEq + Eq + Debug + Hash + Display + Sized{
    fn string_to_data_item<LT: LType>(s: &str, symbols: &HashMap<String, (usize, LT)>) -> io::Result<Self>;

    fn data_item_to_string(&self) -> String;
}

pub trait Data: PartialEq + Debug + Display {
    /*
    fn items<DI: DataItem>(&self) -> Vec<&DI>;
    fn len(&self) -> usize;
    fn add<DI: DataItem>(&mut self, item: DI) -> bool;
    fn contains<DI: DataItem>(&self, item: &DI) -> bool;

     */
}

pub trait AxiomItem {}
pub trait Axioms {}
