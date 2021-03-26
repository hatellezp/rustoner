use std::fmt;

use crate::dl_lite::node::NodeDllite;
use crate::dl_lite::tbox_item::TbiDllite;
use crate::kb::knowledge_base::{AbRule, Implier};

use crate::dl_lite::abox_item::AbiDllite;
use crate::kb::knowledge_base::ABoxItem;
use crate::kb::types::{DLType, CR};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};


/*
   remember that only base roles and base concepts are allowed here !!
*/
#[derive(Debug, Clone)]
pub struct AbiqDllite {
    abi: AbiDllite, // role or concept assertion
    prevalue: f64,
    value: Option<f64>,
    level: usize,
    impliers: Vec<(CR, Vec<TbiDllite>, Vec<AbiqDllite>)>,
}

// TODO: this might apport some unseen problems... :(
impl PartialEq for AbiqDllite {
    fn eq(&self, other: &Self) -> bool {
        self.abi.eq(other.abi())
    }
}

impl Eq for AbiqDllite {}

// TODO: is this enough ????
//     : Clippy complais that I only hash the abi, maybe I should do the same for the abiq ?
impl Hash for AbiqDllite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.abi.hash(state);
    }
}

impl fmt::Display for AbiqDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {:?})", self.abi, self.prevalue, self.value)
    }
}

impl PartialOrd for AbiqDllite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.abi.partial_cmp(&other.abi)
    }
}

impl Ord for AbiqDllite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Implier for AbiqDllite {
    type Imp = (CR, Vec<TbiDllite>, Vec<AbiqDllite>);

    fn implied_by(&self) -> &Vec<(CR, Vec<TbiDllite>, Vec<AbiqDllite>)> {
        &self.impliers
    }

    fn add_to_implied_by(&mut self, implier: (CR, Vec<TbiDllite>, Vec<AbiqDllite>)) {
        let r = implier.0;
        let mut tb = implier.1;
        let mut ab = implier.2;

        // sorting always needed
        tb.sort();
        ab.sort();
        let implier = (r, tb, ab);

        let contains = self.contains_implier(&implier);

        match contains {
            Option::Some(Ordering::Less) => {
                let mut cmpd: Option<Ordering>;
                let mut inner_implier: &(CR, Vec<TbiDllite>, Vec<AbiqDllite>);
                let lenght: usize = self.impliers.len();

                for index in 0..lenght {
                    inner_implier = &self.impliers.get(index).unwrap();
                    cmpd = Self::cmp_imp(&implier, inner_implier);

                    if let Some(Ordering::Less) = cmpd {
                        self.impliers[index] = implier;
                        break; // rust is very smart, told me that value was being used in future iteration, so I put a break right here
                    }
                }
            }
            Option::None => self.impliers.push(implier),
            Option::Some(Ordering::Equal) | Option::Some(Ordering::Greater) => (),
        }
    }

    fn cmp_imp(imp1: &Self::Imp, imp2: &Self::Imp) -> Option<Ordering> {
        let r1 = &imp1.0;
        let r2 = &imp2.0;
        let tb1 = (&imp1.1).clone();
        let tb2 = (&imp2.1).clone();
        let ab1 = &imp1.2;
        let ab2 = &imp2.2;

        let tb_cmp = TbiDllite::cmp_imp(&(*r1, tb1), &(*r2, tb2));
        let ab_cmp = AbiqDllite::compare_two_vectors(ab1, ab2);

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

impl ABoxItem for AbiqDllite {
    type NodeItem = NodeDllite;
    type TBI = TbiDllite;

    fn negate(&self) -> Self {
        let abi_neg = self.abi.negate();

        // really dangerous here
        AbiqDllite::new(abi_neg, Some(self.prevalue), self.value, self.level + 1)
    }

    fn t(&self) -> DLType {
        self.abi.t()
    }

    fn item(&self) -> &NodeDllite {
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

impl AbiqDllite {
    pub fn new(
        abi: AbiDllite,
        prevalue: Option<f64>,
        value: Option<f64>,
        level: usize,
    ) -> AbiqDllite {
        let prevalue = match prevalue {
            Some(pv) => pv,
            _ => 1.0,
        };

        let impliers: Vec<(CR, Vec<TbiDllite>, Vec<AbiqDllite>)> = Vec::new();

        AbiqDllite {
            abi,
            prevalue,
            value,
            level,
            impliers,
        }
    }

    pub fn abi(&self) -> &AbiDllite {
        &self.abi
    }

    pub fn prevalue(&self) -> f64 {
        self.prevalue
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = Some(v);
    }

    pub fn set_prevalue(&mut self, v: f64) {
        self.prevalue = v;
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

    pub fn same_nominal(&self, other: &Self) -> bool {
        self.abi.same_nominal(&other.abi)
    }

    // pub fn apply_two(one: &ABIQ, two: &ABIQ, tbox: &TB) -> Option<Vec<ABIQ>> {}
    pub fn apply_rule(
        abiqs: Vec<&AbiqDllite>,
        tbis: Vec<&TbiDllite>,
        rule: &AbRule<TbiDllite, AbiqDllite>,
        deduction_tree: bool,
    ) -> Option<Vec<AbiqDllite>> {
        let prov_vec = match tbis.len() {
            1 => rule(abiqs, tbis, deduction_tree),
            2 => rule(abiqs, tbis, deduction_tree),
            _ => Option::None,
        };

        if let Some(some_vec) = prov_vec {
            let mut final_vec: Vec<AbiqDllite> = Vec::new();

            for item in some_vec {
                if !item.abi().is_trivial() {
                    final_vec.push(item);
                }
            }

            Some(final_vec)
        } else {
            Option::None
        }
    }

    pub fn compare_two_vectors(v1: &[AbiqDllite], v2: &[AbiqDllite]) -> Option<Ordering> {
        let len1 = v1.len();
        let len2 = v2.len();
        let mut all_good = true;
        let mut abiq1: &AbiqDllite;
        let mut abiq2: &AbiqDllite;

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
}
