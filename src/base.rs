use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::hash::{Hash, Hasher};
// use crate::base::DLType::{InverseRole, NegatedConcept};

/*
a base item, these should be the only ones that are not
references, they posses their type implicitly by the enum
and a name, that is an integer,
remember that '0' is always reserved for bottom
 */
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum BI {
    // BI stand for Base Item
    Role(usize),
    Concept(usize),
    Nominal(usize),
}

/*
all more complex items: CI, CI, TBoxItem, ABoxItem are built as references, so they
point somewhere, this is the function of the Context, all base items are reserved here and
CI, TBoxItem and ABoxItem reference BIs in a context
 */
#[derive(PartialEq, Eq, Debug)]
pub struct Context {
    bag: HashSet<BI>,
}

// =================================================================================================

/*
DLType allows for easy of comparison and construction of more complex items, also ensures that
the syntax is respected
 */
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

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Negated {
    T,
    F,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum Inverted {
    T,
    F,
}

/*
this is more a middle object, all more complex items have two parts, a context to reference to
and an specification of how the item is built, this is the function of CI, it stores it its
construction the building specification of the item
 */
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub enum CI<'a> {
    Bottom(&'a Context, Negated),
    BC(&'a Context, &'a BI, Negated),           // base concept
    BR(&'a Context, &'a BI, Negated, Inverted), // base role
    EC(&'a Context, &'a BI, Negated, Inverted), // exists concept
} // I'm testing consomming the references TODO: I'm testing this

// =================================================================================================
// here are implementation for known traits

impl Hash for Context {
    /*
    I needed this, as deriving Hash automatically was not posssible for the rust compiler
     */
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut counter: usize = 0;

        for b in &self.bag {
            b.hash(state);
            counter.hash(state);
            counter = counter + 1;
        }
    }
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
impl fmt::Display for BI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BI::Role(n) => write!(f, "r({})", n),
            BI::Concept(n) => write!(f, "c({})", n),
            BI::Nominal(n) => write!(f, "n({})", n),
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::from("[");
        for b in &self.bag {
            s.push_str(b.to_string().as_str());
            s.push_str(", ");
        }

        let l = s.len();
        s = (s.as_str())[0..(l - 2)].to_string(); // ugly ass transformation :(
        s.push_str("]");
        write!(f, "{}", s)
    }
}

impl<'a> fmt::Display for CI<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}", self.ci)
        match self {
            CI::Bottom(_, _) => write!(f, "<BOTTOM>"),
            // CI::Top(_) => write!(f, "<TOP>"),
            CI::BR(_, bi, Negated::F, Inverted::F) => write!(f, "{}", *bi),
            CI::BR(_, bi, Negated::F, Inverted::T) => write!(f, "{}^-", *bi),
            CI::BR(_, bi, Negated::T, Inverted::F) => write!(f, "-{}", *bi),
            CI::BR(_, bi, Negated::T, Inverted::T) => write!(f, "-({}^-)", *bi),
            CI::BC(_, bi, Negated::F) => write!(f, "{}", *bi),
            CI::BC(_, bi, Negated::T) => write!(f, "-{}", *bi),
            CI::EC(_, bi, Negated::F, Inverted::F) => write!(f, "E[{}]", *bi),
            CI::EC(_, bi, Negated::F, Inverted::T) => write!(f, "E[{}^-]", *bi),
            CI::EC(_, bi, Negated::T, Inverted::F) => write!(f, "-E[{}]", *bi),
            CI::EC(_, bi, Negated::T, Inverted::T) => write!(f, "-E[{}^-]", *bi),
        }
    }
}

// =================================================================================================
// here my implementations

impl Negated {
    pub fn other(&self) -> Negated {
        match self {
            Negated::F => Negated::T,
            Negated::T => Negated::F,
        }
    }
}

impl Inverted {
    pub fn other(&self) -> Inverted {
        match self {
            Inverted::F => Inverted::T,
            Inverted::T => Inverted::F,
        }
    }
}

impl DLType {
    pub fn all_roles(t1: DLType, t2: DLType) -> bool {
        match t1 {
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => match t2 {
                DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn all_concepts(t1: DLType, t2: DLType) -> bool {
        match t1 {
            DLType::BaseConcept | DLType::ExistsConcept | DLType::NegatedConcept => match t2 {
                DLType::BaseConcept | DLType::ExistsConcept | DLType::NegatedConcept => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn same_type(t1: DLType, t2: DLType) -> bool {
        DLType::all_roles(t1, t2) || DLType::all_concepts(t1, t2)
    }
}

impl BI {
    pub fn new(n: usize, t: DLType) -> Option<BI> {
        /*
        ensures that new BI objects are well defined
         */
        match (t, n) {
            (_, 0) => Option::None, // zero is reserved for Bottom!
            (DLType::BaseRole, _) => Some(BI::Role(n)),
            (DLType::BaseConcept, _) => Some(BI::Concept(n)),
            (DLType::Nominal, _) => Some(BI::Nominal(n)),
            (_, _) => Option::None,
        }
    }

    pub fn n(&self) -> usize {
        /*
        retrieve the name of the concept, simply that
         */
        match self {
            BI::Role(n) | BI::Concept(n) | BI::Nominal(n) => *n,
        }
    }

    pub fn t(&self) -> DLType {
        match self {
            BI::Role(_) => DLType::BaseRole,
            BI::Concept(_) => DLType::BaseConcept,
            BI::Nominal(_) => DLType::Nominal,
        }
    }
}

impl Context {
    // hardcore code here, I for once don't like using generics...
    pub fn new<I>(it: I) -> Context
    where
        I: Iterator<Item = BI>,
    {
        let mut bag: HashSet<BI> = HashSet::new();

        for item in it {
            bag.insert(item);
        }

        Context { bag }
    }

    pub fn is_sub_context(&self, other: &Context) -> bool {
        /*
        returns true if all BI objects in self appears also in other
         */
        for b in &self.bag {
            if !other.bag.contains(b) {
                return false; // the return instruction in rust of course terminates the function
            }
        }
        true
    }

    /*
    simply verify that the bag on self contains the BI bi
     */
    pub fn contains(&self, bi: &BI) -> bool {
        self.bag.contains(bi)
    }
}

impl<'a> CI<'a> {
    pub fn context(&'a self) -> &'a Context {
        match self {
            // CI::Bottom(c) |
            CI::Bottom(c, _) | CI::BR(c, _, _, _) | CI::BC(c, _, _) | CI::EC(c, _, _, _) => *c,
        }
    }

    pub fn new_bottom(context: &'a Context) -> CI<'a> {
        CI::Bottom(context, Negated::F)
    }
    pub fn new_top(context: &'a Context) -> CI<'a> {
        CI::Bottom(context, Negated::T)
    }

    pub fn new_from_base<'b>(context: &'a Context, bi: &'b BI, t: DLType) -> Option<CI<'a>> {
        /*
        builds a CI from a BI object, we return an Option because if the item is not well
        built then None is returned
         */

        // first verify that bi is a valid baseitem name
        for item in &context.bag {
            if item == bi {
                return match (item, t) {
                    (BI::Role(_), DLType::BaseRole) | (BI::Role(_), DLType::InverseRole) => {
                        Some(CI::BR(context, item, Negated::F, Inverted::F))
                    }
                    (BI::Concept(_), DLType::BaseConcept) => {
                        Some(CI::BC(context, item, Negated::F))
                    }
                    _ => Option::None,
                };
            }
        }
        Option::None
    }

    // when building this, normally the name is valid
    // TODO: test this !!!
    pub fn new_from_complex(ci: CI<'a>, t: DLType) -> Option<CI<'a>> {
        match (ci, t) {
            (CI::BR(c, bi, Negated::F, inverted), DLType::InverseRole) => {
                Some(CI::BR(c, bi, Negated::F, inverted.other()))
            }
            (CI::BR(c, bi, Negated::F, inverted), DLType::ExistsConcept) => {
                Some(CI::EC(c, bi, Negated::F, inverted))
            }
            (CI::BR(c, bi, negated, inverted), DLType::NegatedRole) => {
                Some(CI::BR(c, bi, negated.other(), inverted))
            }
            (CI::BC(c, bi, negated), DLType::NegatedConcept) => {
                Some(CI::BC(c, bi, negated.other()))
            }
            (CI::EC(c, bi, negated, inverted), DLType::NegatedConcept) => {
                Some(CI::EC(c, bi, negated.other(), inverted))
            }
            _ => Option::None,
        }
    }

    pub fn clone_with_context<'b>(&'b self, context: &'a Context) -> Option<CI<'a>> {
        match self {
            CI::Bottom(_, negated) => Some(CI::Bottom(context, *negated)),
            CI::BR(_, bi, negated, inverted) => {
                if context.contains(bi) {
                    Some(CI::BR(context, bi, *negated, *inverted))
                } else {
                    Option::None
                }
            }
            CI::BC(_, bi, negated) => {
                if context.contains(bi) {
                    Some(CI::BC(context, bi, *negated))
                } else {
                    Option::None
                }
            }
            CI::EC(_, bi, negated, inverted) => {
                if context.contains(bi) {
                    Some(CI::EC(context, bi, *negated, *inverted))
                } else {
                    Option::None
                }
            }
        }
    }

    pub fn t(&self) -> DLType {
        /*
        retrieve the type of the item
         */
        match self {
            CI::Bottom(_, Negated::F) => DLType::Bottom,
            CI::Bottom(_, Negated::T) => DLType::Top,
            // CI::Top(_) => DLType::Top,
            CI::BR(_, _, Negated::F, Inverted::F) => DLType::BaseRole,
            CI::BR(_, _, Negated::F, Inverted::T) => DLType::InverseRole,
            CI::BR(_, _, Negated::T, _) => DLType::NegatedRole,
            CI::EC(_, _, _, _) => DLType::ExistsConcept,
            CI::BC(_, _, Negated::F) => DLType::BaseConcept,
            CI::BC(_, _, Negated::T) => DLType::NegatedConcept,
        }
    }

    pub fn n(&self) -> usize {
        /*
        retrieve the name of the item
         */
        match self {
            CI::Bottom(_, _) => 0,
            CI::BR(_, bi, _, _) | CI::BC(_, bi, _) | CI::EC(_, bi, _, _) => bi.n(),
        }
    }

    /*
    pub fn negate_with_copy(&'a self) -> Option<CI<'a>> {
       let new_self = self.clone();
        match new_self.t() {
            DLType::Bottom => Some(CI::new_top(new_self.context())),
            DLType::Top => Some(CI::new_bottom(new_self.context())),
            DLType::BaseRole | DLType::InverseRole | DLType::NegatedRole => CI::new_from_complex(&new_self, DLType::NegatedRole),
            DLType::BaseConcept | DLType::ExistsConcept | DLType::NegatedConcept => CI::new_from_complex(&new_self, DLType::NegatedConcept),
            _ => Option::None,
        }
    }
     */

    pub fn negate<'b>(&'b self, context: &'a Context) -> Option<CI<'a>> {
        /*
        this functions negate (whenever possible) a ci, that is is self can be negated,
        then returns such negation,
        OBS: it should to take item for itself
         */
        match self {
            CI::Bottom(_, negated) => Some(CI::Bottom(context, negated.other())),
            // CI::Top(_) => Some(CI::new_bottom(self.context())),
            CI::BR(_, bi, negated, inverted) => {
                if context.contains(bi) {
                    Some(CI::BR(context, bi, negated.other(), *inverted))
                } else {
                    Option::None
                }
            }
            CI::EC(_, bi, negated, inverted) => {
                if context.contains(bi) {
                    Some(CI::EC(context, bi, negated.other(), *inverted))
                } else {
                    Option::None
                }
            }
            CI::BC(_, bi, negated) => {
                if context.contains(bi) {
                    Some(CI::BC(context, bi, negated.other()))
                } else {
                    Option::None
                }
            }
        }
    }

    pub fn is_negated(&self) -> bool {
        /*
        verify that the item is one the negated types: NegatedRole or NegatedConcept
         */
        match self {
            CI::Bottom(_, Negated::T) | CI::BR(_, _, Negated::T, _) | CI::BC(_, _, Negated::T) => {
                true
            }
            _ => false,
        }
    }

    fn _is_negation(&'a self, other: &CI<'a>) -> bool {
        /*
        verify that self is the negation of other, that is self must be in negated form,
        other cannot be in negated form and the child of self must be equal to other

        this function is auxiliary to is_negation that verify both possibilities
         */

        // maybe verify context too ?
        match (self, other) {
            (CI::Bottom(_, negated), CI::Bottom(_, negated2)) => *negated == negated2.other(),
            (CI::BR(_, bi, negated, inverted), CI::BR(_, bi2, negated2, inverted2)) => {
                bi == bi2 && inverted == inverted2 && *negated == negated2.other()
            }
            (CI::BC(_, bi, negated), CI::BC(_, bi2, negated2)) => {
                bi == bi2 && *negated == negated2.other()
            }
            _ => false,
        }
    }

    pub fn is_negation(&'a self, other: &CI<'a>) -> bool {
        /*
        verify that (not self) = other, one way or another,
         */
        self._is_negation(other) || other._is_negation(self)
    }

    pub fn inverse<'b>(&'b self, context: &'a Context) -> Option<CI<'a>> {
        match self {
            CI::BR(_, bi, Negated::F, inverted) => {
                if context.contains(bi) {
                    Some(CI::BR(context, bi, Negated::F, inverted.other()))
                } else {
                    Option::None
                }
            }
            _ => Option::None,
        }
    }

    pub fn is_inverted(&'a self) -> bool {
        self.t() == DLType::InverseRole
    }

    fn _is_inverse(&'a self, other: &CI<'a>) -> bool {
        match (self, other) {
            (CI::BR(_, bi, Negated::F, inverted), CI::BR(_, bi2, Negated::F, inverted2)) => {
                bi == bi2 && *inverted == inverted2.other()
            }
            _ => false,
        }
    }

    pub fn is_inverse(&'a self, other: &CI<'a>) -> bool {
        self._is_inverse(other) || other._is_inverse(self)
    }

    pub fn is_valid(&self, context: &'a Context) -> bool {
        match self {
            CI::Bottom(_, _) => true,
            CI::BR(_, bi, _, _) | CI::BC(_, bi, _) => context.contains(bi),
            _ => false,
        }
    }
}
