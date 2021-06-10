use crate::kb::knowledge_base::LeveledItem;
use std::cmp::Ordering;

/// Helps ordering size of impliers both in abox items and tbox items.
pub fn ordering_cmp_helper(len1: usize, len2: usize) -> (usize, Ordering) {
    match len1.cmp(&len2) {
        Ordering::Less => (len1, Ordering::Less),
        Ordering::Equal => (len1, Ordering::Equal),
        Ordering::Greater => (len2, Ordering::Greater),
    }
}

pub fn get_max_level_abstract<T: LeveledItem>(items: &[T]) -> usize {
    let mut max_level: usize = 0;

    for item in items {
        max_level = max_level.max(item.level());
    }

    max_level
}
