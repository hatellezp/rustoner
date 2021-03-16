use std::fmt;

use crate::dl_lite::node::NodeDllite;
use crate::kb::knowledge_base::{Implier, TbRule};
use crate::kb::knowledge_base::{Item, TBoxItem};
use crate::kb::types::DLType;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct TbiDllite {
    lside: NodeDllite,
    rside: NodeDllite,
    level: usize,
    impliers: Vec<Vec<TbiDllite>>,
}

impl PartialEq for TbiDllite {
    fn eq(&self, other: &Self) -> bool {
        self.lside == other.lside && self.rside == other.rside
    }
}

// I must implement Hash myself because PartialEq is implemented and can cause unforeseen errors: thanks Clippy
impl Hash for TbiDllite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lside.hash(state);
        self.rside.hash(state);
    }
}

impl Eq for TbiDllite {}

impl fmt::Display for TbiDllite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} < {}", self.lside, self.rside)
    }
}

impl PartialOrd for TbiDllite {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.lside().cmp(other.lside()) == Ordering::Less {
            Some(Ordering::Less)
        } else if self.lside().cmp(other.lside()) == Ordering::Greater {
            Some(Ordering::Greater)
        } else {
            self.rside().partial_cmp(other.rside())
        }
    }
}

impl Ord for TbiDllite {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Implier for TbiDllite {
    type Imp = Vec<TbiDllite>;

    fn implied_by(&self) -> &Vec<Vec<TbiDllite>> {
        &(self.impliers)
    }

    fn cmp_imp(imp1: &Vec<TbiDllite>, imp2: &Vec<TbiDllite>) -> Option<Ordering> {
        let len1 = imp1.len();
        let len2 = imp2.len();
        let mut all_good = true;
        let mut tbi1: &TbiDllite;
        let mut tbi2: &TbiDllite;
        let (lenght, ordering) = match len1.cmp(&len2) {
            Ordering::Less => (len1, Ordering::Less),
            Ordering::Equal => (len1, Ordering::Equal),
            Ordering::Greater => (len2, Ordering::Greater),
        };

        for i in 0..lenght {
            tbi1 = imp1.get(i).unwrap();
            tbi2 = imp2.get(i).unwrap();

            all_good = all_good && (tbi1 == tbi2);
        }

        match all_good {
            true => Some(ordering),
            false => Option::None,
        }
    }

    fn add_to_implied_by(&mut self, mut implier: Vec<TbiDllite>) {
        // before everything, here in DLlite we can negation implies reverse negation
        // we must avoid to add an element as self implier
        if !&implier.contains(&self) {
            implier.sort();
            let contains = self.contains_implier(&implier);

            match contains {
                Option::Some(Ordering::Less) => {
                    let mut cmpd: Option<Ordering>;
                    let mut inner_implier: &Vec<TbiDllite>;
                    let lenght: usize = self.impliers.len();

                    for index in 0..lenght {
                        inner_implier = &self.impliers.get(index).unwrap();
                        cmpd = Self::cmp_imp(&implier, inner_implier);

                        if let Option::Some(Ordering::Less) = cmpd {
                            self.impliers[index] = implier;
                            break; // rust is very smart, told me that value was being used in future iteration, so I put a break right here
                        }
                    }
                }
                Option::None => self.impliers.push(implier),
                Option::Some(Ordering::Equal) | Option::Some(Ordering::Greater) => (),
            }
        }
    }
}

impl TBoxItem for TbiDllite {
    type NodeItem = NodeDllite;

    fn lside(&self) -> &NodeDllite {
        &(self.lside)
    }

    fn rside(&self) -> &NodeDllite {
        &(self.rside)
    }

    fn is_trivial(&self) -> bool {
        self.lside.t() == DLType::Bottom || self.rside.t() == DLType::Top
    }

    fn is_negative_inclusion(&self) -> bool {
        self.rside.is_negated()
    }
}

impl TbiDllite {
    pub fn new(lside: NodeDllite, rside: NodeDllite, level: usize) -> Option<TbiDllite> {
        if (lside.t() == DLType::Nominal || rside.t() == DLType::Nominal)
            || (lside.is_negated() || !DLType::same_type(lside.t(), rside.t()))
        {
            Option::None
        } else {
            let implied_by: Vec<Vec<TbiDllite>> = Vec::new();

            Some(TbiDllite {
                lside,
                rside,
                level,
                impliers: implied_by,
            })
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn is_contradiction(&self) -> bool {
        self.lside.is_negation(&self.rside)
    }

    pub fn reverse_negation(&self, add_level: bool) -> Option<TbiDllite> {
        /*
        this method creates a new item
         */
        if self.rside.is_negated() {
            let lside = self.lside.clone();
            let rside = self.rside.clone();

            let level: usize = if !add_level {
                self.level
            } else {
                self.level + 1
            };

            TbiDllite::new(rside.negate(), lside.negate(), level)
        } else {
            Option::None
        }
    }

    pub fn apply_rule(
        tbis: Vec<&TbiDllite>,
        rule: &TbRule<TbiDllite>,
        deduction_tree: bool,
    ) -> Option<Vec<TbiDllite>> {
        /*
        put a switch here to add consequences when needed
        every vector in the answer get the vectors that created it in the implied_by field
         */

        let prov_vec = match tbis.len() {
            1 => rule(tbis, deduction_tree),
            2 => rule(tbis, deduction_tree),
            _ => Option::None,
        };

        match prov_vec {
            Option::None => Option::None,
            Some(prov_vec_unwrapped) => {
                let mut final_vec: Vec<TbiDllite> = Vec::new();

                for item in &prov_vec_unwrapped {
                    if !item.is_redundant() {
                        final_vec.push(item.clone());
                    }
                }

                Some(final_vec)
            }
        }
    }

    // function utility for levels
    pub fn get_extrema_level(v: Vec<&TbiDllite>, max_index: usize, get_max: bool) -> usize {
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
