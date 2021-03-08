use std::fmt;

use crate::shiq::node::Node_SHIQ;
use crate::kb::knowledge_base::AbRule;
use crate::shiq::tbox_item::TBI_SHIQ;

use crate::shiq::abox_item::{Side, ABI_SHIQ};
use crate::kb::types::DLType;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::kb::knowledge_base::ABoxItem;

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(PartialEq, Debug, Clone)]
pub struct ABIQ_SHIQ {
    abi: ABI_SHIQ, // role or concept assertion
    prevalue: f64,
    value: Option<f64>,
}

impl Eq for ABIQ_SHIQ {}

// TODO: is this enough ????
impl Hash for ABIQ_SHIQ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.abi.hash(state);
    }
}

impl fmt::Display for ABIQ_SHIQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {:?})", self.abi, self.prevalue, self.value)
    }
}

impl PartialOrd for ABIQ_SHIQ {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.abi.partial_cmp(&other.abi)
    }
}

impl Ord for ABIQ_SHIQ {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ABoxItem for ABIQ_SHIQ {
    type NodeItem = Node_SHIQ;

    fn negate(&self) -> Self {
        let abi_neg = self.abi.negate();

        ABIQ_SHIQ::new(abi_neg, Some(self.prevalue), self.value)
    }

    fn t(&self) -> DLType {
        self.abi.t()
    }

    fn item(&self) -> &Node_SHIQ {
        /*
        returns a reference to the role or concept symbol of the  abox item
         */
        self.abi.symbol()
    }
}

/*
the language only allows for base concept and role assertions,
here we however, we will allow for negation of and other complex constructions
this will allow for finding that 'a doesn't belong to A'
 */

impl ABIQ_SHIQ {
    pub fn new(abi: ABI_SHIQ, prevalue: Option<f64>, value: Option<f64>) -> ABIQ_SHIQ {
        let prevalue = match prevalue {
            Some(pv) => pv,
            _ => 1.0,
        };

        ABIQ_SHIQ {
            abi,
            prevalue,
            value,
        }
    }

    pub fn abi(&self) -> &ABI_SHIQ {
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

    // reference to the concept or role in the abox_item


    pub fn nominal(&self, position: usize) -> Option<&Node_SHIQ> {
        self.abi.nominal(position)
    }

    pub fn same_nominal(&self, other: &Self) -> bool {
        self.abi.same_nominal(&other.abi)
    }

    pub fn is_match(&self, tbi: &TBI_SHIQ) -> Vec<Side> {
        self.abi.is_match(tbi)
    }

    pub fn get_abis(abiqs: Vec<&ABIQ_SHIQ>) -> Vec<&ABI_SHIQ> {
        let abis: Vec<&ABI_SHIQ> = abiqs.iter().map(|&x| x.abi()).collect::<Vec<_>>();

        abis
    }

    // pub fn apply_two(one: &ABIQ, two: &ABIQ, tbox: &TB) -> Option<Vec<ABIQ>> {}
    pub fn apply_rule(abiqs: Vec<&ABIQ_SHIQ>, tbis: Vec<&TBI_SHIQ>, rule: &AbRule<TBI_SHIQ, ABIQ_SHIQ>) -> Option<Vec<ABIQ_SHIQ>> {
        let prov_vec = match tbis.len() {
            1 => rule(abiqs, tbis),
            2 => rule(abiqs, tbis),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec: Vec<ABIQ_SHIQ> = prov_vec.unwrap();
            let mut final_vec: Vec<ABIQ_SHIQ> = Vec::new();

            for item in &prov_vec {
                if !item.abi().is_trivial() {
                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }
}
