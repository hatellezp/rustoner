extern crate itertools;

use crate::kb::knowledge_base::AggrFn;
use itertools::fold;

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

fn min(v: Vec<f64>) -> f64 {
    match v.len() {
        0 => 0 as f64,
        _ => fold(v.iter(), v[0], |a, &b| f64::min(a, b)),
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

pub const AGGR_SUM: AggrFn = sum;
pub const AGGR_MAX: AggrFn = max;
pub const AGGR_MIN: AggrFn = min;
pub const AGGR_MEAN: AggrFn = arith_mean;
pub const AGGR_COUNT: AggrFn = count;
