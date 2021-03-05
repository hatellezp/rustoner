use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::rule::TbRule;
use crate::dl_lite::types::DLType;
use crate::kb::knowledge_base::AxiomItem;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct TBI {
    lside: Node,
    rside: Node,
    level: usize,
    implied_by: Vec<Vec<TBI>>,
}

impl AxiomItem for TBI {}

impl fmt::Display for TBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} < {}", self.lside, self.rside)
    }
}

impl PartialOrd for TBI {
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

impl Ord for TBI {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TBI {
    pub fn new(lside: Node, rside: Node) -> Option<TBI> {
        if lside.t() == DLType::Nominal || rside.t() == DLType::Nominal {
            Option::None
        } else if lside.is_negated() || !DLType::same_type(lside.t(), rside.t()) {
            Option::None
        } else {
            let implied_by: Vec<Vec<TBI>> = Vec::new();
            let level: usize = 0;

            Some(TBI { lside, rside, level, implied_by })
        }
    }

    pub fn lside(&self) -> &Node {
        &(self.lside)
    }

    pub fn rside(&self) -> &Node {
        &(self.rside)
    }

    pub fn implied_by(&self) -> &Vec<Vec<TBI>>  { &(self.implied_by) }

    pub fn is_contradiction(&self) -> bool {
        self.lside.is_negation(&self.rside)
    }

    pub fn is_redundant(&self) -> bool {
        self.lside == self.rside
    }

    pub fn is_trivial(&self) -> bool {
        self.lside.t() == DLType::Bottom || self.rside.t() == DLType::Top
    }

    pub fn is_negative_inclusion(&self) -> bool {
        self.rside.is_negated()
    }

    pub fn is_positive_inclusion(&self) -> bool {
        !self.is_negative_inclusion()
    }

    pub fn reverse_negation(&self) -> Option<TBI> {
        /*
        this method creates a new item
         */
        if self.rside.is_negated() {
            let lside = self.lside.clone();
            let rside = self.rside.clone();

            TBI::new(rside.negate(), lside.negate())
        } else {
            Option::None
        }
    }

    pub fn apply_rule(tbis: Vec<&TBI>, rule: &TbRule) -> Option<Vec<TBI>> {
        /*
        put a switch here to add consequences when needed
        every vector in the answer get the vectors that created it in the implied_by field
         */

        let prov_vec = match tbis.len() {
            1 => rule(tbis),
            2 => rule(tbis),
            _ => Option::None,
        };

        if prov_vec.is_none() {
            Option::None
        } else {
            let prov_vec = prov_vec.unwrap();
            let mut final_vec: Vec<TBI> = Vec::new();

            for item in &prov_vec {
                if !item.is_redundant() {
                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }
}
