use std::cmp::Ordering;

pub fn ordering_cmp_helper(len1: usize, len2: usize) -> (usize, Ordering) {
    match len1.cmp(&len2) {
        Ordering::Less => (len1, Ordering::Less),
        Ordering::Equal => (len1, Ordering::Equal),
        Ordering::Greater => (len2, Ordering::Greater),
    }
}

