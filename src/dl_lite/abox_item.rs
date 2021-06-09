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

use std::fmt;
use std::cmp::Ordering;

use crate::dl_lite::node::ItemDllite;

use crate::kb::types::DLType;
use crate::kb::knowledge_base::Item;


/*
   remember that only base roles and base concepts are allowed here !!
*/
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

    pub fn new_ca(c: ItemDllite, a: ItemDllite, for_completion: bool) -> Option<AbiDllite> {
        let is_base_concept = c.t() == DLType::BaseConcept || for_completion;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(AbiDllite::CA(c, a))
        }
    }

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

    pub fn is_trivial(&self) -> bool {
        match self {
            AbiDllite::CA(c, _) => c.t() == DLType::Top,
            _ => false,
        }
    }

    pub fn t(&self) -> DLType {
        match self {
            AbiDllite::RA(_, _, _) => DLType::BaseRole,
            AbiDllite::CA(_, _) => DLType::BaseConcept,
        }
    }

    // reference to the concept or role in the abox_item
    pub fn symbol(&self) -> &ItemDllite {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        match self {
            AbiDllite::RA(r, _, _) => r,
            AbiDllite::CA(c, _) => c,
        }
    }

    pub fn same_nominal(&self, other: &Self) -> bool {
        match (self, other) {
            (AbiDllite::RA(_, a1, b1), AbiDllite::RA(_, a2, b2)) => a1 == a2 && b1 == b2,
            (AbiDllite::CA(_, a1), AbiDllite::CA(_, a2)) => a1 == a2,
            (_, _) => false,
        }
    }

    /*
    pub fn nominal(&self, position: usize) -> Option<&NodeDllite> {
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
            AbiDllite::RA(_, a, b) => match position {
                0 => Some(a),
                1 => Some(b),
                _ => Option::None,
            },
            AbiDllite::CA(_, a) => match position {
                0 => Some(a),
                _ => Option::None,
            },
        }
    }

    pub fn decompact(self) -> (NodeDllite, NodeDllite, Option<NodeDllite>) {
        match self {
            AbiDllite::RA(r, a, b) => (r, a, Some(b)),
            AbiDllite::CA(c, a) => (c, a, Option::None),
        }
    }

    pub fn decompact_with_clone(&self) -> (NodeDllite, NodeDllite, Option<NodeDllite>) {
        let new_self = self.clone();
        new_self.decompact()
    }

    pub fn is_match(&self, tbi: &TbiDllite) -> Vec<Side> {
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

     */
}
