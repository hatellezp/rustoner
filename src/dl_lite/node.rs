/*
TODO: I'm making a big choice here, top will be the negated one, that way we respect that no
      negations are present in the left hand
 */

use std::fmt;

use crate::kb::knowledge_base::Item;
use crate::kb::types::DLType;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Mod {
    N, // negation
    I, // inverse
    E, // exists
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Node_DLlite {
    B,
    T,
    R(usize),
    C(usize),
    N(usize),
    X(Mod, Box<Node_DLlite>),
}

impl fmt::Display for Node_DLlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node_DLlite::B => write!(f, "<B>"),
            Node_DLlite::T => write!(f, "<T>"),
            Node_DLlite::R(n) => write!(f, "r({})", n),
            Node_DLlite::C(n) => write!(f, "c({})", n),
            Node_DLlite::N(n) => write!(f, "n({})", n),
            Node_DLlite::X(m, bn) => match m {
                Mod::N => write!(f, "-{}", *((*bn).deref())),
                Mod::I => write!(f, "{}^-", *((*bn).deref())),
                Mod::E => write!(f, "E{}", *((*bn).deref())),
            },
        }
    }
}

impl PartialOrd for Node_DLlite {
    /*
    concepts before roles before nominals
    in concepts: bottom before base before exists before not before top
    in roles: base before inverse before not
     */
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            if self.t().is_concept_type() && !other.t().is_concept_type() {
                Some(Ordering::Less)
            } else if self.t().is_role_type() && other.t().is_nominal_type() {
                Some(Ordering::Less)
            } else if self.t().is_nominal_type() && !other.t().is_nominal_type() {
                Some(Ordering::Greater)
            } else if self.t().is_role_type() && other.t().is_concept_type() {
                Some(Ordering::Greater)
            } else if DLType::all_concepts(self.t(), other.t()) {
                if self.t() == DLType::Bottom || other.t() == DLType::Top {
                    Some(Ordering::Less)
                } else if self.t() == DLType::Top || other.t() == DLType::Bottom {
                    Some(Ordering::Greater)
                } else {
                    match self.t() {
                        DLType::BaseConcept => match other.t() {
                            DLType::BaseConcept => self.n().partial_cmp(&other.n()),
                            DLType::ExistsConcept | DLType::NegatedConcept => Some(Ordering::Less),
                            _ => Option::None,
                        },
                        DLType::ExistsConcept => match other.t() {
                            DLType::BaseConcept => Some(Ordering::Greater),
                            DLType::NegatedConcept => Some(Ordering::Less),
                            DLType::ExistsConcept => match (self, other) {
                                (
                                    Node_DLlite::X(Mod::E, bnself),
                                    Node_DLlite::X(Mod::E, bnother),
                                ) => bnself.partial_cmp(bnother),
                                (_, _) => Option::None,
                            },
                            _ => Option::None,
                        },
                        DLType::NegatedConcept => match other.t() {
                            DLType::BaseConcept | DLType::ExistsConcept => Some(Ordering::Greater),
                            DLType::NegatedConcept => match (self, other) {
                                (
                                    Node_DLlite::X(Mod::N, bnself),
                                    Node_DLlite::X(Mod::N, bnother),
                                ) => bnself.partial_cmp(bnother),
                                (_, _) => Option::None,
                            },
                            _ => Option::None,
                        },
                        _ => Option::None,
                    }
                }
            } else if DLType::all_roles(self.t(), other.t()) {
                match self.t() {
                    DLType::BaseRole => match other.t() {
                        DLType::BaseRole => self.n().partial_cmp(&other.n()),
                        DLType::InverseRole | DLType::NegatedRole => Some(Ordering::Less),
                        _ => Option::None,
                    },
                    DLType::InverseRole => match other.t() {
                        DLType::BaseRole => Some(Ordering::Greater),
                        DLType::NegatedRole => Some(Ordering::Less),
                        DLType::InverseRole => match (self, other) {
                            (Node_DLlite::X(Mod::I, bnself), Node_DLlite::X(Mod::I, bnother)) => {
                                bnself.partial_cmp(bnother)
                            }
                            (_, _) => Option::None,
                        },
                        _ => Option::None,
                    },
                    DLType::NegatedRole => match other.t() {
                        DLType::BaseRole | DLType::InverseRole => Some(Ordering::Greater),
                        DLType::NegatedRole => match (self, other) {
                            (Node_DLlite::X(Mod::N, bnself), Node_DLlite::X(Mod::N, bnother)) => {
                                bnself.partial_cmp(bnother)
                            }
                            (_, _) => Option::None,
                        },
                        _ => Option::None,
                    },
                    _ => Option::None,
                }
            } else if DLType::all_nominals(self.t(), other.t()) {
                // forcibly all nominals...
                self.n().partial_cmp(&other.n())
            } else {
                Option::None
            }
        }
    }
}

impl Ord for Node_DLlite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Item for Node_DLlite {
    fn t(&self) -> DLType {
        // they have to be well formed !
        match self {
            Node_DLlite::B => DLType::Bottom,
            Node_DLlite::T => DLType::Top,
            Node_DLlite::C(_) => DLType::BaseConcept,
            Node_DLlite::R(_) => DLType::BaseRole,
            Node_DLlite::N(_) => DLType::Nominal,
            Node_DLlite::X(t, bn) => match (t, bn.deref()) {
                (Mod::N, Node_DLlite::R(_)) | (Mod::N, Node_DLlite::X(Mod::I, _)) => {
                    DLType::NegatedRole
                }
                (Mod::N, Node_DLlite::C(_)) | (Mod::N, Node_DLlite::X(Mod::E, _)) => {
                    DLType::NegatedConcept
                }
                (Mod::I, Node_DLlite::R(_)) => DLType::InverseRole,
                (Mod::E, Node_DLlite::R(_)) | (Mod::E, Node_DLlite::X(Mod::I, _)) => {
                    DLType::ExistsConcept
                }
                (_, _) => panic!("incorrect format for node"),
            },
        }
    }

    fn base(node: &Node_DLlite) -> &Self {
        match node {
            Node_DLlite::B => &Node_DLlite::B,
            Node_DLlite::T => &Node_DLlite::T,
            Node_DLlite::C(_) => node,
            Node_DLlite::R(_) => node,
            Node_DLlite::N(_) => node,
            Node_DLlite::X(_, bn) => Node_DLlite::base(bn),
        }
    }

    // recursive version
    fn child(node: Option<&Node_DLlite>, depth: usize) -> Option<Vec<&Self>> {
        match node {
            Option::None => Option::None,
            Some(n) => match (n, depth) {
                (_, 0) => Some(vec![node.unwrap()]),
                (Node_DLlite::B, _)
                | (Node_DLlite::T, _)
                | (Node_DLlite::C(_), _)
                | (Node_DLlite::R(_), _)
                | (Node_DLlite::N(_), _) => Option::None,
                (Node_DLlite::X(_, bn), _) => Node_DLlite::child(Some(&bn), depth - 1),
            },
        }
    }

    fn is_negated(&self) -> bool {
        match self {
            Node_DLlite::T | Node_DLlite::X(Mod::N, _) => true, // it is BOTTOM which is negated ! UPDATE: is Top which is negated now...
            _ => false,
        }
    }
}

impl Node_DLlite {
    pub fn new(n: Option<usize>, t: DLType) -> Option<Node_DLlite> {
        match (n, t) {
            (_, DLType::Bottom) => Some(Node_DLlite::B),
            (_, DLType::Top) => Some(Node_DLlite::T),
            (Option::Some(n), DLType::BaseConcept) => Some(Node_DLlite::C(n)),
            (Option::Some(n), DLType::BaseRole) => Some(Node_DLlite::R(n)),
            (Option::Some(n), DLType::Nominal) => Some(Node_DLlite::N(n)),
            (_, _) => Option::None,
        }
    }

    pub fn n(&self) -> usize {
        /*
        0 and 1 are reserved values
         */
        match self {
            Node_DLlite::T => 1,
            Node_DLlite::B => 0,
            Node_DLlite::C(n) | Node_DLlite::R(n) | Node_DLlite::N(n) => *n,
            Node_DLlite::X(_, bn) => (*bn).n(),
        }
    }

    pub fn child_old(node: Option<&Node_DLlite>) -> Option<&Self> {
        match node {
            Option::None => Option::None,
            Some(n) => match n {
                Node_DLlite::B
                | Node_DLlite::T
                | Node_DLlite::C(_)
                | Node_DLlite::R(_)
                | Node_DLlite::N(_) => Option::None,
                Node_DLlite::X(_, bn) => Some(&bn),
            },
        }
    }

    pub fn exists(self) -> Option<Self> {
        match (&self).t() {
            DLType::BaseRole | DLType::InverseRole => Some(Node_DLlite::X(Mod::E, Box::new(self))),
            _ => Option::None,
        }
    }

    pub fn negate(self) -> Self {
        match self {
            Node_DLlite::X(Mod::N, bn) => *bn,
            Node_DLlite::B => Node_DLlite::T,
            Node_DLlite::T => Node_DLlite::B,
            _ => Node_DLlite::X(Mod::N, Box::new(self)),
        }
    }

    pub fn is_negation(&self, other: &Node_DLlite) -> bool {
        match (self, other) {
            // bottom and top
            (Node_DLlite::B, Node_DLlite::T) | (Node_DLlite::T, Node_DLlite::B) => true,
            // if both are negated return false
            (Node_DLlite::X(Mod::N, _), Node_DLlite::X(Mod::N, _)) => false,
            // if one is negated compare its child with the other
            (Node_DLlite::X(Mod::N, bn), _) => bn.deref() == other,
            (_, Node_DLlite::X(Mod::N, bn)) => self == bn.deref(),
            // anything else is false
            (_, _) => false,
        }
    }

    pub fn inverse(self) -> Option<Self> {
        match self {
            Node_DLlite::R(_) => Some(Node_DLlite::X(Mod::I, Box::new(self))),
            Node_DLlite::X(Mod::I, bn) => Some(*bn),
            _ => Option::None,
        }
    }

    pub fn is_inverted(&self) -> bool {
        self.t() == DLType::InverseRole
    }

    pub fn is_inverse(&self, other: &Node_DLlite) -> bool {
        match (self, other) {
            (Node_DLlite::X(Mod::I, _), Node_DLlite::X(Mod::I, _)) => false,
            (Node_DLlite::X(Mod::I, bn), _) => bn.deref() == other,
            (_, Node_DLlite::X(Mod::I, bn)) => self == bn.deref(),
            (_, _) => false,
        }
    }

    pub fn print_iter<I>(it: I) -> String
    where
        I: Iterator<Item = Node_DLlite>,
    {
        let mut s_accumulator = String::new();
        let mut waiting_s: String;
        for item in it {
            waiting_s = item.to_string();
            s_accumulator.push_str(waiting_s.as_str());
        }

        s_accumulator
    }
}
