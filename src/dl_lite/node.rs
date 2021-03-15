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
pub enum NodeDllite {
    B,
    T,
    R(usize),
    C(usize),
    N(usize),
    X(Mod, Box<NodeDllite>),
}

impl fmt::Display for NodeDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeDllite::B => write!(f, "<B>"),
            NodeDllite::T => write!(f, "<T>"),
            NodeDllite::R(n) => write!(f, "r({})", n),
            NodeDllite::C(n) => write!(f, "c({})", n),
            NodeDllite::N(n) => write!(f, "n({})", n),
            NodeDllite::X(m, bn) => match m {
                Mod::N => write!(f, "-{}", *((*bn).deref())),
                Mod::I => write!(f, "{}^-", *((*bn).deref())),
                Mod::E => write!(f, "E{}", *((*bn).deref())),
            },
        }
    }
}

impl PartialOrd for NodeDllite {
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
                                (NodeDllite::X(Mod::E, bnself), NodeDllite::X(Mod::E, bnother)) => {
                                    bnself.partial_cmp(bnother)
                                }
                                (_, _) => Option::None,
                            },
                            _ => Option::None,
                        },
                        DLType::NegatedConcept => match other.t() {
                            DLType::BaseConcept | DLType::ExistsConcept => Some(Ordering::Greater),
                            DLType::NegatedConcept => match (self, other) {
                                (NodeDllite::X(Mod::N, bnself), NodeDllite::X(Mod::N, bnother)) => {
                                    bnself.partial_cmp(bnother)
                                }
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
                            (NodeDllite::X(Mod::I, bnself), NodeDllite::X(Mod::I, bnother)) => {
                                bnself.partial_cmp(bnother)
                            }
                            (_, _) => Option::None,
                        },
                        _ => Option::None,
                    },
                    DLType::NegatedRole => match other.t() {
                        DLType::BaseRole | DLType::InverseRole => Some(Ordering::Greater),
                        DLType::NegatedRole => match (self, other) {
                            (NodeDllite::X(Mod::N, bnself), NodeDllite::X(Mod::N, bnother)) => {
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

impl Ord for NodeDllite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Item for NodeDllite {
    fn t(&self) -> DLType {
        // they have to be well formed !
        match self {
            NodeDllite::B => DLType::Bottom,
            NodeDllite::T => DLType::Top,
            NodeDllite::C(_) => DLType::BaseConcept,
            NodeDllite::R(_) => DLType::BaseRole,
            NodeDllite::N(_) => DLType::Nominal,
            NodeDllite::X(t, bn) => match (t, bn.deref()) {
                (Mod::N, NodeDllite::R(_)) | (Mod::N, NodeDllite::X(Mod::I, _)) => {
                    DLType::NegatedRole
                }
                (Mod::N, NodeDllite::C(_)) | (Mod::N, NodeDllite::X(Mod::E, _)) => {
                    DLType::NegatedConcept
                }
                (Mod::I, NodeDllite::R(_)) => DLType::InverseRole,
                (Mod::E, NodeDllite::R(_)) | (Mod::E, NodeDllite::X(Mod::I, _)) => {
                    DLType::ExistsConcept
                }
                (_, _) => panic!("incorrect format for node"),
            },
        }
    }

    fn base(node: &NodeDllite) -> &Self {
        match node {
            NodeDllite::B => &NodeDllite::B,
            NodeDllite::T => &NodeDllite::T,
            NodeDllite::C(_) => node,
            NodeDllite::R(_) => node,
            NodeDllite::N(_) => node,
            NodeDllite::X(_, bn) => NodeDllite::base(bn),
        }
    }

    // recursive version
    fn child(node: Option<&NodeDllite>, depth: usize) -> Option<Vec<&Self>> {
        match node {
            Option::None => Option::None,
            Some(n) => match (n, depth) {
                (_, 0) => Some(vec![node.unwrap()]),
                (NodeDllite::B, _)
                | (NodeDllite::T, _)
                | (NodeDllite::C(_), _)
                | (NodeDllite::R(_), _)
                | (NodeDllite::N(_), _) => Option::None,
                (NodeDllite::X(_, bn), _) => NodeDllite::child(Some(&bn), depth - 1),
            },
        }
    }

    fn is_negated(&self) -> bool {
        match self {
            NodeDllite::T | NodeDllite::X(Mod::N, _) => true, // it is BOTTOM which is negated ! UPDATE: is Top which is negated now...
            _ => false,
        }
    }
}

impl NodeDllite {
    pub fn new(n: Option<usize>, t: DLType) -> Option<NodeDllite> {
        match (n, t) {
            (_, DLType::Bottom) => Some(NodeDllite::B),
            (_, DLType::Top) => Some(NodeDllite::T),
            (Option::Some(n), DLType::BaseConcept) => Some(NodeDllite::C(n)),
            (Option::Some(n), DLType::BaseRole) => Some(NodeDllite::R(n)),
            (Option::Some(n), DLType::Nominal) => Some(NodeDllite::N(n)),
            (_, _) => Option::None,
        }
    }

    pub fn n(&self) -> usize {
        /*
        0 and 1 are reserved values
         */
        match self {
            NodeDllite::T => 1,
            NodeDllite::B => 0,
            NodeDllite::C(n) | NodeDllite::R(n) | NodeDllite::N(n) => *n,
            NodeDllite::X(_, bn) => (*bn).n(),
        }
    }

    pub fn child_old(node: Option<&NodeDllite>) -> Option<&Self> {
        match node {
            Option::None => Option::None,
            Some(n) => match n {
                NodeDllite::B
                | NodeDllite::T
                | NodeDllite::C(_)
                | NodeDllite::R(_)
                | NodeDllite::N(_) => Option::None,
                NodeDllite::X(_, bn) => Some(&bn),
            },
        }
    }

    pub fn exists(self) -> Option<Self> {
        match (&self).t() {
            DLType::BaseRole | DLType::InverseRole => Some(NodeDllite::X(Mod::E, Box::new(self))),
            _ => Option::None,
        }
    }

    pub fn negate(self) -> Self {
        match self {
            NodeDllite::X(Mod::N, bn) => *bn,
            NodeDllite::B => NodeDllite::T,
            NodeDllite::T => NodeDllite::B,
            _ => NodeDllite::X(Mod::N, Box::new(self)),
        }
    }

    pub fn is_negation(&self, other: &NodeDllite) -> bool {
        match (self, other) {
            // bottom and top
            (NodeDllite::B, NodeDllite::T) | (NodeDllite::T, NodeDllite::B) => true,
            // if both are negated return false
            (NodeDllite::X(Mod::N, _), NodeDllite::X(Mod::N, _)) => false,
            // if one is negated compare its child with the other
            (NodeDllite::X(Mod::N, bn), _) => bn.deref() == other,
            (_, NodeDllite::X(Mod::N, bn)) => self == bn.deref(),
            // anything else is false
            (_, _) => false,
        }
    }

    pub fn inverse(self) -> Option<Self> {
        match self {
            NodeDllite::R(_) => Some(NodeDllite::X(Mod::I, Box::new(self))),
            NodeDllite::X(Mod::I, bn) => Some(*bn),
            _ => Option::None,
        }
    }

    /*
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

     */

    /*
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

     */
}
