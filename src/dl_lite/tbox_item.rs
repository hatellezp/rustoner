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
}

impl AxiomItem for TBI {}

impl fmt::Display for TBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}<={}", self.lside, self.rside)
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
            Some(TBI { lside, rside })
        }
    }

    pub fn lside(&self) -> &Node {
        &(self.lside)
    }

    pub fn rside(&self) -> &Node {
        &(self.rside)
    }

    pub fn is_contradiction(&self) -> bool {
        self.lside.is_negation(&self.rside)
    }

    pub fn is_redundant(&self) -> bool {
        self.lside == self.rside
    }

    pub fn is_trivial(&self) -> bool {
        self.lside.t() == DLType::Bottom || self.rside.t() == DLType::Top
    }

    // this functions consumes self
    pub fn decompact(self) -> (Node, Node) {
        (self.lside, self.rside)
    }

    // same but leaves self alone
    pub fn decompact_with_clone(&self) -> (Node, Node) {
        (self.clone()).decompact()
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
                // println!("trying to add: {}", item);

                if !item.is_redundant() {
                    // println!("    success");

                    final_vec.push(item.clone());
                }
            }

            Some(final_vec)
        }
    }
}
