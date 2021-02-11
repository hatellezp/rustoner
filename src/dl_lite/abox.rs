use std::fmt;

use crate::dl_lite::abox_item::ABI;
use crate::kb::knowledge_base::Data;

#[derive(PartialEq, Debug, Clone)]
pub struct AB {
    name: String,
    items: Vec<ABI>,
    length: usize,
}

impl Data for AB {}

impl fmt::Display for AB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 {
            write!(f, "<AB>[]")
        } else {
            let mut s: String = format!("<AB({}>[", self.name);

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
    pub fn new(name: &str) -> AB {
        AB {
            name: name.to_string(),
            items: vec![],
            length: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.length
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

    pub fn items(&self) -> &Vec<ABI> {
        &self.items
    }
}
