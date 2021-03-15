mod alg_math;
mod dl_lite;
mod interface;
mod kb;

use crate::alg_math::bounds::find_bound_complex_wrapper;
use crate::alg_math::utilities::solve_system_wrapper_only_id_mod;

const TOLERANCE: f64 = 0.000001;
const M_SCALER: f64 = 1.1;
const B_TRANSLATE: f64 = 1.;

fn main() {
    let v: Vec<f64> = vec![1., 1., 0., 1.5];
    let res = find_bound_complex_wrapper(v.clone(), TOLERANCE, M_SCALER, B_TRANSLATE);

    println!("{:?}", &res);

    let mut receiver: Vec<f64> = vec![0.; 2];

    solve_system_wrapper_only_id_mod(v, &mut receiver, res.unwrap());

    let res = find_bound_complex_wrapper(vec![], TOLERANCE, M_SCALER, B_TRANSLATE);
    println!("{:?}", res);
}
