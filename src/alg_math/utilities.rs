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

// =================================================================================================
// IMPORTS
use nalgebra::{Complex, DMatrix, DVector};
use std::cmp::Ordering;
use std::f64::consts::PI;

// END OF IMPORTS
// =================================================================================================

/*
   Must (if not all) operations on matrices in this module are done on references,
   copies are to be avoided whenever possible.
*/

/*
   I found that there several precision errors when computing division,
   this is a working solution.
   This problem is recurrent in python, rust, C and R too.
   Computing sine (or cosine for that matter) introduce rounding errors 15 digits  after 15
   the point.
   My attempts to solve this issue resulted in a function that is not worst than rust
   implementation (nor c nor python) but not better, thus I leave it as it was.
*/
const PRECISION_ROUNDER: f64 = 1000000000000000.; // to solve rounding errors.
pub fn round_to_15_f64(v: f64) -> f64 {
    (v * PRECISION_ROUNDER).round() / PRECISION_ROUNDER
}

/// takes an integer 'n' as argument and returns
/// a Complex DMatrix struct (naglebra defined) (float numbers are double precision)
/// dimension of the matrix is n*n
pub fn create_identity_matrix_complex(n: usize) -> DMatrix<Complex<f64>> {
    let id: DMatrix<Complex<f64>> =
        DMatrix::from_vec(n, n, vec![Complex { re: 1.0, im: 0. }; n * n]);
    id
}

/// put the n nth complex roots of the unity in the vector roots
/// roots must be of dimension n, otherwise the procedure will fail and the
/// program will exit (remark that roots must be a mutable reference)
pub fn create_unity_roots(roots: &mut DVector<Complex<f64>>, n: usize, inverse: bool) {
    if roots.len() != n {
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
            roots[k] = new_complex;
        }
    }
}

#[inline]
pub fn output_unity_root(n: usize, k: usize, inverse: bool) -> Complex<f64> {
    let t: f64 = if inverse { -1_f64 } else { 1_f64 };

    let arg = (t * 2_f64 * PI * (k as f64)) / (n as f64);
    let re = round_to_15_f64(arg.cos());
    let im = round_to_15_f64(arg.sin());

    Complex { re, im }
}

/// verify that matrix is an all zero matrix
pub fn matrix_is_zero_complex(matrix: &DMatrix<Complex<f64>>) -> bool {
    // this implementation is more elegant and rust will optimize it for us
    let zero: Complex<f64> = Complex { re: 0., im: 0. };
    matrix.iter().all(|&entry| entry == zero)
}

/// receiver will get receiver - minus
/// if there is a dimension mismatch the operation will fail with
/// a DATAERR error code (see the exitcode module)
pub fn matrix_subtraction(receiver: &mut DMatrix<Complex<f64>>, minus: &DMatrix<Complex<f64>>) {
    if receiver.ncols() != minus.ncols() || receiver.nrows() != minus.nrows() {
        println!("mismatched dimension");
        std::process::exit(exitcode::DATAERR)
    } else {
        for r in 0..receiver.nrows() {
            for n in 0..receiver.ncols() {
                receiver[(r, n)] -= minus[(r, n)];
            }
        }
    }
}

pub fn multiply_vector_complex(vector: &mut DVector<Complex<f64>>, scalar: Complex<f64>) {
    // better implementation
    vector.apply(|entry| entry * scalar);
}

pub fn multiply_matrix_complex(matrix: &mut DMatrix<Complex<f64>>, scalar: Complex<f64>) {
    // better implementation
    matrix.apply(|entry| entry * scalar);
}

/// this function solves the following system:
/// (identity_mod * 1 - matrix_mod * m)X = vector_mod * (1,...1)
/// will fail if matrix is not square or
/// if the solution receiver vector length is mismatch matrix dimension
pub fn solve_system(
    matrix: &DMatrix<Complex<f64>>,
    mut solution: &mut DVector<Complex<f64>>,
    identity_mod: Complex<f64>,
    matrix_mod: Complex<f64>,
    vector_mod: Complex<f64>,
) {
    // keep this here to avoid calling repeatedly to search for these values
    let rows = matrix.nrows();
    let cols = matrix.ncols();
    let solution_len = solution.len();

    if rows != cols {
        // bad dimension
        println!("not an square matrix!");
        std::process::exit(exitcode::DATAERR)
    } else if rows != solution_len {
        // mismatch dimension between solution vector and matrix
        println!("mismatch size of vector and matrix");
        std::process::exit(exitcode::DATAERR)
    } else {
        let global_size = rows; // to avoid misunderstandings, all dimensions are the same

        // the solution vector will first store the information for the right side vector of
        // the equation
        // after it will store the solution of the system
        for i in 0..global_size {
            solution[i] = Complex { re: 1.0, im: 0. };
        }

        // receiver matrix will store the computation for the left side matrix
        // (identity_mod * 1 - matrix_mod * m)X
        let mut receiver_matrix: DMatrix<Complex<f64>> = DMatrix::from_vec(
            global_size,
            global_size,
            vec![Complex { re: 0., im: 0. }; global_size * global_size],
        );

        // for the moment receiver is the identity matrix
        for i in 0..global_size {
            receiver_matrix[(i, i)] = Complex { re: 1., im: 0. };
        }

        // copy content from matrix to copy_matrix
        let mut copy_matrix = matrix.clone();

        // receiver <- receiver * identity_mod
        multiply_matrix_complex(&mut receiver_matrix, identity_mod);

        // copy_matrix <- copy_matrix * matrix_mod
        multiply_matrix_complex(&mut copy_matrix, matrix_mod);

        // now receiver has the form : identity_mod * 1 - matrix_mod * m
        receiver_matrix -= copy_matrix;

        // solution <- solution * vector_mod
        multiply_vector_complex(&mut solution, vector_mod);

        // I'm using LU decomposition here
        let decomposed = receiver_matrix.lu();

        // solution receives the actual solution
        decomposed.solve_mut(&mut solution);
    }
}

/// wrapper over the solve_system_wrapper function
/// matrix modifier is set to 1
/// vector modifier is set to 1
pub fn solve_system_wrapper_only_id_mod(
    vector: &[f64],
    receiver: &mut Vec<f64>,
    id_mod: f64,
) -> bool {
    solve_system_wrapper(vector, receiver, id_mod, 1., 1.)
}

/// takes mutable sequences of f64 numbers and transform
/// to matrix (DMatrix) and vector (DVector) respectively
/// to perform matrix equation solution
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
        // check if the matrix cannot be squared
        println!(
            "not an square matrix can be formed, {} is not a perfect square!",
            nsquared
        );
        false
    } else if n_receiver != n {
        // checks if receiver has the good length
        println!(
            "the receiver vector has mismatched lenght: vector len: {}, matrix dim: {}",
            n_receiver, n
        );
        false
    } else {
        // builds a complex vector from the f64 real values stored in v
        let v_c64 = v
            .iter()
            .map(|x| Complex { re: *x, im: 0. })
            .collect::<Vec<Complex<f64>>>();
        // build the matrix from v_64
        let mut matrix: DMatrix<Complex<f64>> = DMatrix::from_vec(n, n, v_c64);

        // the matrix is not in the good position
        matrix.transpose_mut();

        // transforms each modifier to its complex equivalent
        let identity_mod: Complex<f64> = Complex { re: id_mod, im: 0. };
        let matrix_mod: Complex<f64> = Complex { re: ma_mod, im: 0. };
        let vector_mod: Complex<f64> = Complex { re: ve_mod, im: 0. };

        // same, create solution vecto equivalent
        let mut solution: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 0., im: 0. }; n]);

        // call the real solver function
        solve_system(&matrix, &mut solution, identity_mod, matrix_mod, vector_mod);

        // copy everything back to the receiver vector
        for i in 0..n {
            receiver[i] = solution[i].re;
        }

        true
    }
}

// =================================================================================================
// these functions are to test new features

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

pub fn remove_clean_facts(matrix: &[f64]) -> Vec<f64> {
    let n_square = matrix.len();

    let n = (n_square as f64).sqrt() as usize;

    let mut clean_index_vec: Vec<usize> = Vec::new();

    for index in 0..n {
        let mut index_is_clean = true;

        for j in 0..n {
            index_is_clean =
                index_is_clean && matrix[n * index + j] == 0_f64 && matrix[n * j + index] == 0_f64;

            if !index_is_clean {
                break;
            }
        }

        if index_is_clean {
            clean_index_vec.push(index);
        }
    }

    let mut new_matrix: Vec<f64> = vec![];

    for i in 0..n {
        if clean_index_vec.contains(&i) {
            continue;
        } else {
            for j in 0..n {
                if clean_index_vec.contains(&j) {
                    continue;
                } else {
                    new_matrix.push(matrix[n * i + j]);
                }
            }
        }
    }

    new_matrix
}

#[derive(Clone, Debug, PartialEq)]
pub struct UpperTriangle {
    n: usize,
    index: usize,
    limit: usize,
    current_state: (usize, usize),
}

impl UpperTriangle {
    // TODO: come back here and implement the chunks functionality
    pub fn new(n: usize, many_chunks: usize) -> Vec<UpperTriangle> {
        let global_limit = (n * (n - 1)) / 2;

        match many_chunks {
            1 => vec![UpperTriangle {
                n,
                index: 0,
                limit: global_limit,
                current_state: (0, 1),
            }],
            _ => {
                let elements_by_chunk = n / many_chunks;
                if elements_by_chunk <= 1 {
                    vec![UpperTriangle {
                        n,
                        index: 0,
                        limit: global_limit,
                        current_state: (0, 1),
                    }]
                } else {
                    vec![UpperTriangle {
                        n,
                        index: 0,
                        limit: global_limit,
                        current_state: (0, 1),
                    }]
                }
            }
        }
    }
}

impl Iterator for UpperTriangle {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.limit {
            None
        } else {
            let this_state = self.current_state;
            self.index += 1;

            if self.current_state.1 == self.n - 1 {
                self.current_state = (self.current_state.0 + 1, self.current_state.0 + 2);
            } else {
                self.current_state = (self.current_state.0, self.current_state.1 + 1);
            }

            Some(this_state)
        }
    }
}
