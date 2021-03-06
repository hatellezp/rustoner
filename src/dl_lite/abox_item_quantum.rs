use std::fmt;

use crate::dl_lite::node::Node_DLlite;
use crate::kb::knowledge_base::AbRule;
use crate::dl_lite::tbox_item::TBI_DLlite;

use crate::dl_lite::abox_item::{Side, ABI_DLlite};
use crate::kb::types::DLType;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use crate::kb::knowledge_base::ABoxItem;

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(PartialEq, Debug, Clone)]
pub struct ABIQ_DLlite {
    abi: ABI_DLlite, // role or concept assertion
    prevalue: f64,
    value: Option<f64>,
}

impl Eq for ABIQ_DLlite {}

// TODO: is this enough ????
impl Hash for ABIQ_DLlite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.abi.hash(state);
    }
}

impl fmt::Display for ABIQ_DLlite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {:?})", self.abi, self.prevalue, self.value)
    }
}

impl PartialOrd for ABIQ_DLlite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.abi.partial_cmp(&other.abi)
    }
}

impl Ord for ABIQ_DLlite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl ABoxItem for ABIQ_DLlite {
    type NodeItem = Node_DLlite;

    fn negate(&self) -> Self {
        let abi_neg = self.abi.negate();

        ABIQ_DLlite::new(abi_neg, Some(self.prevalue), self.value)
    }

    fn t(&self) -> DLType {
        self.abi.t()
    }

    fn item(&self) -> &Node_DLlite {
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

impl ABIQ_DLlite {
    pub fn new(abi: ABI_DLlite, prevalue: Option<f64>, value: Option<f64>) -> ABIQ_DLlite {
        let prevalue = match prevalue {
            Some(pv) => pv,
            _ => 1.0,
        };

        ABIQ_DLlite {
            abi,
            prevalue,
            value,
        }
    }

    pub fn abi(&self) -> &ABI_DLlite {
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


    pub fn nominal(&self, position: usize) -> Option<&Node_DLlite> {
        self.abi.nominal(position)
    }

    pub fn same_nominal(&self, other: &Self) -> bool {
        self.abi.same_nominal(&other.abi)
    }

    pub fn is_match(&self, tbi: &TBI_DLlite) -> Vec<Side> {
        self.abi.is_match(tbi)
    }

    pub fn get_abis(abiqs: Vec<&ABIQ_DLlite>) -> Vec<&ABI_DLlite> {
        let abis: Vec<&ABI_DLlite> = abiqs.iter().map(|&x| x.abi()).collect::<Vec<_>>();

        abis
    }

    // pub fn apply_two(one: &ABIQ, two: &ABIQ, tbox: &TB) -> Option<Vec<ABIQ>> {}
    pub fn apply_rule(abiqs: Vec<&ABIQ_DLlite>, tbis: Vec<&TBI_DLlite>, rule: &AbRule<TBI_DLlite, ABIQ_DLlite>) -> Option<Vec<ABIQ_DLlite>> {
        let prov_vec = match tbis.len() {
            1 => rule(abiqs, tbis),
            2 => rule(abiqs, tbis),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec: Vec<ABIQ_DLlite> = prov_vec.unwrap();
            let mut final_vec: Vec<ABIQ_DLlite> = Vec::new();

            for item in &prov_vec {
                if !item.abi().is_trivial() {
                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }
}
