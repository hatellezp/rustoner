use std::fmt;

use crate::shiq::node::Node_SHIQ;
use crate::kb::knowledge_base::TbRule;
use crate::kb::types::DLType;
use crate::kb::knowledge_base::{Item, ABoxItem, ABox, TBoxItem, TBox};
use std::cmp::Ordering;

#[derive(Debug, Hash, Clone)]
pub struct TBI_SHIQ {
    lside: Node_SHIQ,
    rside: Node_SHIQ,
    level: usize,
    implied_by: Vec<Vec<TBI_SHIQ>>,
}

impl PartialEq for TBI_SHIQ {
    fn eq(&self, other: &Self) -> bool {
        self.lside == other.lside && self.rside == other.rside
    }
}

impl Eq for TBI_SHIQ {}

impl fmt::Display for TBI_SHIQ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} < {}", self.lside, self.rside)
    }
}

impl PartialOrd for TBI_SHIQ {
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

impl Ord for TBI_SHIQ {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TBoxItem for TBI_SHIQ {
    type NodeItem = Node_SHIQ;

    fn lside(&self) -> &Node_SHIQ {
        &(self.lside)
    }

    fn rside(&self) -> &Node_SHIQ {
        &(self.rside)
    }

    fn is_trivial(&self) -> bool {
        self.lside.t() == DLType::Bottom || self.rside.t() == DLType::Top
    }

    fn is_negative_inclusion(&self) -> bool {
        self.rside.is_negated()
    }


    fn implied_by(&self) -> &Vec<Vec<TBI_SHIQ>> {
        &(self.implied_by)
    }

    fn add_to_implied_by(&mut self, impliers: Vec<TBI_SHIQ>) {
        self.implied_by.push(impliers);
    }

}

impl TBI_SHIQ {
    pub fn new(lside: Node_SHIQ, rside: Node_SHIQ, level: usize) -> Option<TBI_SHIQ> {
        if lside.t() == DLType::Nominal || rside.t() == DLType::Nominal {
            Option::None
        } else if lside.is_negated() || !DLType::same_type(lside.t(), rside.t()) {
            Option::None
        } else {
            let implied_by: Vec<Vec<TBI_SHIQ>> = Vec::new();

            Some(TBI_SHIQ {
                lside,
                rside,
                level,
                implied_by,
            })
        }
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn is_contradiction(&self) -> bool {
        self.lside.is_negation(&self.rside)
    }

    pub fn reverse_negation(&self, add_level: bool) -> Option<TBI_SHIQ> {
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

            TBI_SHIQ::new(rside.negate(), lside.negate(), level)
        } else {
            Option::None
        }
    }

    pub fn apply_rule(tbis: Vec<&TBI_SHIQ>, rule: &TbRule<TBI_SHIQ>, deduction_tree: bool) -> Option<Vec<TBI_SHIQ>> {
        /*
        put a switch here to add consequences when needed
        every vector in the answer get the vectors that created it in the implied_by field
         */

        let prov_vec = match tbis.len() {
            1 => rule(tbis, deduction_tree),
            2 => rule(tbis, deduction_tree),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec = prov_vec.unwrap();
            let mut final_vec: Vec<TBI_SHIQ> = Vec::new();

            for item in &prov_vec {
                if !item.is_redundant() {
                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }

    // function utility for levels
    pub fn get_extrema_level(v: Vec<&TBI_SHIQ>, max_index: usize, get_max: bool) -> usize {
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