use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::rule::AbRule;
use crate::dl_lite::tbox_item::TBI;

use crate::dl_lite::abox_item::{Side, ABI};
use crate::dl_lite::types::DLType;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

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
        write!(f, "{} ({}, {:?})", self.abi, self.prevalue, self.value)
    }
}

impl PartialOrd for ABIQ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.abi.partial_cmp(&other.abi)
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
            _ => 1.0,
        };

        ABIQ {
            abi,
            prevalue,
            value,
        }
    }

    pub fn negate(&self) -> ABIQ {
        let abi_neg = self.abi.negate();

        ABIQ::new(abi_neg, Some(self.prevalue), self.value)
    }

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
        self.abi.nominal(position)
    }

    pub fn same_nominal(&self, other: &Self) -> bool {
        self.abi.same_nominal(&other.abi)
    }

    pub fn is_match(&self, tbi: &TBI) -> Vec<Side> {
        self.abi.is_match(tbi)
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
                if !item.is_trivial() {
                    let abiq = ABIQ::new(item.clone(), Option::None, Option::None);
                    final_vec.push(abiq);
                }
            }

            Some(final_vec)
        }
    }
}
