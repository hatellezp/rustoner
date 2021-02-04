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
    pub fn is_nominal_type(&self) -> bool {
        match self {
            DLType::Nominal => true,
            _ => false,
        }
    }

    pub fn is_role_type(&self) -> bool {
        match self {
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => true,
            _ => false,
        }
    }

    pub fn is_concept_type(&self) -> bool {
        match self {
            DLType::Bottom
            | DLType::Top
            | DLType::BaseConcept
            | DLType::ExistsConcept
            | DLType::NegatedConcept => true,
            _ => false,
        }
    }

    pub fn all_roles(t1: DLType, t2: DLType) -> bool {
        match t1 {
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => match t2 {
                DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => true,
                _ => false,
            },
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
            | DLType::NegatedConcept => match t2 {
                DLType::Bottom
                | DLType::Top
                | DLType::BaseConcept
                | DLType::ExistsConcept
                | DLType::NegatedConcept => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn all_nominals(t1: DLType, t2: DLType) -> bool {
        t1 == DLType::Nominal && t2 == DLType::Nominal
    }

    pub fn same_type(t1: DLType, t2: DLType) -> bool {
        DLType::all_roles(t1, t2) || DLType::all_concepts(t1, t2) || DLType::all_nominals(t1, t2)
    }

    pub fn for_db(&self) -> String {
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

        let res =String::from(res);
        res
    }
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
            CR::Seventh => write!(f, "sevent"),
            CR::Eight => write!(f, "eight"),
        }
    }
}
