use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::rule::AbRule;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;
use std::cmp::Ordering;

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
pub enum ABI {
    RA(Node, Node, Node), // role assertion
    CA(Node, Node),       // concept assertion
}

impl fmt::Display for ABI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABI::RA(r, a, b) => write!(f, "<{}: {}, {}>", r, a, b),
            ABI::CA(c, a) => write!(f, "<{}: {}>", c, a),
        }
    }
}

impl PartialOrd for ABI {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self {
                ABI::CA(c1, a1) => {
                    match other {
                        ABI::RA(_, _, _) => Some(Ordering::Less),
                        ABI::CA(c2, a2) => {
                            match c1.cmp(c2) {
                                Ordering::Equal => {
                                    Some(a1.cmp(a2))
                                },
                                _ => Some(c1.cmp(c2))
                            }
                        },
                    }
                },
                ABI::RA(r1, a1, b1) => {
                    match other {
                        ABI::CA(_, _) => Some(Ordering::Greater),
                        ABI::RA(r2, a2, b2) => {
                            match r1.cmp(r2) {
                                Ordering::Equal => {
                                    match a1.cmp(a2) {
                                        Ordering::Equal => Some(b1.cmp(b2)),
                                        _ => Some(a1.cmp((a2)))
                                    }
                                },
                                _ => Some(r1.cmp(r2))
                            }
                        },
                    }
                },
            }
        }
    }
}

impl Ord for ABI {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*
the language only allows for base concept and role assertions,
here we however, we will allow for negation of and other complex constructions
this will allow for finding that 'a doesn't belong to A'
 */

impl ABI {
    pub fn new_ra(r: Node, a: Node, b: Node, for_completion: bool) -> Option<ABI> {
        let is_base_role = r.t() == DLType::BaseRole || for_completion;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            Some(ABI::RA(r, a, b))
        }
    }

    pub fn new_ca(c: Node, a: Node, for_completion: bool) -> Option<ABI> {
        let is_base_concept = c.t() == DLType::BaseConcept || for_completion;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(ABI::CA(c, a))
        }
    }

    pub fn is_trivial(&self) -> bool {
        match self {
            ABI::CA(c, _) => c.t() == DLType::Top,
            _ => false,
        }
    }

    pub fn t(&self) -> DLType {
        match self {
            ABI::RA(_, _, _) => DLType::BaseRole,
            ABI::CA(_, _) => DLType::BaseConcept,
        }
    }

    // reference to the concept or role in the abox_item
    pub fn symbol(&self) -> &Node {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        match self {
            ABI::RA(r, _, _) => r,
            ABI::CA(c, _) => c,
        }
    }

    pub fn nominal(&self, position: usize) -> Option<&Node> {
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
            ABI::RA(_, a, b) => match position {
                0 => Some(a),
                1 => Some(b),
                _ => Option::None,
            },
            ABI::CA(_, a) => match position {
                0 => Some(a),
                _ => Option::None,
            },
        }
    }

    pub fn decompact(self) -> (Node, Node, Option<Node>) {
        match self {
            ABI::RA(r, a, b) => (r, a, Some(b)),
            ABI::CA(c, a) => (c, a, Option::None),
        }
    }

    pub fn decompact_with_clone(&self) -> (Node, Node, Option<Node>) {
        let new_self = self.clone();
        new_self.decompact()
    }

    pub fn is_match(&self, tbi: &TBI) -> Side {
        // because tbox_item(s) are well formed, you only need to test against one
        let all_roles = DLType::all_roles(tbi.lside().t(), self.t());
        let all_concepts = DLType::all_concepts(tbi.lside().t(), self.t());

        if !all_roles && !all_concepts {
            Side::None
        } else {
            let sym = self.symbol();
            let left = sym == tbi.lside();
            let right = sym == tbi.rside();

            if left {
                Side::Left
            } else if right {
                Side::Right
            } else {
                Side::None
            }
        }
    }

    // pub fn apply_two(one: &ABI, two: &ABI, tbox: &TB) -> Option<Vec<ABI>> {}
    pub fn apply_rule(abis: Vec<&ABI>, tbis: Vec<&TBI>, rule: &AbRule) -> Option<Vec<ABI>> {
        let prov_vec = match tbis.len() {
            1 => rule(abis, tbis),
            2 => rule(abis, tbis),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec: Vec<ABI> = prov_vec.unwrap();
            let mut final_vec: Vec<ABI> = Vec::new();

            for item in &prov_vec {
                // println!("trying to add: {}", item);

                if !item.is_trivial() {
                    // println!("    success");

                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }
}
