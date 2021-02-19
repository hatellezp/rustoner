use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::rule::AbRule;
use crate::dl_lite::tbox_item::TBI;

use crate::dl_lite::types::DLType;
use std::cmp::Ordering;
use crate::dl_lite::abox_item::{ABI, Side};
use std::hash::{Hash, Hasher};

/*
// help enum for the match function in the ABIQ implementation
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Side {
    None,
    Left,
    Right,
}

 */

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(PartialEq, Debug, Clone)]
pub struct ABIQ {
    abi: ABI, // role or concept assertion
    prevalue: f64,
    value: Option<f64>,
}

impl Eq for ABIQ {}

// TODO: is this enough ????
impl Hash for ABIQ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.abi.hash(state);
    }
}

impl fmt::Display for ABIQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abi)
    }
}

impl PartialOrd for ABIQ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.abi.partial_cmp(&other.abi)

         /*
        if self == other {
            Some(Ordering::Equal)
        } else {
            match self {
                ABIQ::CA(c1, a1, _, _) => {
                    match other {
                        ABIQ::RA(_, _, _, _, _) => Some(Ordering::Less),
                        ABIQ::CA(c2, a2, _, _) => {
                            match c1.cmp(c2) {
                                Ordering::Equal => {
                                    Some(a1.cmp(a2))
                                },
                                _ => Some(c1.cmp(c2))
                            }
                        },
                    }
                },
                ABIQ::RA(r1, a1, b1, _, _) => {
                    match other {
                        ABIQ::CA(_, _, _, _) => Some(Ordering::Greater),
                        ABIQ::RA(r2, a2, b2, _, _) => {
                            match r1.cmp(r2) {
                                Ordering::Equal => {
                                    match a1.cmp(a2) {
                                        Ordering::Equal => Some(b1.cmp(b2)),
                                        _ => Some(a1.cmp((a2)))
                                    }
                                },
                                _ => Some(r1.cmp(r2))
                            }
                        },
                    }
                },
            }
        }

          */
    }
}

impl Ord for ABIQ {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

/*
the language only allows for base concept and role assertions,
here we however, we will allow for negation of and other complex constructions
this will allow for finding that 'a doesn't belong to A'
 */

impl ABIQ {
    pub fn new(abi: ABI, prevalue: Option<f64>, value: Option<f64>) -> ABIQ {
        let prevalue = match prevalue {
            Some(pv) => pv,
            _ => 1.0
        };

        ABIQ { abi, prevalue, value }
    }

    /*
    pub fn new_ra(r: Node, a: Node, b: Node, for_completion: bool, prevalue: Option<f64>, value: Option<f64>) -> Option<ABIQ> {
        let is_base_role = r.t() == DLType::BaseRole || for_completion;
        let all_nominals = DLType::all_nominals(a.t(), b.t());

        if !is_base_role || !all_nominals {
            Option::None
        } else {
            let prevalue = match prevalue {
                Some(pv) => pv,
                _ => 1.0
            };

            Some(ABIQ::RA(r, a, b, prevalue))
        }
    }

    pub fn new_ca(c: Node, a: Node, for_completion: bool) -> Option<ABIQ> {
        let is_base_concept = c.t() == DLType::BaseConcept || for_completion;
        let is_nominal = a.t() == DLType::Nominal;
        if !is_base_concept || !is_nominal {
            Option::None
        } else {
            Some(ABIQ::CA(c, a))
        }
    }

     */

    pub fn abi(&self) -> &ABI {
        &self.abi
    }

    pub fn prevalue(&self) -> f64 {
        self.prevalue
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn is_trivial(&self) -> bool {
        self.abi.is_trivial()
    }

    pub fn t(&self) -> DLType {
        self.abi.t()
    }

    // reference to the concept or role in the abox_item
    pub fn symbol(&self) -> &Node {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        self.abi.symbol()
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
        self.abi.nominal(position)
    }

    /*
    pub fn decompact(self) -> (Node, Node, Option<Node>) {
        match self {
            ABIQ::RA(r, a, b) => (r, a, Some(b)),
            ABIQ::CA(c, a) => (c, a, Option::None),
        }
    }

    pub fn decompact_with_clone(&self) -> (Node, Node, Option<Node>) {
        let new_self = self.clone();
        new_self.decompact()
    }

     */

    pub fn is_match(&self, tbi: &TBI) -> Side {
        self.abi.is_match(tbi)
        /*
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
    */
    }

    pub fn get_abis(abiqs: Vec<&ABIQ>) -> Vec<&ABI> {
        let abis: Vec<&ABI> = abiqs.iter().map(|&x| x.abi()).collect::<Vec<_>>();

        abis
    }

    // pub fn apply_two(one: &ABIQ, two: &ABIQ, tbox: &TB) -> Option<Vec<ABIQ>> {}
    pub fn apply_rule(abis: Vec<&ABIQ>, tbis: Vec<&TBI>, rule: &AbRule) -> Option<Vec<ABIQ>> {
        let prov_vec = match tbis.len() {
            1 => rule(ABIQ::get_abis(abis), tbis),
            2 => rule(ABIQ::get_abis(abis), tbis),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec: Vec<ABI> = prov_vec.unwrap();
            let mut final_vec: Vec<ABIQ> = Vec::new();

            for item in &prov_vec {
                // println!("trying to add: {}", item);

                if !item.is_trivial() {
                    // println!("    success");
                    let abiq = ABIQ::new(item.clone(), Option::None, Option::None);

                    final_vec.push(abiq);
                }
            }

            Some(final_vec)
        }
    }
}
