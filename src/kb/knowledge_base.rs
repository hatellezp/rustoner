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

pub trait Data {}

pub trait AxiomItem {}
pub trait Axioms {}
