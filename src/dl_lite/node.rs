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
    B,
    T,
    R(usize),
    C(usize),
    N(usize),
    X(Mod, Box<ItemDllite>),
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
    /*
    concepts before roles before nominals
    in concepts: bottom before base before exists before not before top
    in roles: base before inverse before not
     */
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
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Item for ItemDllite {
    fn t(&self) -> DLType {
        // they have to be well formed !
        match self {
            ItemDllite::B => DLType::Bottom,
            ItemDllite::T => DLType::Top,
            ItemDllite::C(_) => DLType::BaseConcept,
            ItemDllite::R(_) => DLType::BaseRole,
            ItemDllite::N(_) => DLType::Nominal,
            ItemDllite::X(t, bn) => match (t, bn.deref()) {
                (Mod::N, ItemDllite::R(_)) | (Mod::N, ItemDllite::X(Mod::I, _)) => {
                    DLType::NegatedRole
                }
                (Mod::N, ItemDllite::C(_)) | (Mod::N, ItemDllite::X(Mod::E, _)) => {
                    DLType::NegatedConcept
                }
                (Mod::I, ItemDllite::R(_)) => DLType::InverseRole,
                (Mod::E, ItemDllite::R(_)) | (Mod::E, ItemDllite::X(Mod::I, _)) => {
                    DLType::ExistsConcept
                }
                (_, _) => panic!("incorrect format for node"),
            },
        }
    }

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

    // recursive version
    fn child(node: Option<&ItemDllite>, depth: usize) -> Option<Vec<&Self>> {
        match node {
            Option::None => Option::None,
            Some(n) => match (n, depth) {
                (_, 0) => Some(vec![node.unwrap()]),
                (ItemDllite::B, _)
                | (ItemDllite::T, _)
                | (ItemDllite::C(_), _)
                | (ItemDllite::R(_), _)
                | (ItemDllite::N(_), _) => Option::None,
                (ItemDllite::X(_, bn), _) => ItemDllite::child(Some(&bn), depth - 1),
            },
        }
    }

    fn is_negated(&self) -> bool {
        // it is BOTTOM which is negated ! UPDATE: is Top which is negated now...
        matches!(self, ItemDllite::T | ItemDllite::X(Mod::N, _))
    }
}

impl ItemDllite {
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

    pub fn n(&self) -> usize {
        /*
        0 and 1 are reserved values
         */
        match self {
            ItemDllite::T => 1,
            ItemDllite::B => 0,
            ItemDllite::C(n) | ItemDllite::R(n) | ItemDllite::N(n) => *n,
            ItemDllite::X(_, bn) => (*bn).n(),
        }
    }

    pub fn child_old(node: Option<&ItemDllite>) -> Option<&Self> {
        match node {
            Option::None => Option::None,
            Some(n) => match n {
                ItemDllite::B
                | ItemDllite::T
                | ItemDllite::C(_)
                | ItemDllite::R(_)
                | ItemDllite::N(_) => Option::None,
                ItemDllite::X(_, bn) => Some(&bn),
            },
        }
    }

    pub fn exists(self) -> Option<Self> {
        match (&self).t() {
            DLType::BaseRole | DLType::InverseRole => Some(ItemDllite::X(Mod::E, Box::new(self))),
            _ => Option::None,
        }
    }

    pub fn negate(self) -> Self {
        match self {
            ItemDllite::X(Mod::N, bn) => *bn,
            ItemDllite::B => ItemDllite::T,
            ItemDllite::T => ItemDllite::B,
            _ => ItemDllite::X(Mod::N, Box::new(self)),
        }
    }

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

    pub fn inverse(self) -> Option<Self> {
        match self {
            ItemDllite::R(_) => Some(ItemDllite::X(Mod::I, Box::new(self))),
            ItemDllite::X(Mod::I, bn) => Some(*bn),
            _ => Option::None,
        }
    }
}
