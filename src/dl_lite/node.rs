/*
© - 2021 – UMONS
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/

/// The ItemDllite struct is the materialization of the Item trait present in
/// kb module 'knowledge_base.rs' file, it is the basic construct for ontologies
/// objects.
/// The inner structure follows a basic enum construction:
/// - a bottom object
/// - a top object
/// - a role object with an identifier (a integer)
/// - a concept object with an identifier (a integer)
/// - a nominal object with an identifier (a integer)
/// - a complex object with a modifier (inverse, negated, exists quantifier)
///   and a reference to a child object.
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
pub enum ItemDllite {
    B,                       // bottom
    T,                       // top
    R(usize),                // base role
    C(usize),                // base concept
    N(usize),                // nominal
    X(Mod, Box<ItemDllite>), // complex type of items (e.g. inverse role or negated concept)
}

impl fmt::Display for ItemDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemDllite::B => write!(f, "<B>"),
            ItemDllite::T => write!(f, "<T>"),
            ItemDllite::R(n) => write!(f, "r({})", n),
            ItemDllite::C(n) => write!(f, "c({})", n),
            ItemDllite::N(n) => write!(f, "n({})", n),
            ItemDllite::X(m, bn) => match m {
                Mod::N => write!(f, "-{}", *((*bn).deref())),
                Mod::I => write!(f, "{}^-", *((*bn).deref())),
                Mod::E => write!(f, "E{}", *((*bn).deref())),
            },
        }
    }
}

impl PartialOrd for ItemDllite {
    /// compares self to other, with the following rule:
    /// concepts always before roles and roles always before nominals
    /// for basic constructs the identifier decides precedence
    /// for complex constructs:
    /// concept: bottom before base concept before exists quantifier before negation before top
    /// role: base before inverse before negation
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            if (self.t().is_concept_type() && !other.t().is_concept_type())
                || (self.t().is_role_type() && other.t().is_nominal_type())
            {
                Some(Ordering::Less)
            } else if (self.t().is_nominal_type() && !other.t().is_nominal_type())
                || (self.t().is_role_type() && other.t().is_concept_type())
            {
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
                                (ItemDllite::X(Mod::E, bnself), ItemDllite::X(Mod::E, bnother)) => {
                                    bnself.partial_cmp(bnother)
                                }
                                (_, _) => Option::None,
                            },
                            _ => Option::None,
                        },
                        DLType::NegatedConcept => match other.t() {
                            DLType::BaseConcept | DLType::ExistsConcept => Some(Ordering::Greater),
                            DLType::NegatedConcept => match (self, other) {
                                (ItemDllite::X(Mod::N, bnself), ItemDllite::X(Mod::N, bnother)) => {
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
                            (ItemDllite::X(Mod::I, bnself), ItemDllite::X(Mod::I, bnother)) => {
                                bnself.partial_cmp(bnother)
                            }
                            (_, _) => Option::None,
                        },
                        _ => Option::None,
                    },
                    DLType::NegatedRole => match other.t() {
                        DLType::BaseRole | DLType::InverseRole => Some(Ordering::Greater),
                        DLType::NegatedRole => match (self, other) {
                            (ItemDllite::X(Mod::N, bnself), ItemDllite::X(Mod::N, bnother)) => {
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

impl Ord for ItemDllite {
    /*
       The real function is defined in the PartialOrd trait.
    */
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Item for ItemDllite {
    /// Every Item as a type (a DLType).
    fn t(&self) -> DLType {
        // they have to be well formed, otherwise it will fail
        match self {
            ItemDllite::B => DLType::Bottom,
            ItemDllite::T => DLType::Top,
            ItemDllite::C(_) => DLType::BaseConcept,
            ItemDllite::R(_) => DLType::BaseRole,
            ItemDllite::N(_) => DLType::Nominal,
            ItemDllite::X(t, bn) => match (t, bn.deref()) {
                // if the item is not well formed is not the job of this function to detect it
                (Mod::N, ItemDllite::R(_)) | (Mod::N, ItemDllite::X(Mod::I, _)) => {
                    DLType::NegatedRole
                }
                (Mod::N, ItemDllite::C(_)) | (Mod::N, ItemDllite::X(Mod::E, _)) => {
                    DLType::NegatedConcept
                }
                (Mod::I, _) => DLType::InverseRole,
                (Mod::E, _) => DLType::ExistsConcept,
                (_, _) => {
                    println!("incorrect format for node: {}", &self);
                    std::process::exit(exitcode::DATAERR)
                }
            },
        }
    }

    /// Returns a reference to the base node, recursive function
    /// (e.g. 'NOT EXISTS INV eats' -> 'eats'
    /// but also 'eats' -> 'eats').
    fn base(node: &ItemDllite) -> &Self {
        match node {
            ItemDllite::B => &ItemDllite::B,
            ItemDllite::T => &ItemDllite::T,
            ItemDllite::C(_) => node,
            ItemDllite::R(_) => node,
            ItemDllite::N(_) => node,
            ItemDllite::X(_, bn) => ItemDllite::base(bn),
        }
    }

    /// tries to return the depth-th child of the current node
    /// return is wrapped in an option to ward against not finding the child
    /// (e.g.
    /// ('A INTER B', depth: 1) -> Some(['A', 'B'])
    /// ('A INTER B', depth: 0) -> Some(['A INTER B'])
    /// ('A INTER B', depth: 2) -> None
    /// ('NOT A', depth: 1) -> Some(['A'])
    /// )
    fn child(node: Option<&ItemDllite>, depth: usize) -> Option<Vec<&Self>> {
        match node {
            Option::None => Option::None,
            Some(n) => match (n, depth) {
                (_, 0) => Some(vec![node.unwrap()]),
                (ItemDllite::B, _)
                | (ItemDllite::T, _)
                | (ItemDllite::C(_), _)
                | (ItemDllite::R(_), _)
                | (ItemDllite::N(_), _) => Option::None, // there is no child for the bases types
                (ItemDllite::X(_, bn), _) => ItemDllite::child(Some(&bn), depth - 1),
            },
        }
    }

    /// returns true if self is equal to Top (which is the negation of bottom, the base type)
    /// or if self is effectively a negated construct
    fn is_negated(&self) -> bool {
        matches!(self, ItemDllite::T | ItemDllite::X(Mod::N, _))
    }
}

impl ItemDllite {
    /// creates a new ItemDllite from the specification
    /// an integer n as identifier and a type t
    /// the integer is wrapped in a Option type to allow for border cases
    /// like bottom and top
    pub fn new(n: Option<usize>, t: DLType) -> Option<ItemDllite> {
        match (n, t) {
            (_, DLType::Bottom) => Some(ItemDllite::B),
            (_, DLType::Top) => Some(ItemDllite::T),
            (Option::Some(n), DLType::BaseConcept) => Some(ItemDllite::C(n)),
            (Option::Some(n), DLType::BaseRole) => Some(ItemDllite::R(n)),
            (Option::Some(n), DLType::Nominal) => Some(ItemDllite::N(n)),
            (_, _) => Option::None,
        }
    }

    /// returns the identifier of the Item
    /// top is always 1 and bottom is always 0,values are reserved
    pub fn n(&self) -> usize {
        /*
            if you try to write a parser for rustoner be aware that this values
            are always reserved
        */
        match self {
            ItemDllite::T => 1,
            ItemDllite::B => 0,
            ItemDllite::C(n) | ItemDllite::R(n) | ItemDllite::N(n) => *n,
            ItemDllite::X(_, bn) => (*bn).n(),
        }
    }

    pub fn is_purely_negated(&self) -> bool {
        matches!(self, ItemDllite::X(Mod::N, _))
    }

    /// tries to build an Exists concept from self, will fail if self is not
    /// of the right form
    /// e.g. 'teaches' is a role: 'teaches' -> Some('EXISTS teaches')
    /// 'NOT teaches' -> None
    /// 'Human' is a concept: 'Human' -> None
    pub fn exists(self) -> Option<Self> {
        match (&self).t() {
            DLType::BaseRole | DLType::InverseRole => Some(ItemDllite::X(Mod::E, Box::new(self))),
            _ => Option::None,
        }
    }

    /// build a new ItemDllite struct with self as a child and a
    /// negation (NOT) modifier
    pub fn negate(self) -> Self {
        match self {
            ItemDllite::X(Mod::N, bn) => *bn,
            ItemDllite::B => ItemDllite::T,
            ItemDllite::T => ItemDllite::B,
            _ => ItemDllite::X(Mod::N, Box::new(self)),
        }
    }

    /// verfies that self is the negation of other
    /// e.g. ('NOT Human', 'Human') -> true
    /// ('NOT Human', 'Mortal') -> false
    pub fn is_negation(&self, other: &ItemDllite) -> bool {
        match (self, other) {
            // bottom and top
            (ItemDllite::B, ItemDllite::T) | (ItemDllite::T, ItemDllite::B) => true,
            // if both are negated return false
            (ItemDllite::X(Mod::N, _), ItemDllite::X(Mod::N, _)) => false,
            // if one is negated compare its child with the other
            (ItemDllite::X(Mod::N, bn), _) => bn.deref() == other,
            (_, ItemDllite::X(Mod::N, bn)) => self == bn.deref(),
            // anything else is false
            (_, _) => false,
        }
    }

    /// will try to inverse self, will fail if self is not of the correct form
    /// which is a base role or an inverse role,
    /// a negated role, a concept or a nominal are all not invertible
    pub fn inverse(self) -> Option<Self> {
        match self {
            ItemDllite::R(_) => Some(ItemDllite::X(Mod::I, Box::new(self))),
            ItemDllite::X(Mod::I, bn) => Some(*bn),
            _ => Option::None,
        }
    }

    /// check if other is the inverse of self, this implicitly
    /// implies that both other and self are roles
    pub fn is_inverse(&self, other: &Self) -> bool {
        match (self, other) {
            (ItemDllite::X(Mod::I, bn), _) => bn.deref() == other,
            (_, ItemDllite::X(Mod::I, bn)) => self == bn.deref(),
            (_, _) => false,
        }
    }
}
