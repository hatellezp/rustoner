extern crate itertools;

use itertools::fold;
use crate::kb::knowledge_base::AggrFn;

fn sum(v: Vec<f64>) -> f64 {
    match v.len() {
        0 => 0 as f64,
        _ => v.iter().sum::<f64>(),
    }
}

fn max(v: Vec<f64>) -> f64 {
    match v.len() {
        0 => 0 as f64, // find minimum and maximum values in rust for f64
        _ => fold(v.iter(), v[0], |a, &b| f64::max(a, b)),
    }
}

fn arith_mean(v: Vec<f64>) -> f64 {
    match v.len() {
        0 => 0 as f64,
        _ => v.iter().sum::<f64>() / (v.len() as f64),
    }
}

fn count(v: Vec<f64>) -> f64 {
    v.len() as f64
}

pub const aggr_sum: AggrFn = sum;
pub const aggr_max: AggrFn = max;
pub const aggr_mean: AggrFn = arith_mean;
pub const aggr_count: AggrFn = count;
