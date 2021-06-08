/*
 © - 2021 – UMONS
Horacio Alejandro Tellez Perez

LICENSE GPLV3+:
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see https://www.gnu.org/licenses/.
*/


/*
    This module define some usual aggregation functions to work with the ranking algorithm,
    they are:
    - sum
    - max
    - min
    - avg (arithmetic mean)
    - count (cardinality)
 */

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
