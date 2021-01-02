use crate::item::DLType::{UNDEFINED, RoleBase, RoleInverse, RoleNegated, ConceptBottom, ConceptBase, ConceptExists, ConceptNegated, Constant};
use std::fmt;

//--------------------------------------
//----enums and structs-----------------
//--------------------------------------
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum DLType {
    UNDEFINED,
    ConceptBottom,
    ConceptBase,
    ConceptExists,
    ConceptNegated,
    RoleBase,
    RoleInverse,
    RoleNegated,
    Constant,
}

// test for private struct ?
// to optimize space and speed, names are given by a number, no string whatsoever
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Role {
    n: usize,  // TODO: later change this to a more bounded type ? or dynamic bounding ?
    t: DLType,
    c: Option<Box<Role>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Concept {
    n: usize,
    t: DLType,
    r: Option<Box<Role>>,
    c: Option<Box<Concept>>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Nominal {
    n: usize,
    t: DLType,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Item {
    R(Role),
    C(Concept),
    N(Nominal),
}

//--------------------------------------
//---implementations--------------------
//--------------------------------------

impl fmt::Display for DLType {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            UNDEFINED => write!(f, "UNDEFINED"),
            ConceptBottom => write!(f, "ConceptBottom"),
            ConceptBase => write!(f, "ConceptBase"),
            ConceptExists => write!(f, "ConceptExists"),
            ConceptNegated => write!(f, "ConceptNegated"),
            RoleBase => write!(f, "RoleBase"),
            RoleInverse => write!(f, "RoleInverse"),
            RoleNegated => write!(f, "RoleNegated"),
            Constant => write!(f, "Constant"),
        }
    }
}

impl DLType {
    pub fn is_role_type(&self) -> bool {
        // TODO: maybe put roles as a static constant?
        let roles: [DLType; 3] = [RoleBase, RoleInverse, RoleNegated];

        roles.contains(self)
    }

    pub fn is_concept_type(&self) -> bool {
        // TODO: maybe put concepts as a static constant?
        let concepts:  [DLType; 4] = [ConceptBottom, ConceptBase, ConceptExists, ConceptNegated];

        concepts.contains(self)
    }

    pub fn is_constant_type(&self) -> bool {
        *self == Constant
    }

    pub fn same_kind(&self, other: &DLType) -> bool {
        let r = self.is_role_type() && other.is_role_type();
        let c = self.is_concept_type() && other.is_concept_type();
        let i = self.is_constant_type() && other.is_constant_type();

        r || c || i
    }

    pub fn is_negated(&self) -> bool {
        let negs: [DLType; 2] = [RoleNegated, ConceptNegated];

        negs.contains(self)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match (*self).t {
            RoleBase => write!(f, "({})", self.n),
            RoleInverse => write!(f, "({}^-)", ((*self).c).as_ref().unwrap().n),
            RoleNegated => write!(f, "(-{})", ((*self).c).as_ref().unwrap().n),
            _ => panic!("Not a role type, given type: {}", (*self).t), // I won't write, I will panic! write!(f, "not a role type !!!")
        }
    }
}

// not only for role, but verify, whenever you can, that your system is well built
// now I have to play with Option types :/
impl Role {
    pub fn n(&self) -> &usize {
        &((*self).n)
    }

    pub fn t(&self) -> &DLType {
        &((*self).t)
    }

    pub fn c(&self) -> &Option<Box<Role>> {
        &((*self).c)
    }

    pub fn unwrap_type(r: Option<Box<Role>>) -> DLType {
        (*(*(r.as_ref().unwrap()))).t
    }
    // the instantiating must be flawless, the function will not correct your errors
    // in any case, the instantiation of a new role ensures that the role is well built
    pub fn new(n: usize, t: DLType, c: Option<Box<Role>>) -> Option<Role>  {
        if t.is_role_type() {
           match t {
               RoleBase => {
                   if c.is_none() {
                       Some(Role { n, t, c, })
                   } else {
                       Option::None
                   }
               },
               RoleInverse => {
                   if c.is_some() {
                       match (*(*(c.as_ref().unwrap()))).t { // this is horrible !!!
                           RoleBase => Some(Role { n:0, t, c, }),
                           RoleInverse => Some(*(*c.unwrap()).c.unwrap()), // TODO: read this again
                           _ => Option::None
                       }
                   } else {
                       Option::None
                   }
               }
               RoleNegated => {
                   if c.is_some() {
                       match (*(*(c.as_ref().unwrap()))).t {
                           RoleNegated => Some(*(*c.unwrap()).c.unwrap()), // TODO: read this again
                           RoleBase | RoleInverse => Some(Role { n:0, t, c, }), // names only for base
                           _ => Option::None,
                       }
                   } else {
                       Option::None
                   }
               }
               _ => Option::None
           }
        } else {
            Option::None
        }
    }

    pub fn neg(self) -> Option<Role> {
        if self.t.is_role_type() {
            match self.t {
                RoleNegated => Some(*(self.c.unwrap())),
                _ => Role::new(0, RoleNegated, Some(Box::new(self))),
            }
        } else {
            // I will panic, this should not arrive, ever
            panic!("Not a role type for a role structure, given type: {}", self.t);
        }
    }

    // negations are always on top
    pub fn is_neg(&self, other: &Role) -> bool {
        match ((*self).t, (*other).t) {
            (RoleBase, RoleNegated) => {
                (*(*(*other).c.as_ref().unwrap()) == *self) || (*other).is_neg(self) // is this the good form ?
            },
            (RoleInverse, RoleNegated) => {
                (*(*(*other).c.as_ref().unwrap()) == *self) || (*other).is_neg(self) // I hope this works
            },
            _ => false
        }
    }

    pub fn inverse(self) -> Option<Role> {
        if self.t.is_role_type() {
            match self.t {
                RoleBase => Role::new(0, RoleInverse, Some(Box::new(self))), // no name unless base
                RoleInverse => {
                    Some(*(self.c.unwrap()))
                },
                _ => Option::None, // the RoleNegated type can't be inversed
            }
        } else {
            panic!("Not a role type, given type: {}", self.t);
        }

    }

    pub fn is_inverse(&self, other: &Role) -> bool {
        match ((*self).t, (*other).t) {
            (RoleBase, RoleInverse) => {
                (*(*(*other).c.as_ref().unwrap()) == *self) || (*other).is_inverse(self) // is this the good form ?
            },
            _ => false
        }
    }
}

impl fmt::Display for Concept {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match (*self).t {
            ConceptBottom => write!(f, "BOTTOM"),
            ConceptBase => write!(f, "[{}]", (*self).n),
            ConceptExists => write!(f, "E{}", *(*self).r.as_ref().unwrap()),
            ConceptNegated => write!(f, "N{}", *(*self).c.as_ref().unwrap()),
            _ => panic!("Not a concept type, given type: {}", (*self).t),
        }
    }
}

impl Concept {
    pub fn n(&self) -> &usize {
        &(*self).n
    }

    pub fn t(&self) -> &DLType {
        &(*self).t
    }

    pub fn r(&self) -> &Option<Box<Role>> {
        &(*self).r
    }

    pub fn c(&self) -> &Option<Box<Concept>> {
        &(*self).c
    }

    pub fn new(n: usize, t: DLType, r: Option<Box<Role>>, c: Option<Box<Concept>>) -> Option<Concept> {
        if t.is_concept_type() {
            match t {
                ConceptBottom => {
                    if r.is_none() && c.is_none() {
                        Some(Concept { n:0, t, r, c }) // there is the small difference bottom doesn't take a name
                    } else {
                        Option::None
                    }
                },
                ConceptBase => {
                    if r.is_none() && c.is_none() {
                        Some(Concept { n, t, r, c })
                    } else {
                        Option::None
                    }
                },
                ConceptExists => {
                    if c.is_none() && r.is_some() {
                        match (*(r.as_ref().unwrap())).t {
                            RoleBase | RoleInverse => Some(Concept { n:0, t, r, c }),
                            _ => Option::None,
                        }
                    } else {
                        Option::None
                    }
                },
                ConceptNegated => {
                    if r.is_none() && c.is_some() {
                        match (*(r.as_ref().unwrap())).t {
                            ConceptNegated => Some(*((*c.unwrap()).c.unwrap())),
                            ConceptBase | ConceptExists | ConceptBottom => Some(Concept {n:0, t, r, c }),
                            _ => Option::None,
                        }
                    } else {
                        Option::None
                    }
                },
                _ => Option::None,
            }
        } else {
            Option::None
        }
    }

    pub fn neg(self) -> Option<Concept> {
        if self.t.is_concept_type() {
            match self.t {
                ConceptNegated => Some(*(self.c.unwrap())),
                _ => Some(Concept {
                    n: 0, // if a you have a child only the child name is important
                    t: ConceptNegated,
                    r: Option::None,
                    c: Some(Box::new(self)),
                }),
            }
        } else {
            panic!("Not a concept type, given type: {}", self.t);
        }
    }

    pub fn from_role(r: Role) -> Option<Concept> {
        match r.t {
            RoleBase | RoleInverse => Some(
                Concept {
                    n: 0,
                    t: ConceptExists,
                    r: Some(Box::new(r)),
                    c: Option::None,
                }),
            _ => Option::None,
        }
    }

    pub fn is_neg(&self, other: &Concept) -> bool {
        match ((*self).t, (*other).t) {
            (ConceptBase, ConceptNegated) => {
                (*(*(*other).c.as_ref().unwrap()) == *self) || (*other).is_neg(self) // is this the good form ?
            },
            (ConceptExists, ConceptNegated) => {
                (*(*(*other).c.as_ref().unwrap()) == *self) || (*other).is_neg(self) // I hope this works
            },
            _ => false,
        }
    }
}

impl fmt::Display for Nominal {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        if (*self).t.is_constant_type() {
            write!(f, "{}", (*self).n)
        } else {
            panic!("Not a constant type, given type: {}", (*self).t);
        }

    }
}

impl Nominal {
    pub fn new(n: usize, t: DLType) -> Option<Nominal> {
        match t {
            Constant => Some(Nominal { n, t}),
            _ => Option::None,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Item::C(c) => write!(f, "{}", c),
            Item::R(r) => write!(f, "{}", r),
            Item::N(n) => write!(f, "{}", n),
        }
    }
}

impl Item {
    pub fn t(&self) -> &DLType {
        match *self {
            Item::R(ref r) => r.t(),
            Item::C(ref c) => c.t(),
            Item::N(_) => &(DLType::Constant),
        }
    }

    pub fn is_type(&self, t: DLType) -> bool {
        match self {
            Item::R(r) => (*r).t == t,
            Item::C(c) => (*c).t == t,
            Item::N(n) => (*n).t == t,
        }
    }

    pub fn neg(self) -> Option<Item> {
        match self {
            Item::R(r) => {
                let not_r = r.neg();
                if not_r.is_some() {
                    Some(Item::R(not_r.unwrap()))
                } else {
                    Option::None
                }
            },
            Item::C(c) => {
                let not_c = c.neg();
                if not_c.is_some() {
                    Some(Item::C(not_c.unwrap()))
                } else {
                    Option::None
                }
            },
            _ => Option::None,
        }
    }

    // I'm not using the same pattern of negate and then compare, constants will evaluate to
    // None, I'm not sure how rust will treat this in the future, better to think for
    // future-proof code, whenever possible
    pub fn is_neg(&self, other: &Item) -> bool {
        match (self, other) {
            (Item::R(r1), Item::R(r2)) => r1.is_neg(r2),
            (Item::C(c1), Item::C(c2)) => c1.is_neg(c2),
            _ => false, //  can't deny constants
        }
    }

    pub fn inverse(self) -> Option<Item> {
        match self {
            Item::R(r) => {
                let inv_r = r.inverse();
                if inv_r.is_some() {
                    Some(Item::R(inv_r.unwrap()))
                } else {
                    Option::None
                }
            },
            _ => Option::None,
        }
    }

    pub fn is_inverse(&self, other: &Item) -> bool {
        match (self, other) {
            (Item::R(r1), Item::R(r2)) => r1.is_inverse(r2),
            _ => false,
        }
    }
}


