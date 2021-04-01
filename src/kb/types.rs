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

use crate::interface::format_constants::{
    UNICODE_BOT, UNICODE_EXISTS, UNICODE_NEG, UNICODE_RIGHTARROW, UNICODE_SQSUBSETEQ,
    UNICODE_SUBSETEQ, UNICODE_TOP,
};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;

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
    /*
    pub fn is_base_type(&self) -> bool {
        match self {
            DLType::Nominal | DLType::BaseRole | DLType::BaseConcept => true,
            _ => false,
        }
    }

     */

    pub fn is_nominal_type(&self) -> bool {
        matches!(self, DLType::Nominal)
    }

    pub fn is_role_type(&self) -> bool {
        matches!(
            self,
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole
        )
    }

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
        t1 == DLType::Nominal && t2 == DLType::Nominal
    }

    pub fn same_type(t1: DLType, t2: DLType) -> bool {
        DLType::all_roles(t1, t2) || DLType::all_concepts(t1, t2) || DLType::all_nominals(t1, t2)
    }

    /*
    pub fn types() -> [DLType; 9] {
        [
            DLType::Bottom,
            DLType::Top,
            DLType::BaseConcept,
            DLType::BaseRole,
            DLType::InverseRole,
            DLType::NegatedRole,
            DLType::ExistsConcept,
            DLType::NegatedConcept,
            DLType::Nominal,
        ]
    }

     */

    /*
    pub fn to_usize_for_db(&self) -> usize {
        match self {
            DLType::Bottom => 0,
            DLType::Top => 1,
            DLType::BaseConcept => 2,
            DLType::BaseRole => 3,
            DLType::InverseRole => 4,
            DLType::NegatedRole => 5,
            DLType::ExistsConcept => 6,
            DLType::NegatedConcept => 7,
            DLType::Nominal => 8,
        }
    }

    pub fn to_string_for_db(&self) -> String {
        let res = match self {
            DLType::Bottom => "bottom",
            DLType::Top => "top",
            DLType::BaseConcept => "baseconcept",
            DLType::BaseRole => "baserole",
            DLType::InverseRole => "inverserole",
            DLType::NegatedRole => "negatedrole",
            DLType::ExistsConcept => "existsconcept",
            DLType::NegatedConcept => "negatedconcept",
            DLType::Nominal => "nominal",
        };

        let res = String::from(res);
        res
    }

    pub fn usize_type_from_usize_for_db(id: usize) -> Option<DLType> {
        match id {
            0 => Some(DLType::Bottom),
            1 => Some(DLType::Top),
            2 => Some(DLType::BaseConcept),
            3 => Some(DLType::BaseRole),
            4 => Some(DLType::InverseRole),
            5 => Some(DLType::NegatedRole),
            6 => Some(DLType::ExistsConcept),
            7 => Some(DLType::NegatedConcept),
            8 => Some(DLType::Nominal),
            _ => Option::None,
        }
    }

     */
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CR {
    // stand for count rules
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

    // true for tbi, false for abi
    pub fn description(&self, for_tbi: bool) -> String {
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

    pub fn identifier(&self) -> String {
        format!("R{}", &self.to_usize())
    }
}

impl PartialOrd for CR {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_usize().partial_cmp(&other.to_usize())
    }
}

impl Ord for CR {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize().cmp(&other.to_usize())
    }
}

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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum FileType {
    JSON,
    NATIVE,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum ConflictType {
    Clean,
    Conflict,
    SelfConflict,
}
