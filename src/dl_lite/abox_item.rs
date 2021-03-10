use std::fmt;

use crate::dl_lite::node::Node_DLlite;

use crate::dl_lite::tbox_item::TBI_DLlite;
use crate::kb::types::DLType;
use std::cmp::Ordering;

use crate::kb::knowledge_base::{Item, TBoxItem};

// help enum for the match function in the ABI implementation
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Side {
    None,
    Left,
    Right,
}

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ABI_DLlite {
    RA(Node_DLlite, Node_DLlite, Node_DLlite), // role assertion
    CA(Node_DLlite, Node_DLlite),              // concept assertion
}

impl fmt::Display for ABI_DLlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABI_DLlite::RA(r, a, b) => write!(f, "<{}: {}, {}>", r, a, b),
            ABI_DLlite::CA(c, a) => write!(f, "<{}: {}>", c, a),
        }
    }
}

impl PartialOrd for ABI_DLlite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self {
                ABI_DLlite::CA(c1, a1) => match other {
                    ABI_DLlite::RA(_, _, _) => Some(Ordering::Less),
                    ABI_DLlite::CA(c2, a2) => match c1.cmp(c2) {
                        Ordering::Equal => Some(a1.cmp(a2)),
                        _ => Some(c1.cmp(c2)),
                    },
                },
                ABI_DLlite::RA(r1, a1, b1) => match other {
                    ABI_DLlite::CA(_, _) => Some(Ordering::Greater),
                    ABI_DLlite::RA(r2, a2, b2) => match r1.cmp(r2) {
                        Ordering::Equal => match a1.cmp(a2) {
                            Ordering::Equal => Some(b1.cmp(b2)),
                            _ => Some(a1.cmp(a2)),
                        },
                        _ => Some(r1.cmp(r2)),
                    },
                },
            }
        }
    }
}

impl Ord for ABI_DLlite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*
the language only allows for base concept and role assertions,
here we however, we will allow for negation of and other complex constructions
this will allow for finding that 'a doesn't belong to A'
 */

impl ABI_DLlite {
    pub fn new_ra(
        r: Node_DLlite,
        a: Node_DLlite,
        b: Node_DLlite,
        for_completion: bool,
    ) -> Option<ABI_DLlite> {
        let is_base_role = r.t() == DLType::BaseRole || for_completion;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            Some(ABI_DLlite::RA(r, a, b))
        }
    }

    pub fn new_ca(c: Node_DLlite, a: Node_DLlite, for_completion: bool) -> Option<ABI_DLlite> {
        let is_base_concept = c.t() == DLType::BaseConcept || for_completion;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(ABI_DLlite::CA(c, a))
        }
    }

    pub fn negate(&self) -> ABI_DLlite {
        match self {
            ABI_DLlite::CA(c, a) => {
                let c_neg = c.clone().negate();

                ABI_DLlite::new_ca(c_neg, a.clone(), true).unwrap()
            }
            ABI_DLlite::RA(r, a, b) => {
                let r_neg = r.clone().negate();

                ABI_DLlite::new_ra(r_neg, a.clone(), b.clone(), true).unwrap()
            }
        }
    }

    pub fn is_trivial(&self) -> bool {
        match self {
            ABI_DLlite::CA(c, _) => c.t() == DLType::Top,
            _ => false,
        }
    }

    pub fn t(&self) -> DLType {
        match self {
            ABI_DLlite::RA(_, _, _) => DLType::BaseRole,
            ABI_DLlite::CA(_, _) => DLType::BaseConcept,
        }
    }

    // reference to the concept or role in the abox_item
    pub fn symbol(&self) -> &Node_DLlite {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        match self {
            ABI_DLlite::RA(r, _, _) => r,
            ABI_DLlite::CA(c, _) => c,
        }
    }

    pub fn same_nominal(&self, other: &Self) -> bool {
        match (self, other) {
            (ABI_DLlite::RA(_, a1, b1), ABI_DLlite::RA(_, a2, b2)) => a1 == a2 && b1 == b2,
            (ABI_DLlite::CA(_, a1), ABI_DLlite::CA(_, a2)) => a1 == a2,
            (_, _) => false,
        }
    }

    pub fn nominal(&self, position: usize) -> Option<&Node_DLlite> {
        /*
        will return a reference (wrapped in an Option) to the wanted nominal:
        if first position:
            A(a) -> a
            R(a,b) -> a
        if second position:
            A(a) -> None
            R(a,b) -> b
        any other position:
            -> None

            WARNING: this function returns positions with numeration beginning at 0!!
         */
        match self {
            ABI_DLlite::RA(_, a, b) => match position {
                0 => Some(a),
                1 => Some(b),
                _ => Option::None,
            },
            ABI_DLlite::CA(_, a) => match position {
                0 => Some(a),
                _ => Option::None,
            },
        }
    }

    pub fn decompact(self) -> (Node_DLlite, Node_DLlite, Option<Node_DLlite>) {
        match self {
            ABI_DLlite::RA(r, a, b) => (r, a, Some(b)),
            ABI_DLlite::CA(c, a) => (c, a, Option::None),
        }
    }

    pub fn decompact_with_clone(&self) -> (Node_DLlite, Node_DLlite, Option<Node_DLlite>) {
        let new_self = self.clone();
        new_self.decompact()
    }

    pub fn is_match(&self, tbi: &TBI_DLlite) -> Vec<Side> {
        // because tbox_item(s) are well formed, you only need to test against one
        let all_roles = DLType::all_roles(tbi.lside().t(), self.t());
        let all_concepts = DLType::all_concepts(tbi.lside().t(), self.t());

        if !all_roles && !all_concepts {
            vec![Side::None]
        } else {
            let sym = self.symbol();
            let left = sym == tbi.lside();
            let right = sym == tbi.rside();

            let mut v: Vec<Side> = Vec::new();

            if left {
                v.push(Side::Left);
            }

            if right {
                v.push(Side::Right)
            }

            v
        }
    }
}
