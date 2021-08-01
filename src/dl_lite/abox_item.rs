/*
UMONS 2021
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

use std::cmp::Ordering;
use std::fmt;

use crate::dl_lite::node::ItemDllite;

use crate::kb::knowledge_base::Item;
use crate::kb::types::DLType;

/// ABox items are built as an enumerate struct.
/// We have two types of ABox assertions, role assertions and concept assertions.
/// They are built like a tuple of Items, where the first is a role or a concept
/// and the following are the nominal (constants).
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum AbiDllite {
    RA(ItemDllite, ItemDllite, ItemDllite), // role assertion
    CA(ItemDllite, ItemDllite),             // concept assertion
}

impl fmt::Display for AbiDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AbiDllite::RA(r, a, b) => write!(f, "<{}: {}, {}>", r, a, b),
            AbiDllite::CA(c, a) => write!(f, "<{}: {}>", c, a),
        }
    }
}

// Almost all construct in rustoner can be ordered.
// As always, order is lexicographic.
impl PartialOrd for AbiDllite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self {
                AbiDllite::CA(c1, a1) => match other {
                    AbiDllite::RA(_, _, _) => Some(Ordering::Less),
                    AbiDllite::CA(c2, a2) => match c1.cmp(c2) {
                        Ordering::Equal => Some(a1.cmp(a2)),
                        _ => Some(c1.cmp(c2)),
                    },
                },
                AbiDllite::RA(r1, a1, b1) => match other {
                    AbiDllite::CA(_, _) => Some(Ordering::Greater),
                    AbiDllite::RA(r2, a2, b2) => match r1.cmp(r2) {
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

// The real function is implemented in PartialOrd
impl Ord for AbiDllite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*
the language only allows for base concept and role assertions,
here we however, we will allow for negation of and other complex constructions
this will allow for finding that 'a doesn't belong to A'
 */
impl AbiDllite {
    // There are two constructors for ABox items, role assertions are
    // different enough of concept assertions to justify defining two
    // methods.
    // We could do a sole general method, but keeping it easy
    // to read is one of my mottos when writing code.

    /// Creates a new role assertion in the form of a ABox item.
    /// Three principal arguments are provided, a role and two
    /// constants.
    /// A last argument: 'for_completion', is a switch to allow
    /// incorrect syntactically assertions during the reasoning tasks.
    pub fn new_ra(
        r: ItemDllite,
        a: ItemDllite,
        b: ItemDllite,
        for_completion: bool,
    ) -> Option<AbiDllite> {
        let is_base_role = r.t() == DLType::BaseRole || for_completion;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            Some(AbiDllite::RA(r, a, b))
        }
    }

    /// Same approach as 'new_ra', but this time two principal arguments, a concept
    /// item and a constant item.
    /// The 'for_completion' argument is as in 'new_ra'.
    pub fn new_ca(c: ItemDllite, a: ItemDllite, for_completion: bool) -> Option<AbiDllite> {
        let is_base_concept = c.t() == DLType::BaseConcept || for_completion;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(AbiDllite::CA(c, a))
        }
    }

    /// Negate the constant or role in the ABox item that self is.
    /// Be aware that this method produces a new object.
    pub fn negate(&self) -> AbiDllite {
        match self {
            AbiDllite::CA(c, a) => {
                let c_neg = c.clone().negate();

                AbiDllite::new_ca(c_neg, a.clone(), true).unwrap()
            }
            AbiDllite::RA(r, a, b) => {
                let r_neg = r.clone().negate();

                AbiDllite::new_ra(r_neg, a.clone(), b.clone(), true).unwrap()
            }
        }
    }

    /// Checks if self is of the form 'some_constant : Top' which
    /// is a trivial assertion.
    pub fn is_trivial(&self) -> bool {
        match self {
            AbiDllite::CA(c, _) => c.t() == DLType::Top,
            _ => false,
        }
    }

    /// Retrieves the type of the non constants element in
    /// the ABox assertions.
    pub fn t(&self) -> DLType {
        // rewrote this to be in accord with the actual implementation
        // before it was BaseRole or BaseConcept that were returned,
        // but during completion some other DLType are allowed, thus I
        // adjusted the method
        match self {
            AbiDllite::RA(c_or_r, _, _) | AbiDllite::CA(c_or_r, _) => c_or_r.t(),
        }
    }

    /// Returns a reference to the role item if self is a role assertion
    /// or a reference to the concept item if self is a concept assertion.
    pub fn symbol(&self) -> &ItemDllite {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        match self {
            AbiDllite::RA(r, _, _) => r,
            AbiDllite::CA(c, _) => c,
        }
    }

    /// Checks if self and other are of the same type (role assertion or
    /// concept assertion) and if true then checks if the items in
    /// self and other are the same.
    /// e.g.
    /// ('horacio': 'Human', 'alejandro': 'Human') -> true
    /// ('horacio': 'Human', 'horacio': 'Mortal') -> false
    /// ('horacio': 'Human', 'horacio', "apple': 'eats') -> false
    pub fn same_nominal(&self, other: &Self) -> bool {
        match (self, other) {
            (AbiDllite::RA(_, a1, b1), AbiDllite::RA(_, a2, b2)) => a1 == a2 && b1 == b2,
            (AbiDllite::CA(_, a1), AbiDllite::CA(_, a2)) => a1 == a2,
            (_, _) => false,
        }
    }

    pub fn decompact_nominals_refs(&self) -> Vec<&ItemDllite> {
        match self {
            AbiDllite::CA(_, a) => vec![a],
            AbiDllite::RA(_, a, b) => vec![a, b],
        }
    }
}
