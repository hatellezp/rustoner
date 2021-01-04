use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::tbox::TB;
use crate::dl_lite::tbox_item::TBI;
use crate::dl_lite::types::DLType;

// help enum for the match function in the ABI implementation
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Side {
    None,
    Left,
    Right,
}

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum ABI {
    RA(Node, Node, Node), // role assertion
    CA(Node, Node),       // concept assertion
}

impl fmt::Display for ABI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ABI::RA(r, a, b) => write!(f, "<{}: {}, {}>", r, a, b),
            ABI::CA(c, a) => write!(f, "<{}: {}>", c, a),
        }
    }
}

impl ABI {
    pub fn new_ra(r: Node, a: Node, b: Node) -> Option<ABI> {
        let is_base_role = r.t() == DLType::BaseRole;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            Some(ABI::RA(r, a, b))
        }
    }

    pub fn new_ca(c: Node, a: Node) -> Option<ABI> {
        let is_base_concept = c.t() == DLType::BaseConcept;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(ABI::CA(c, a))
        }
    }

    pub fn t(&self) -> DLType {
        match self {
            ABI::RA(_, _, _) => DLType::BaseRole,
            ABI::CA(_, _) => DLType::BaseConcept,
        }
    }

    // reference to the concept or role in the abox_item
    pub fn symbol(&self) -> &Node {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        match self {
            ABI::RA(r, _, _) => r,
            ABI::CA(c, _) => c,
        }
    }

    pub fn nominal(&self, position: usize) -> Option<&Node> {
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
            ABI::RA(_, a, b) => match position {
                0 => Some(a),
                1 => Some(b),
                _ => Option::None,
            },
            ABI::CA(_, a) => match position {
                0 => Some(a),
                _ => Option::None,
            },
        }
    }

    pub fn decompact(self) -> (Node, Node, Option<Node>) {
        match self {
            ABI::RA(r, a, b) => (r, a, Some(b)),
            ABI::CA(c, a) => (c, a, Option::None),
        }
    }

    pub fn decompact_with_clone(&self) -> (Node, Node, Option<Node>) {
        let new_self = self.clone();
        new_self.decompact()
    }

    pub fn is_match(&self, tbi: &TBI) -> Side {
        // because tbox_item(s) are well formed, you only need to test against one
        let all_roles = DLType::all_roles(tbi.lside().t(), self.t());
        let all_concepts = DLType::all_concepts(tbi.lside().t(), self.t());

        if !all_roles && !all_concepts {
            Side::None
        } else {
            let sym = self.symbol();
            let left = sym == tbi.lside();
            let right = sym == tbi.rside();

            if left {
                Side::Left
            } else if right {
                Side::Right
            } else {
                Side::None
            }
        }
    }

    // this is more complex than what I think
    pub fn apply_one(one: &ABI, tbox: &TB) -> Option<Vec<ABI>> {
        let mut abox_items: Vec<ABI> = Vec::new();
        for tbi in tbox.items() {
            if one.is_match(tbi) == Side::Left {
                // create new abi
                match one.symbol().t() {
                    DLType::BaseRole => {
                        let r = one.symbol().clone();
                        let a = one.nominal(0).unwrap().clone();
                        let b = one.nominal(0).unwrap().clone();

                        let new_abi = ABI::new_ra(r, a, b).unwrap();

                        if !abox_items.contains(&new_abi) {
                            abox_items.push(new_abi);
                        }
                    }
                    DLType::BaseConcept => {
                        let c = one.symbol().clone();
                        let a = one.nominal(0).unwrap().clone();

                        let new_abi = ABI::new_ca(c, a).unwrap();

                        if !abox_items.contains(&new_abi) {
                            abox_items.push(new_abi);
                        }
                    }
                    _ => (),
                }
            }
        }

        if abox_items.len() == 0 {
            Option::None
        } else {
            Some(abox_items)
        }
    }

    // pub fn apply_two(one: &ABI, two: &ABI, tbox: &TB) -> Option<Vec<ABI>> {}
}
