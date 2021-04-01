/*
UMONS 2021
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

use nalgebra::{Complex, DMatrix, DVector};
use std::cmp::Ordering;
use std::f64::consts::PI;

// creation of matrix and vectors

/*
   I found that there several prescision errors when computing division,
   this is a working solution
*/
const PRECISION_ROUNDER: f64 = 1000000000000000.;
pub fn round_to_15_f64(v: f64) -> f64 {
    (v * PRECISION_ROUNDER).round() / PRECISION_ROUNDER
}

pub fn create_indentity_matrix_complex(n: usize) -> DMatrix<Complex<f64>> {
    // creates a matrix of size n*n
    // be aware of this
    let id: DMatrix<Complex<f64>> =
        DMatrix::from_vec(n, n, vec![Complex { re: 1.0, im: 0. }; n * n]);
    id
}

pub fn create_unity_roots(v: &mut DVector<Complex<f64>>, n: usize, inverse: bool) {
    if v.len() != n {
        std::process::exit(exitcode::DATAERR)
    } else {
        let n64 = n as f64;
        let t: f64 = if inverse { -1. } else { 1. };
        let mut arg: f64;
        let mut re: f64;
        let mut im: f64;
        let mut new_complex: Complex<f64>;

        for k in 0..n {
            arg = (t * (2. * PI * (k as f64))) / n64;
            re = arg.cos();
            im = arg.sin();

            re = round_to_15_f64(re);
            im = round_to_15_f64(im);

            new_complex = Complex { re, im };
            v[k] = new_complex;
        }
    }
    // println!("{}", &v);
}

/*
pub fn create_all_units_vector_complex(n: usize) -> DVector<Complex<f64>> {
    let v: DVector<Complex<f64>> = DVector::from_vec(vec![Complex { re: 1.0, im: 0. }; n]);
    v
}

 */

// basic operations

pub fn matrix_is_zero_complex(m: &DMatrix<Complex<f64>>) -> bool {
    let zero: Complex<f64> = Complex { re: 0., im: 0. };

    for r in 0..m.nrows() {
        for c in 0..m.ncols() {
            if m[(r, c)] != zero {
                return false;
            }
        }
    }

    true
}

pub fn matrix_subtraction(receiver: &mut DMatrix<Complex<f64>>, minus: &DMatrix<Complex<f64>>) {
    if receiver.ncols() != minus.ncols() || receiver.nrows() != minus.nrows() {
        println!("mismatched dimension");
    } else {
        for r in 0..receiver.nrows() {
            for n in 0..receiver.ncols() {
                receiver[(r, n)] -= minus[(r, n)];
            }
        }
    }
}

pub fn multiply_vector_complex(v: &mut DVector<Complex<f64>>, mul: Complex<f64>) {
    let v2 = v.map(|x| x * mul);
    v.copy_from(&v2);
}

pub fn multiply_matrix_complex(m: &mut DMatrix<Complex<f64>>, mul: Complex<f64>) {
    let m2 = m.map(|x| x * mul);
    m.copy_from(&m2);
}

// more complex behaviour

pub fn solve_system(
    matrix: &DMatrix<Complex<f64>>,
    mut solution: &mut DVector<Complex<f64>>,
    identity_mod: Complex<f64>,
    matrix_mod: Complex<f64>,
    vector_mod: Complex<f64>,
) {
    let rows = matrix.nrows();
    let cols = matrix.ncols();
    let solution_len = solution.len();

    if rows != cols {
        println!("not an square matrix!");
        std::process::exit(exitcode::DATAERR)
    } else if rows != solution_len {
        println!("mismatch size of vector and matrix");
        std::process::exit(exitcode::DATAERR)
    } else {
        // here we solve (identity_mod * 1 - matrix_mod * m)X = vector_mod * (1,...1)
        let global_size = rows; // to avoid misunderstandings

        // let mut receiver_matrix = create_indetity_matrix_complex(global_size);
        // let mut copy_matrix = create_indentity_matrix_complex(global_size);

        // this idea allocate to vector, we are going to put everyting in solution
        // let mut vector = create_all_units_vector_complex(global_size);
        // put everything in solution
        for i in 0..global_size {
            solution[i] = Complex { re: 1.0, im: 0. };
        }

        let mut receiver_matrix: DMatrix<Complex<f64>> = DMatrix::from_vec(
            global_size,
            global_size,
            vec![Complex { re: 0., im: 0. }; global_size * global_size],
        );
        for i in 0..global_size {
            receiver_matrix[(i, i)] = Complex { re: 1., im: 0. };
        }

        // copy content from matrxi to copy_matrix
        let mut copy_matrix = matrix.clone();
        // copy_matrix.copy_from(matrix);

        // receiver = receiver * identity_mod
        multiply_matrix_complex(&mut receiver_matrix, identity_mod);

        // copy_matrix = copy_matrix * matrix_mod
        multiply_matrix_complex(&mut copy_matrix, matrix_mod);

        // now receiver has the form : identity_mod * 1 - matrix_mod * m
        receiver_matrix -= copy_matrix;

        // first test in-place modification
        // mutliply vector
        // multiply_vector_complex(&mut vector, vector_mod);
        multiply_vector_complex(&mut solution, vector_mod);

        // I'm using LU decomposition here
        let decomposed = receiver_matrix.lu();
        decomposed.solve_mut(&mut solution);

        // put everything in solution
        // solution.copy_from(&vector);

        // done
    }
}

// wrappers
pub fn solve_system_wrapper_only_id_mod(v: &[f64], receiver: &mut Vec<f64>, id_mod: f64) -> bool {
    solve_system_wrapper(v, receiver, id_mod, 1., 1.)
}

pub fn solve_system_wrapper(
    v: &[f64],
    receiver: &mut Vec<f64>,
    id_mod: f64,
    ma_mod: f64,
    ve_mod: f64,
) -> bool {
    let nsquared = v.len();
    let n = (nsquared as f64).sqrt() as usize;
    let n_receiver = receiver.len();

    if n * n != nsquared {
        println!(
            "not an square matrix can be formed, {} is not a perfect square!",
            nsquared
        );
        false
    } else if n_receiver != n {
        println!(
            "the receiver vector has mismatched lenght: vector len: {}, matrix dim: {}",
            n_receiver, n
        );
        false
    } else {
        let v_c64 = v
            .iter()
            .map(|x| Complex { re: *x, im: 0. })
            .collect::<Vec<Complex<f64>>>();
        let mut matrix: DMatrix<Complex<f64>> = DMatrix::from_vec(n, n, v_c64);
        matrix.transpose_mut();

        let identity_mod: Complex<f64> = Complex { re: id_mod, im: 0. };
        let matrix_mod: Complex<f64> = Complex { re: ma_mod, im: 0. };
        let vector_mod: Complex<f64> = Complex { re: ve_mod, im: 0. };

        let mut solution: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 0., im: 0. }; n]);

        solve_system(&matrix, &mut solution, identity_mod, matrix_mod, vector_mod);

        // copy everything
        for i in 0..n {
            receiver[i] = solution[i].re;
        }

        true
    }
}

// for some simple statistics tasks
// from rust nursery
fn partition(data: &[f64]) -> Option<(Vec<f64>, f64, Vec<f64>)> {
    match data.len() {
        0 => None,
        _ => {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            Some((left, pivot, right))
        }
    }
}

fn select(data: &[f64], k: usize) -> Option<f64> {
    let part = partition(data);

    match part {
        None => None,
        Some((left, pivot, right)) => {
            let pivot_idx = left.len();

            match pivot_idx.cmp(&k) {
                Ordering::Equal => Some(pivot),
                Ordering::Greater => select(&left, k),
                Ordering::Less => select(&right, k - (pivot_idx + 1)),
            }
        }
    }
}

pub fn median(data: &[f64]) -> Option<f64> {
    let size = data.len();

    match size {
        even if even % 2 == 0 => {
            let fst_med = select(data, (even / 2) - 1);
            let snd_med = select(data, even / 2);

            match (fst_med, snd_med) {
                (Some(fst), Some(snd)) => Some((fst + snd) / 2.0),
                _ => None,
            }
        }
        odd => select(data, odd / 2),
    }
}

// TODO: find a way to generalize to every numeric type: integer, float, complex...
pub fn null_vector(v: &[i8]) -> bool {
    for item in v {
        if *item != 0 {
            return false;
        }
    }

    true
}
