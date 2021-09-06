mod alg_math;

mod benching;
mod dl_lite;
mod helper;
mod interface;
mod kb;

use crate::dl_lite::ontology::OntologyDllite;

use crate::helper::rank_abox;

use crate::kb::aggr_functions::*;
use crate::kb::knowledge_base::{ABox, TBox, TBoxItem};
use crate::kb::types::FileType;

use crate::alg_math::bounds::find_bound_complex_wrapper;
use crate::alg_math::utilities::UpperTriangle;

use rand::Rng;
use std::time::Instant;

/*
use benching::generate_tests::{
    bench_find_bound_complex_wrapper_simple, generate_random_aggr_matrix_simple,
    generate_random_prevalues_simple,
};

 */

use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::dl_lite::abox::AbqDllite;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, io};

use crate::alg_math::bounds::Adjusters;

use rand::thread_rng;
use rand_distr::{Distribution, Normal, NormalError};

use pad::PadStr;

// constants for the bound computing
const TOLERANCE: f64 = 0.0000000000000001;
const M_SCALER: f64 = 1.1;
const B_TRANSLATE: f64 = 1.;
const DOT_COMMAND_LINUX: &str = "dot";
const DOT_COMMAND_WINDOWS: &str = "dot.exe";
const COMMAND_SHELL_LINUX: &str = "sh";
const COMMAND_SHELL_WINDOWS: &str = "cmd";

pub fn main() {
    println!("this is bench 2!!!");


    let upper = UpperTriangle::new(10);

    for tuple in upper {
        println!("({}, {})", tuple.0, tuple.1);
    }

    /*
       What I want to do:
           - test gencontb time
           - test genconab time
           - test rank time
           - test matrix conflict building time
           - test stabilizer value time

       How I do this:
           - first the easy part is: test matrix conflict building time:
               * create random matrices and compute the ranking
           - for the other tasks:
               * create tbox of different sizes,
               * create several aboxes (of different sizes) for each tbox
               * run each task several times
               * find min, max, and mean times for each run
               * create your graphs

       KNOW THAT I WANT TO CREATE AN ASYNC VERSION AND TEST EVERYTHING WITH THE SECOND VERSION
    */

    let ns = [
        10, 20, 25, 50, 75, 100, 150, 200, 250, 300, 350, 400, 450, 500, 600, 700, 800, 900, 1000,
    ];
    let densities = [0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.85, 0.9, 0.95, 1.];
    let lower_bound = -5_f64;
    let upper_bound = 5_f64;
    let full_iterations: usize = 4000;

    let header = "| n           | lower       | upper       | density     | iterations  | concurrency | min         | mean        | max         |";
    let header_length: usize = header.len();
    let lines = vec!["-"; header_length].join("");

    println!("{}", lines);
    println!("{}", header);
    println!("{}", lines);

    for n in ns.iter() {

        for density in densities.iter() {
            // for the moment no concurrency implemented

            let iterations = (((full_iterations / *n) as f64) * (2_f64 / *density)) as usize;

            for use_concurency in &[false, true] {
                let _ = bench_rank_finding(
                    *n,
                    lower_bound,
                    upper_bound,
                    *density,
                    iterations,
                    *use_concurency,
                );
            }
            println!("{}", lines);
        }
    }
}

pub fn bench_rank_finding(
    n: usize,
    lower_bound: f64,
    upper_bound: f64,
    density: f64,
    iterations: usize,
    use_concurrency: bool,
) -> Result<(), NormalError> {
    // println!("================================================================");
    // println!("INFO: testing the rank bound finding algorithm with\n  n: {}\n  lower bound: {}\n  upper bound: {}\n  density: {}\n  iterations: {}", n, lower_bound, upper_bound, density, iterations);

    // repeat the processus for the number of iterations

    let inner_iterations = iterations;
    // let inner_iterations = 1;

    let mut values = vec![0_f64; inner_iterations];
    for iteration in 0..inner_iterations {
        // build a matrix
        let mut matrix = build_matrix(n, lower_bound, upper_bound, density)?;

        // measure the time
        let now = Instant::now();
        let b = find_bound_complex_wrapper(
            matrix.clone(),
            TOLERANCE,
            M_SCALER,
            B_TRANSLATE,
            use_concurrency,
        );
        // println!("bound is {:?}", b);
        let elapsed = now.elapsed().as_secs_f64();

        // store the value
        values[iteration] = elapsed;
    }

    // find the necessary values
    let max = values.iter().fold(values[0], |accum, x| accum.max(*x));
    let min = values.iter().fold(values[0], |accum, x| accum.min(*x));
    let mean = values.iter().fold(0_f64, |accum, x| accum + *x) / (values.len() as f64);

    let width: usize = 11;
    let n_str = format!("{}", n).pad_to_width(width);
    let lower_str = format!("{}", lower_bound).pad_to_width(width);
    let upper_str = format!("{}", upper_bound).pad_to_width(width);
    let density_str = format!("{}", density).pad_to_width(width);
    let iterations_str = format!("{}", inner_iterations).pad_to_width(width);
    let use_concurrency_str = if use_concurrency { format!("{}", "concurrency").pad_to_width(width) } else { format!("{}", "linear").pad_to_width(width) };

    let min_str = format!("{0:.6}", min).pad_to_width(width);
    let mean_str = format!("{0:.6}", mean).pad_to_width(width);
    let max_str = format!("{0:.6}", max).pad_to_width(width);

    println!(
        "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
        n_str,
        lower_str,
        upper_str,
        density_str,
        iterations_str,
        use_concurrency_str,
        min_str,
        mean_str,
        max_str
    );

    Ok(())
}

pub fn build_matrix(
    n: usize,
    lower_bound: f64,
    upper_bound: f64,
    density: f64,
) -> Result<Vec<f64>, NormalError> {
    // size of the matrix
    let n_square = n * n;

    // declate random generator
    let mut rng = thread_rng();
    let normal = Normal::new(lower_bound, upper_bound)?;

    // build a matrix
    let mut matrix = vec![0_f64; n_square];
    for i in 0..n_square {
        let v: f64 = normal.sample(&mut rng);
        matrix[i] = v;
    }

    // add zeroes to gain the necessary density
    // density adjustement here
    let index_to_zeroes_density = 1_f64 - density;

    if index_to_zeroes_density > 0_f64 && index_to_zeroes_density < 1_f64 {
        let index_to_zeroes = ((n as f64) * index_to_zeroes_density) as usize;

        for _ in 0..index_to_zeroes {
            let index = rng.gen_range(0..n);

            for i in 0..n {
                matrix[n*i + index] = 0_f64;
                matrix[n*index + i] = 0_f64;
            }
        }
    }

    // set the diagonal to zero
    for i in 0..n {
        matrix[n * i + i] = 0_f64;
    }

    Ok(matrix)
}
