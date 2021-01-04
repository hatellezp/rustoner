use std::fmt;

use crate::dl_lite::abox_item::ABI;
use crate::kb::knowledge_base::Data;

pub struct AB {
    items: Vec<ABI>,
    length: usize,
}

impl Data for AB {}

impl fmt::Display for AB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 {
            write!(f, "<AB>[]")
        } else {
            let mut s: String = String::from("<AB>[");

            for item in &self.items {
                s.push_str(item.to_string().as_str());
                s.push_str(", ");
            }

            s.push_str("]");

            write!(f, "{}", s)
        }
    }
}

impl AB {
    pub fn new() -> AB {
        AB {
            items: vec![],
            length: 0,
        }
    }

    pub fn add(&mut self, abi: ABI) -> bool {
        /*
        returns true if the item was successfully inserted, false otherwise
         */
        if !self.items.contains(&abi) {
            self.items.push(abi);
            self.length += 1;
            true
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    // pub fn complete(&self, tbox: Option<TB>) -> AB {}
}
