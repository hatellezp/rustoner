use std::fmt;

use crate::dl_lite::types::DLType;
use std::ops::Deref;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum Mod {
    N, // negation
    I, // inverse
    E, // exists
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum Node {
    B,
    T,
    R(usize),
    C(usize),
    N(usize),
    X(Mod, Box<Node>),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::B => write!(f, "<B>"),
            Node::T => write!(f, "<T>"),
            Node::R(n) => write!(f, "r({})", n),
            Node::C(n) => write!(f, "c({})", n),
            Node::N(n) => write!(f, "n({})", n),
            Node::X(m, bn) => match m {
                Mod::N => write!(f, "-{}", *((*bn).deref())),
                Mod::I => write!(f, "{}^-", *((*bn).deref())),
                Mod::E => write!(f, "E{}", *((*bn).deref())),
            },
        }
    }
}

impl Node {
    pub fn new(n: Option<usize>, t: DLType) -> Option<Node> {
        match (n, t) {
            (Option::None, DLType::Bottom) => Some(Node::B),
            (Option::None, DLType::Top) => Some(Node::T),
            (Option::Some(n), DLType::BaseConcept) => Some(Node::C(n)),
            (Option::Some(n), DLType::BaseRole) => Some(Node::R(n)),
            (Option::Some(n), DLType::Nominal) => Some(Node::N(n)),
            (_, _) => Option::None,
        }
    }

    pub fn n(&self) -> usize {
        /*
        0 and 1 are reserved values
         */
        match self {
            Node::T => 1,
            Node::B => 0,
            Node::C(n) | Node::R(n) | Node::N(n) => *n,
            Node::X(_, bn) => (*bn).n(),
        }
    }

    pub fn t(&self) -> DLType {
        // they have to be well formed !
        match self {
            Node::B => DLType::Bottom,
            Node::T => DLType::Top,
            Node::C(_) => DLType::BaseConcept,
            Node::R(_) => DLType::BaseRole,
            Node::N(_) => DLType::Nominal,
            Node::X(t, bn) => match (t, bn.deref()) {
                (Mod::N, Node::R(_)) | (Mod::N, Node::X(Mod::I, _)) => DLType::NegatedRole,
                (Mod::N, Node::C(_)) => DLType::NegatedConcept,
                (Mod::I, Node::R(_)) => DLType::InverseRole,
                (Mod::E, Node::R(_)) | (Mod::E, Node::X(Mod::I, _)) => DLType::ExistsConcept,
                (_, _) => panic!("incorrect format for node"),
            },
        }
    }

    pub fn child(node: Option<&Node>) -> Option<&Self> {
        match node {
            Option::None => Option::None,
            Some(n) => match n {
                Node::B | Node::T | Node::C(_) | Node::R(_) | Node::N(_) => Option::None,
                Node::X(_, bn) => Some(&bn),
            },
        }
    }

    // recursive version
    pub fn child_r(node: Option<&Node>, depth: usize) -> Option<&Self> {
        match node {
            Option::None => Option::None,
            Some(n) => match (n, depth) {
                (_, 0) => node,
                (Node::B, _)
                | (Node::T, _)
                | (Node::C(_), _)
                | (Node::R(_), _)
                | (Node::N(_), _) => Option::None,
                (Node::X(_, bn), _) => Node::child_r(Some(&bn), depth - 1),
            },
        }
    }

    pub fn exists(self) -> Option<Self> {
        match (&self).t() {
            DLType::BaseRole | DLType::InverseRole => Some(Node::X(Mod::E, Box::new(self))),
            _ => Option::None,
        }
    }

    pub fn negate(self) -> Self {
        match self {
            Node::X(Mod::N, bn) => *bn,
            Node::B => Node::T,
            Node::T => Node::B,
            _ => Node::X(Mod::N, Box::new(self)),
        }
    }

    pub fn is_negated(&self) -> bool {
        match self {
            Node::B | Node::X(Mod::N, _) => true, // it is BOTTOM which is negated !
            _ => false,
        }
    }

    pub fn is_negation(&self, other: &Node) -> bool {
        match (self, other) {
            // bottom and top
            (Node::B, Node::T) | (Node::T, Node::B) => true,
            // if both are negated return false
            (Node::X(Mod::N, _), Node::X(Mod::N, _)) => false,
            // if one is negated compare its child with the other
            (Node::X(Mod::N, bn), _) => bn.deref() == other,
            (_, Node::X(Mod::N, bn)) => self == bn.deref(),
            // anything else is false
            (_, _) => false,
        }
    }

    pub fn inverse(self) -> Option<Self> {
        match self {
            Node::R(_) => Some(Node::X(Mod::I, Box::new(self))),
            Node::X(Mod::I, bn) => Some(*bn),
            _ => Option::None,
        }
    }

    pub fn is_inverted(&self) -> bool {
        self.t() == DLType::InverseRole
    }

    pub fn is_inverse(&self, other: &Node) -> bool {
        match (self, other) {
            (Node::X(Mod::I, _), Node::X(Mod::I, _)) => false,
            (Node::X(Mod::I, bn), _) => bn.deref() == other,
            (_, Node::X(Mod::I, bn)) => self == bn.deref(),
            (_, _) => false,
        }
    }

    pub fn print_iter<I>(it: I) -> String
    where
        I: Iterator<Item = Node>,
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
