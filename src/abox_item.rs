use crate::node::Node;
use std::fmt;
use crate::types::DLType;

/*
    remember that only base roles and base concepts are allowed here !!
 */
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ABI {
    RA(Node, Node, Node), // role assertion
    CA(Node, Node), // concept assertion
}

impl fmt::Display for ABI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABI::RA(r, a, b) => write!(f, "<{}: {}, {}>", r, a, b),
            ABI::CA(c, a) => write!(f, "<{}: {}>", c, a),
        }
    }
}

impl ABI {
    pub fn new_ra(r: Node, a: Node, b: Node) -> Option<ABI> {
        let is_base_role = r.t() == DLType::BaseRole;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            Some(ABI::RA(r, a, b))
        }
    }

    pub fn new_ca(c: Node, a: Node) -> Option<ABI> {
        let is_base_concept = c.t() == DLType::BaseConcept;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(ABI::CA(c, a))
        }
    }
}