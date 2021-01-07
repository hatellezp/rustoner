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

pub trait DataItem {}
pub trait AxiomItem {}
pub trait Data {}
pub trait Axioms {}

type AxiomRule = fn(Vec<&dyn AxiomItem>) -> Option<Vec<dyn AxiomItem>>;
type DataRule = fn(Vec<&dyn AxiomItem>, Vec<&dyn DataItem>) -> Option<Vec<dyn DataItem>>;

/*
then we define traits for interacting with real data
 */

pub trait Interpreter {
    fn parse_axioms_from_file(filename: &str) -> Box<dyn Axioms>;
    fn parse_data_from_file(filename: &str) -> Box<dyn Data>;
}
