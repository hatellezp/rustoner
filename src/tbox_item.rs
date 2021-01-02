use std::fmt;

use crate::node::Node;
use crate::rule::TbRule;
use crate::types::DLType;

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
        if lside.is_negated() || !DLType::same_type(lside.t(), rside.t()) {
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

    pub fn decompact(self) -> (Node, Node) {
        (self.lside, self.rside)
    }

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

    pub fn apply(one: &TBI, two: &TBI, rule: &TbRule) -> Option<TBI> {
        rule(vec![one, two])
    }
}
