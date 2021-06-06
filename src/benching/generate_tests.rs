use rand::Rng;
use std::time::Instant;

use crate::alg_math::bounds::find_bound_complex_wrapper;
use crate::alg_math::utilities::{median, solve_system_wrapper_only_id_mod};

// first thing to do, generate a simple random conflict matrix
pub fn generate_random_conflict_matrix_simple(n: usize) -> Vec<i8> {
    let length = n * n;
    let mut v: Vec<i8> = vec![0; length];

    let mut rng = rand::thread_rng();

    for i in 0..length {
        let value: usize = rng.gen_range(0..3);

        match value {
            0 => v[i] = -1,
            2 => v[i] = 1,
            _ => (),
        }
    }

    v
}

// generate values in range [low, high] and change zero to such value if a random number
// in range [0, 1] is bigger that sparcity
// at the end repopulate diagonal with zeroes
pub fn generate_random_aggr_matrix_simple(
    n: usize,
    low: i32,
    high: i32,
    sparcity: f64,
) -> Vec<f64> {
    let length = n * n;
    let mut v = vec![0_f64; length];

    let mut rng = rand::thread_rng();
    let low_f64 = low as f64;
    let high_f64 = high as f64;

    for i in 0..n {
        for j in 0..n {
            // generate a value in range [low, high] if not in diagonal
            if i != j {
                let value = rng.gen_range(low_f64..high_f64);

                // only add if bigger than sparcity
                if rng.gen_range(0_f64..1_f64) > sparcity {
                    v[i * n + j] = value;
                }
            }
        }
    }

    v
}

pub fn generate_random_prevalues_simple(n: usize, epsilon: f64, p: f64) -> Vec<f64> {
    let mut v = vec![1_f64; n];
    let mut rng = rand::thread_rng();

    for i in 0..n {
        let diff = rng.gen_range((-epsilon)..epsilon);

        if rng.gen_range(0_f64..1_f64) > p {
            v[i] += diff;
        }
    }

    v
}

// bench
pub fn bench_find_bound_complex_wrapper_simple(v: Vec<f64>) -> f64 {
    // simple because I stole the current values in main
    const TOLERANCE: f64 = 0.0000000000000001;
    const M_SCALER: f64 = 1.1;
    const B_TRANSLATE: f64 = 1.;

    let now = Instant::now();
    let bound_op = find_bound_complex_wrapper(v, TOLERANCE, M_SCALER, B_TRANSLATE);

    now.elapsed().as_secs_f64()
}
