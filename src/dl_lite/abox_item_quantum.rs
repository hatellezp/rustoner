use std::fmt;

use crate::dl_lite::node::Node_DLlite;
use crate::dl_lite::tbox_item::TBI_DLlite;
use crate::kb::knowledge_base::{AbRule, Implier, TBoxItem};

use crate::dl_lite::abox_item::{ABI_DLlite, Side};
use crate::kb::knowledge_base::ABoxItem;
use crate::kb::types::DLType;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(PartialEq, Debug, Clone)]
pub struct ABIQ_DLlite {
    abi: ABI_DLlite, // role or concept assertion
    prevalue: f64,
    value: Option<f64>,
    level: usize,
    impliers: Vec<(Vec<TBI_DLlite>, Vec<ABIQ_DLlite>)>,
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

impl Implier for ABIQ_DLlite {
    type Imp = (Vec<TBI_DLlite>, Vec<ABIQ_DLlite>);

    fn implied_by(&self) -> &Vec<(Vec<TBI_DLlite>, Vec<ABIQ_DLlite>)> {
        &self.impliers
    }

    fn add_to_implied_by(&mut self, implier: (Vec<TBI_DLlite>, Vec<ABIQ_DLlite>)) {
        let mut tb = implier.0;
        let mut ab = implier.1;

        // sorting always needed
        tb.sort();
        ab.sort();
        let implier = (tb, ab);

        let contains = self.contains_implier(&implier);

        match contains {
            Option::Some(Ordering::Less) => {
                let mut cmpd: Option<Ordering>;
                let mut inner_implier: &(Vec<TBI_DLlite>, Vec<ABIQ_DLlite>);
                let lenght: usize = self.impliers.len();

                for index in 0..lenght {
                    inner_implier = &self.impliers.get(index).unwrap();
                    cmpd = Self::cmp_imp(&implier, inner_implier);

                    match cmpd {
                        Option::Some(Ordering::Less) => {
                            self.impliers[index] = implier;
                            break; // rust is very smart, told me that value was being used in future iteration, so I put a break right here
                        }
                        _ => (),
                    }
                }
            }
            Option::None => self.impliers.push(implier),
            Option::Some(Ordering::Equal) | Option::Some(Ordering::Greater) => (),
        }
    }

    fn cmp_imp(imp1: &Self::Imp, imp2: &Self::Imp) -> Option<Ordering> {
        let tb1 = &imp1.0;
        let tb2 = &imp2.0;
        let ab1 = &imp1.1;
        let ab2 = &imp2.1;

        let tb_cmp = TBI_DLlite::cmp_imp(tb1, tb2);
        let ab_cmp = ABIQ_DLlite::compare_two_vectors(ab1, ab2);

        match (tb_cmp, ab_cmp) {
            (Option::None, _) => Option::None,
            (_, Option::None) => Option::None,
            (Some(Ordering::Less), Some(Ordering::Less))
            | (Some(Ordering::Less), Some(Ordering::Equal)) => Some(Ordering::Less),
            (Some(Ordering::Greater), Some(Ordering::Greater))
            | (Some(Ordering::Greater), Some(Ordering::Equal)) => Some(Ordering::Greater),
            (Some(Ordering::Equal), Some(Ordering::Equal)) => Some(Ordering::Equal),
            (Some(Ordering::Equal), Some(Ordering::Less)) => Some(Ordering::Less),
            (Some(Ordering::Equal), Some(Ordering::Greater)) => Some(Ordering::Greater),
            (Some(Ordering::Less), Some(Ordering::Greater))
            | (Some(Ordering::Greater), Some(Ordering::Less)) => {
                // this needs more analysis
                // a theoretical one I mean
                Option::None
            }
        }
    }
}

impl ABoxItem for ABIQ_DLlite {
    type NodeItem = Node_DLlite;
    type TBI = TBI_DLlite;

    fn negate(&self) -> Self {
        let abi_neg = self.abi.negate();

        // really dangerous here
        ABIQ_DLlite::new(abi_neg, Some(self.prevalue), self.value, self.level + 1)
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
    pub fn new(
        abi: ABI_DLlite,
        prevalue: Option<f64>,
        value: Option<f64>,
        level: usize,
    ) -> ABIQ_DLlite {
        let prevalue = match prevalue {
            Some(pv) => pv,
            _ => 1.0,
        };

        let impliers: Vec<(Vec<TBI_DLlite>, Vec<ABIQ_DLlite>)> = Vec::new();

        ABIQ_DLlite {
            abi,
            prevalue,
            value,
            level,
            impliers,
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

    pub fn level(&self) -> usize {
        self.level
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
    pub fn apply_rule(
        abiqs: Vec<&ABIQ_DLlite>,
        tbis: Vec<&TBI_DLlite>,
        rule: &AbRule<TBI_DLlite, ABIQ_DLlite>,
        deduction_tree: bool,
    ) -> Option<Vec<ABIQ_DLlite>> {
        let prov_vec = match tbis.len() {
            1 => rule(abiqs, tbis, deduction_tree),
            2 => rule(abiqs, tbis, deduction_tree),
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

    pub fn compare_two_vectors(v1: &Vec<ABIQ_DLlite>, v2: &Vec<ABIQ_DLlite>) -> Option<Ordering> {
        let len1 = v1.len();
        let len2 = v2.len();
        let mut all_good = true;
        let mut abiq1: &ABIQ_DLlite;
        let mut abiq2: &ABIQ_DLlite;

        let (lenght, ordering) = match len1.cmp(&len2) {
            Ordering::Less => (len1, Ordering::Less),
            Ordering::Equal => (len1, Ordering::Equal),
            Ordering::Greater => (len2, Ordering::Greater),
        };

        for i in 0..lenght {
            abiq1 = v1.get(i).unwrap();
            abiq2 = v2.get(i).unwrap();

            all_good = all_good && (abiq1 == abiq2);
        }

        match all_good {
            true => Some(ordering),
            false => Option::None,
        }
    }

    // function utility for levels
    pub fn get_extrema_level(v: Vec<&ABIQ_DLlite>, max_index: usize, get_max: bool) -> usize {
        // for max or min
        let mut extrema_level: usize = if get_max { 0 } else { usize::max_value() };

        // this part is independent of max and min
        let v_len = v.len();
        let max_index = if (v_len - 1) >= max_index {
            max_index
        } else {
            v_len
        };

        for i in 0..max_index {
            if get_max {
                extrema_level = extrema_level.max(v.get(i).unwrap().level);
            } else {
                extrema_level = extrema_level.min(v.get(i).unwrap().level);
            }
        }

        extrema_level
    }
}
