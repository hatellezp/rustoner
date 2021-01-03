use std::fmt;

use crate::dl_lite::node::Node;
use crate::dl_lite::rule::TbRule;
use crate::dl_lite::types::DLType;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct TBI {
    lside: Node,
    rside: Node,
}

impl fmt::Display for TBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{}<={}", self.lside, self.rside)
        write!(f, "{}<{}", self.lside, self.rside)
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

    pub fn apply(one: &TBI, two: &TBI, rule: &TbRule) -> Option<Vec<TBI>> {
        rule(vec![one, two])
    }
}
