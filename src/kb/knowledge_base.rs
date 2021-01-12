use crate::kb::types::FileType;
use std::collections::HashMap;

/*
Here I will try to implement a global interface for
what a knowledge base is:
    * data item
    * data
    * axiom
    * axioms
    * deductions rules for axioms
    * deduction rules for axioms acting on data
 */

pub trait SymbolType {}

pub trait DataItem {}

pub trait AxiomItem {}

pub trait Data {
    /*
    fn add(&self, data_item: dyn DataItem) -> bool;
    fn contains(&self, data_item: &dyn DataItem) -> bool;
    fn is_consistent(&self, axioms: &dyn Axioms) -> bool;

     */
}

pub trait Axioms {
    /*
    fn add(&self, axiom_item: dyn AxiomItem) -> bool;
    fn contains(&self, axiom_item: &dyn AxiomItem) -> bool;
    fn complete(&self) -> Box<dyn Axioms>;

     */
}

/*
type AxiomRule = fn(Vec<&dyn AxiomItem>) -> Option<Vec<dyn AxiomItem>>;
type DataRule = fn(Vec<&dyn AxiomItem>, Vec<&dyn DataItem>) -> Option<Vec<dyn DataItem>>;

 */

/*
then we define traits for interacting with real data
 */

/*
pub trait Interpreter {
    fn parse_symbols_from_file(filename: &str, filetype: FileType) -> Box<HashMap<String, (usize, dyn SymbolType)>>;
    fn parse_axioms_from_file(filename: &str, filetype: FileType) -> Option<Box<dyn Axioms>>;
}

 */
