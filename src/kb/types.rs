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

// =================================================================================================
// IMPORTS
use crate::interface::format_constants::{
    UNICODE_BOT, UNICODE_EXISTS, UNICODE_NEG, UNICODE_RIGHTARROW, UNICODE_SQSUBSETEQ,
    UNICODE_SUBSETEQ, UNICODE_TOP,
};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;

// END OF IMPORTS
// =================================================================================================

/// This reasoner need to know the type of each DL expression, so we type them,
/// this is their type.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum DLType {
    Bottom,
    Top,
    BaseConcept,
    BaseRole,
    InverseRole,
    ExistsConcept,
    NegatedRole,
    NegatedConcept,
    Nominal,
}

// implementation of Display for DLType (this is how rust is)
impl fmt::Display for DLType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DLType::Bottom => write!(f, "DLType::Bottom"),
            DLType::Top => write!(f, "DLType::Top"),
            DLType::BaseConcept => write!(f, "DLType::BaseConcept"),
            DLType::ExistsConcept => write!(f, "DLType::ExistsConcept"),
            DLType::NegatedConcept => write!(f, "DLType::NegatedConcept"),
            DLType::BaseRole => write!(f, "DLType::BaseRole"),
            DLType::InverseRole => write!(f, "DLType::InverseRole"),
            DLType::NegatedRole => write!(f, "DLType::NegatedRole"),
            DLType::Nominal => write!(f, "DLType::Nominal"),
        }
    }
}

impl DLType {
    /// test if self is of Nominal type
    pub fn is_nominal_type(&self) -> bool {
        matches!(self, DLType::Nominal)
    }

    /// test if self is a Role type
    pub fn is_role_type(&self) -> bool {
        matches!(
            self,
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole
        )
    }

    /// returns true if self if of Concept type
    pub fn is_concept_type(&self) -> bool {
        matches!(
            self,
            DLType::Bottom
                | DLType::Top
                | DLType::BaseConcept
                | DLType::ExistsConcept
                | DLType::NegatedConcept
        )
    }

    /// test if both t1 and t2 are Role types
    pub fn all_roles(t1: DLType, t2: DLType) -> bool {
        match t1 {
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => matches!(
                t2,
                DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole
            ),
            _ => false,
        }
    }

    /*
    I'm going to add top and bottom in concepts
     */

    /// test if both t1 and t2 are Concept types
    pub fn all_concepts(t1: DLType, t2: DLType) -> bool {
        match t1 {
            DLType::Bottom
            | DLType::Top
            | DLType::BaseConcept
            | DLType::ExistsConcept
            | DLType::NegatedConcept => matches!(
                t2,
                DLType::Bottom
                    | DLType::Top
                    | DLType::BaseConcept
                    | DLType::ExistsConcept
                    | DLType::NegatedConcept
            ),
            _ => false,
        }
    }

    pub fn all_nominals(t1: DLType, t2: DLType) -> bool {
        // returns true if both t1 and t2 are of Nominal type

        t1 == DLType::Nominal && t2 == DLType::Nominal
    }

    pub fn same_type(t1: DLType, t2: DLType) -> bool {
        // test if both t1 and t2 has the same type : both roles, both concepts or
        // both nominals

        DLType::all_roles(t1, t2) || DLType::all_concepts(t1, t2) || DLType::all_nominals(t1, t2)
    }
}

/// Each deduction rule is numbered for pretty print and debug, this numbers
/// are tracked by the CR (count rules) type
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CR {
    Zero,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
}

impl CR {
    /// cast to usize
    pub fn to_usize(&self) -> usize {
        match self {
            CR::Zero => 0,
            CR::First => 1,
            CR::Second => 2,
            CR::Third => 3,
            CR::Fourth => 4,
            CR::Fifth => 5,
            CR::Sixth => 6,
            CR::Seventh => 7,
            CR::Eight => 8,
        }
    }

    /// Pretty printer for rules, the 'for_tbi' bool says
    /// if rules is to be formatted for TBox rules or ABox rules
    pub fn description(&self, for_tbi: bool) -> String {
        if cfg!(target_os = "windows") {
            match self {
                CR::Zero => {
                    if for_tbi {
                        String::from("X -> BOTTOM < X, X < TOP")
                    } else {
                        String::from("R0: NONE")
                    }
                }
                CR::First => {
                    if for_tbi {
                        String::from("X < NOT Y -> Y < NOT X")
                    } else {
                        // if (a,b):r then a:Er and b:Er^⁻
                        String::from("(a,b): r -> a: Exists r, b: Exists r^-")
                    }
                }
                CR::Second => {
                    if for_tbi {
                        String::from("X < Y, Y < Z -> X < Z")
                    } else {
                        // if (a,b):r and r < s then (a,b):s
                        String::from("(a,b): r, r < s -> (a,b): s")
                    }
                }
                CR::Third => {
                    // third rule: r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB
                    if for_tbi {
                        String::from(
                            "r < s, X < NOT EXISTS s -> X < NOT EXISTS r, EXISTS r < NOT X",
                        )
                    } else {
                        // if a:c and c < d then a:d
                        String::from("a: X, X < Y -> a: Y")
                    }
                }
                CR::Fourth => {
                    // fourth rule: r1=>r2 and B=>notExists_r2_inv then B=>notExists_r1_inv
                    if for_tbi {
                        String::from("r < s, B < NOT EXISTS s^- -> X < NOT EXISTS r^-")
                    } else {
                        String::from("R4: NONE")
                    }
                }
                CR::Fifth => {
                    // fifth rule: Exists_r=>notExists_r then r=>not_r and Exists_r_inv=>notExists_r_inv
                    if for_tbi {
                        String::from(
                            "EXISTS r < NOT EXISTS r -> r < NOT r, EXISTS r^- < NOT EXISTS r^-",
                        )
                    } else {
                        String::from("R5: NONE")
                    }
                }
                CR::Sixth => String::from("R6: NONE"),
                CR::Seventh => {
                    // seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv
                    if for_tbi {
                        String::from(
                            "r < NOT r -> EXISTS r < NOT EXISTS r, EXISTS r^- < NOT EXISTS r^-",
                        )
                    } else {
                        String::from("R7: NONE")
                    }
                }
                CR::Eight => {
                    // eight rule: r1=>r2 then r1_inv=>r2_inv and Exists_r1=>Exists_r2
                    if for_tbi {
                        String::from("r < s -> r^- < s^-, EXISTS r < EXISTS s")
                    } else {
                        String::from("R8: NONE")
                    }
                }
            }
        } else {
            match self {
                CR::Zero => {
                    if for_tbi {
                        format!(
                            "{}: X {} X{}{}, {}{}X",
                            self.identifier(),
                            UNICODE_RIGHTARROW,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_TOP,
                            UNICODE_BOT,
                            UNICODE_SQSUBSETEQ
                        )
                    } else {
                        String::from("R0: NONE")
                    }
                }
                CR::First => {
                    if for_tbi {
                        format!(
                            "{}: X{}{}Y {} Y{}{}X",
                            self.identifier(),
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_RIGHTARROW,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG
                        )
                    } else {
                        // if (a,b):r then a:Er and b:Er^⁻
                        format!(
                            "{}: (a,b):r {} a:{}r, b:{}r^-",
                            self.identifier(),
                            UNICODE_RIGHTARROW,
                            UNICODE_EXISTS,
                            UNICODE_EXISTS,
                        )
                    }
                }
                CR::Second => {
                    if for_tbi {
                        format!(
                            "{}: X{}Y, Y{}Z {} X{}Z",
                            self.identifier(),
                            UNICODE_SQSUBSETEQ,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_RIGHTARROW,
                            UNICODE_SQSUBSETEQ
                        )
                    } else {
                        // if (a,b):r and r < s then (a,b):s
                        format!(
                            "{}: (a,b):r, r{}s {} (a,b):s",
                            self.identifier(),
                            UNICODE_SUBSETEQ,
                            UNICODE_RIGHTARROW,
                        )
                    }
                }
                CR::Third => {
                    // third rule: r1=>r2 and B=>notExists_r2 then B=>notExists_r1 and Exists_r1=>notB
                    if for_tbi {
                        format!(
                            "{}: r{}s, X{}{}{}s {} X{}{}{}r, {}r{}{}X",
                            self.identifier(),
                            UNICODE_SUBSETEQ,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_RIGHTARROW,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG
                        )
                    } else {
                        // if a:c and c < d then a:d
                        format!(
                            "{}: a:X, X{}Y {} a:Y",
                            self.identifier(),
                            UNICODE_SQSUBSETEQ,
                            UNICODE_RIGHTARROW,
                        )
                    }
                }
                CR::Fourth => {
                    // fourth rule: r1=>r2 and B=>notExists_r2_inv then B=>notExists_r1_inv
                    if for_tbi {
                        format!(
                            "{}: r{}s, X{}{}{}s^- {} X{}{}{}r^-",
                            self.identifier(),
                            UNICODE_SUBSETEQ,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_RIGHTARROW,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                        )
                    } else {
                        String::from("R4: NONE")
                    }
                }
                CR::Fifth => {
                    // fifth rule: Exists_r=>notExists_r then r=>not_r and Exists_r_inv=>notExists_r_inv
                    if for_tbi {
                        format!(
                            "{}: {}r{}{}{}r {} r{}{}r, {}r^-{}{}{}r^-",
                            self.identifier(),
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_RIGHTARROW,
                            UNICODE_SUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                        )
                    } else {
                        String::from("R5: NONE")
                    }
                }
                CR::Sixth => String::from("R6: NONE"),
                CR::Seventh => {
                    // seventh rule: r=>not_r then Exists_r=>notExists_r and Exists_r_inv=>notExists_r_inv
                    if for_tbi {
                        format!(
                            "{}: r{}{}r {} {}r{}{}{}r, {}r^-{}{}{}r^-",
                            self.identifier(),
                            UNICODE_SUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_RIGHTARROW,
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS,
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_NEG,
                            UNICODE_EXISTS
                        )
                    } else {
                        String::from("R7: NONE")
                    }
                }
                CR::Eight => {
                    // eight rule: r1=>r2 then r1_inv=>r2_inv and Exists_r1=>Exists_r2
                    if for_tbi {
                        format!(
                            "{}: r{}s {} r^-{}s^-, {}r{}{}s ",
                            self.identifier(),
                            UNICODE_SUBSETEQ,
                            UNICODE_RIGHTARROW,
                            UNICODE_SUBSETEQ,
                            UNICODE_EXISTS,
                            UNICODE_SQSUBSETEQ,
                            UNICODE_EXISTS
                        )
                    } else {
                        String::from("R8: NONE")
                    }
                }
            }
        }
    }

    /// cast rule to String, as "Rn" where n is the
    /// number of the rule
    pub fn identifier(&self) -> String {
        format!("R{}", &self.to_usize())
    }
}

// such that Ord works
impl PartialOrd for CR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_usize().partial_cmp(&other.to_usize())
    }
}

/// We need partial ordering to establish an order when applying
/// rules.
impl Ord for CR {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

// pretty printer for counter rules
impl Display for CR {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CR::Zero => write!(f, "zero"),
            CR::First => write!(f, "first"),
            CR::Second => write!(f, "second"),
            CR::Third => write!(f, "third"),
            CR::Fourth => write!(f, "fourth"),
            CR::Fifth => write!(f, "fifth"),
            CR::Sixth => write!(f, "sixth"),
            CR::Seventh => write!(f, "seventh"),
            CR::Eight => write!(f, "eight"),
        }
    }
}

/// Type for files that contains ontologies, for the moment only NATIVE
/// is really operational
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum FileType {
    Json,
    Native,
    // Xml to come
}

/// Classify assertions in an ABox in three types, clean assertions, not
/// conflicting with any other assertions, conflicting assertions.
/// Assertions having conflict with some other assertion and self-conflict
/// assertions which are plain erroneous.
/// This help the ranking algorithm to shrink the matrix and avoid unneeded
/// calculations.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum ConflictType {
    Clean,
    Conflict,
    SelfConflict,
}
