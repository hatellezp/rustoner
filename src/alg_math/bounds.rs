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
   This module has only one important function: find_bound_complex.
   From a matrix computes a bound for stabilized results.
   Not the best place to completely explain this algorithm.
   Go read one of the papers that specifies this, like the following:

   https://soft.vub.ac.be/DBDBD2020/abstract/dbdbd_2020_tellez.pdf

   good luck.
*/

use rayon::iter::ParallelBridge;
use rayon::prelude::*;

use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use nalgebra::Complex;
use nalgebra::{DMatrix, DVector};

// use num_traits::{ Float, Zero };

use crate::alg_math::utilities::{
    matrix_is_zero_complex, matrix_subtraction, multiply_matrix_complex, multiply_vector_complex,
    output_unity_root, remove_clean_facts, round_to_15_f64, UpperTriangle,
};

use crate::alg_math::polynomial_roots::{find_bound_on_polynomial_roots, Method};
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn find_bound_complex_switcher(
    matrix: &mut DMatrix<Complex<f64>>,
    tolerance: f64,
    m_scale: f64,
    _b_translate: f64,
    use_concurrency: bool,
) -> Option<f64> {
    if use_concurrency {
        find_bound_complex_concurrency(matrix, tolerance, m_scale, _b_translate)
    } else {
        find_bound_complex_linear(matrix, tolerance, m_scale, _b_translate)
    }
}

fn find_bound_complex_concurrency(
    matrix: &mut DMatrix<Complex<f64>>,
    tolerance: f64,
    m_scale: f64,
    _b_translate: f64,
) -> Option<f64> {
    // I love to keep values these values such that I can stop calling for them
    let rows = matrix.nrows();
    let cols = matrix.ncols();

    if rows != cols {
        // verify that the matrix is square
        println!("matrix is not square!");
        Option::None
    } else {
        /*
           The matrix dimension is n thus each polynomial that defines the behaviour
           of each component of the solution vector is at most of degree n-2.
           Therefore, we need n-1 samples, this comes down to N = n-1 vectors of dimension n.
        */
        let n = rows;

        // set to zero the diagonal, we don't want any troubles as a corrupted entry in the matrix
        for i in 0..n {
            matrix[(i, i)] = Complex { re: 0., im: 0. };
        }

        // if the matrix is null the bound is 1, easy peasy
        if matrix_is_zero_complex(matrix) {
            return Some(1_f64);
        }

        let n_samples = n + 1; // n - 1; // we need then a n-1 samples
        let mut prov_scale: f64 = 1.; // the matrix need to be scaled, we begin at 1
        let mut current_max: f64; // find the biggest value in the matrix
        let inverse_roots = true; // for the fast fourier transform, we want to go back
                                  // so inverse is set to true

        let mut find_line_maxes_one =
            |inner_matrix: &DMatrix<Complex<f64>>, index: usize, size: usize| {
                let mut current_max = 0.;

                for j in 0..size {
                    // let complex_abs = (&matrix)[(i, j)].norm_sqr().sqrt();
                    let complex_abs = (inner_matrix)[(index, j)].norm_sqr().sqrt();

                    current_max += complex_abs;
                }

                current_max
            };

        prov_scale = (0..n)
            .into_par_iter()
            .map(|x| find_line_maxes_one(matrix, x, n))
            .reduce_with(|x1, x2| x1.max(x2))
            .unwrap();

        // now prov_scale will assure a non singular matrix
        prov_scale += m_scale; // this number guaranteed that 1/prov_scale < 1 strictly
        let scale = Complex {
            // cast to complex
            re: (1. / prov_scale),
            im: 0.,
        };

        // scale the matrix to guaranteed invertibility
        // multiply_matrix_complex(&mut matrix, scale);
        multiply_matrix_complex(matrix, scale);

        /*
            so at this point the matrix has been scaled (don't forget this)
            and now (a*1-m) will be invertible whenever |a| >= 1
            because the sum of each row of our matrix as the property |sum of row| <= 1/(1.1)
        */

        // vector that stores every result
        // N samples
        // n dimension
        // 2 fields (for real and complex)
        // better explanation impossible
        let mut pvalues: Vec<f64> = vec![0.; n_samples * n * 2];

        /*
             so now we make a loop where for each unity root un we compute
             det(un*1 - m) * [(un*1 - m)^(-1) (1,...1)]
             this is vector, and we stored in the pvalues array
        */

        let mut identity: DMatrix<Complex<f64>> =
            DMatrix::from_vec(n, n, vec![Complex { re: 0., im: 0. }; n * n]);
        identity.fill_with_identity();
        let right_vector: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 1., im: 0. }; n]);

        // loop that solves the system N times and populates pvalues with the solution vectors

        let fill_line = |n_value: &usize,
                         nbr_samples: &usize,
                         index: &usize,
                         roots_for_or: &bool,
                         inner_identity: &DMatrix<Complex<f64>>,
                         inner_matrix: &DMatrix<Complex<f64>>,
                         receiver: &mut [f64]| {
            let mut this_identity = inner_identity.clone();
            this_identity.fill_with_identity();

            let root = output_unity_root(*nbr_samples, *index, *roots_for_or);

            multiply_matrix_complex(&mut this_identity, root);
            matrix_subtraction(&mut this_identity, matrix);

            let decomp = this_identity.lu();

            let mut x: DVector<Complex<f64>> = decomp.solve(&right_vector).unwrap(); // usually this is always solvable
            let det = decomp.determinant();

            /*
             now we have the x vector that keeps the solution
             and det that have the determinant
             we perform det*x now
            */
            multiply_vector_complex(&mut x, det);

            for ind_vector in 0..*n_value {
                // note here that we have to cut each complex number in two real values

                receiver[ind_vector] = x[ind_vector].re;
                receiver[ind_vector + 1] = x[ind_vector].im;
            }
        };

        pvalues
            .par_chunks_exact_mut(n * 2)
            .enumerate()
            .for_each(|(index, chunk)| {
                fill_line(
                    &n,
                    &n_samples,
                    &index,
                    &inverse_roots,
                    &identity,
                    &matrix,
                    chunk,
                )
            });
        // eprint!("{:?}\n", &pvalues);

        /*
            we finished populate the pvalues array, now we need to compute each polynomial using the
            fftw3 plan
            we have to compute n*(n+1)/2 polynomials
            these variables are to compute the real degree of each polynomial interpolated and
            compute then the bound on the roots
        */

        /*
           each polynomial is not necessarily of degree n-2, to compute the bound (Cauchy bound)
            on the roots we need the real degree of each of those polynomials
        */
        current_max = 1.; // to avoid zero related problems

        let find_bound = |values_sender: &[f64], index_pi: &usize, index_pj: &usize| {
            let mut in_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
            let mut out_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
            let mut plan: C2CPlan64 =
                C2CPlan::aligned(&[n_samples], Sign::Backward, Flag::MEASURE).unwrap();

            // populate the in vector
            for ind_sample in 0..n_samples {
                let real = values_sender[n_samples * ind_sample + *index_pi]
                    - values_sender[n_samples * ind_sample + *index_pj];
                let imag = values_sender[n_samples * ind_sample + *index_pi + 1]
                    - values_sender[n_samples * ind_sample + *index_pj + 1];

                // this populates the in_vector for the fast fourier transform
                in_vector[ind_sample] = c64::new(real, imag);
            }

            /*
               once the plan is executed the out_vector has the solution, which by our
               choice of values (the unity roots) has the coefficients of the wanted
               polynomial
            */
            plan.c2c(&mut in_vector, &mut out_vector).unwrap(); // TODO: is this safe ??
                                                                // now out_vector has the result
                                                                // now the answer is stored in out
                                                                // I'm reusing the current_max double
                                                                // find the real degree of the polynomial
                                                                // I think these updates are not necessary ...
                                                                // tolerance is there to avoid 0 related problems

            // we still need to find the real degree

            // create vector from the real part of out_vector
            // I think I will round every element that I pass to the polynomial
            // rounding is done before passing the polynomial to root finding algorithms
            let mut polynomial: Vec<f64> = out_vector
                .iter()
                .map(|x| round_to_15_f64(x.re))
                .collect::<Vec<f64>>();

            let polynomial_length = polynomial.len();
            let bound_found = find_bound_on_polynomial_roots(
                &mut polynomial,
                polynomial_length,
                tolerance,
                Method::CauchyCubic,
            );

            // println!("    bound found is: {}", bound_found);
            bound_found
        };

        // TODO: find a way to avoid calling a parallel computing inside a parallel computing

        let upper_triangle = UpperTriangle::new(n, 1)[0].to_owned();
        let val = upper_triangle
            .par_bridge()
            .map(|(pi, pj)| find_bound(&pvalues, &pi, &pj))
            .reduce_with(|x1, x2| x1.max(x2))
            .unwrap();

        current_max = val.max(current_max);

        // now unscale (the value was scaled because of the matrix) and deplace to avoid
        // undefined effects near the bound
        let bound = (current_max) * (0.5 + prov_scale);

        Some(bound)
    }
}

/// will compute a bound for the 'a' value (you should know what it is)
/// such that solutions for the matrix equation given by the matrix  'matrix'
/// are rank equivalent (you should also know what it is)
fn find_bound_complex_linear(
    matrix: &mut DMatrix<Complex<f64>>,
    tolerance: f64,
    m_scale: f64,
    _b_translate: f64,
) -> Option<f64> {
    // I love to keep values these values such that I can stop calling for them
    let rows = matrix.nrows();
    let cols = matrix.ncols();

    if rows != cols {
        // verify that the matrix is square
        println!("matrix is not square!");
        Option::None
    } else {
        /*
           The matrix dimension is n thus each polynomial that defines the behaviour
           of each component of the solution vector is at most of degree n-2.
           Therefore, we need n-1 samples, this comes down to N = n-1 vectors of dimension n.
        */
        let n = rows;

        // set to zero the diagonal, we don't want any troubles as a corrupted entry in the matrix
        for i in 0..n {
            matrix[(i, i)] = Complex { re: 0., im: 0. };
        }

        // if the matrix is null the bound is 1, easy peasy
        if matrix_is_zero_complex(matrix) {
            return Some(1_f64);
        }

        let n_samples = n + 1; // n - 1; // we need then a n-1 samples
        let mut prov_scale: f64 = 1.; // the matrix need to be scaled, we begin at 1
        let mut current_max: f64; // find the biggest value in the matrix
        let inverse_roots = true; // for the fast fourier transform, we want to go back
                                  // so inverse is set to true

        // from the provisional scale we need to find a better bound so the matrix is
        // not singular when added to the identity matrix
        for i in 0..n {
            current_max = 0.;

            for j in 0..n {
                // let complex_abs = (&matrix)[(i, j)].norm_sqr().sqrt();
                let complex_abs = (matrix)[(i, j)].norm_sqr().sqrt();

                current_max += complex_abs;
            }
            prov_scale = current_max.max(prov_scale);
        }

        // now prov_scale will assure a non singular matrix
        prov_scale += m_scale; // this number guaranteed that 1/prov_scale < 1 strictly
        let scale = Complex {
            // cast to complex
            re: (1. / prov_scale),
            im: 0.,
        };

        // scale the matrix to guaranteed invertibility
        // multiply_matrix_complex(&mut matrix, scale);
        multiply_matrix_complex(matrix, scale);

        /*
            so at this point the matrix has been scaled (don't forget this)
            and now (a*1-m) will be invertible whenever |a| >= 1
            because the sum of each row of our matrix as the property |sum of row| <= 1/(1.1)
        */

        // create vector for unity roots, this is a fast fourier transform, we need
        // n-1 unity roots to be capable of interpolate

        // the block below was changed to a unique function that supplies on demand the root,
        // no need to allocate memory for this
        /*
        let mut roots: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 0., im: 0. }; n_samples]);
        create_unity_roots(&mut roots, n_samples, inverse_roots);

         */

        // vector that stores every result
        // N samples
        // n dimension
        // 2 fields (for real and complex)
        // better explanation impossible
        let mut pvalues: Vec<f64> = vec![0.; n_samples * n * 2];

        // FFTW (the original written in C) need to initialize a plan with the specific dimension
        // the rust version is wrapper over the C implementation and thus follows the same
        // strategy
        let mut in_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
        let mut out_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
        let mut plan: C2CPlan64 =
            C2CPlan::aligned(&[n_samples], Sign::Backward, Flag::MEASURE).unwrap();

        /*
             so now we make a loop where for each unity root un we compute
             det(un*1 - m) * [(un*1 - m)^(-1) (1,...1)]
             this is vector, and we stored in the pvalues array
        */

        let mut identity: DMatrix<Complex<f64>> =
            DMatrix::from_vec(n, n, vec![Complex { re: 0., im: 0. }; n * n]);
        identity.fill_with_identity();
        let mut root: Complex<f64>;
        let right_vector: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 1., im: 0. }; n]);
        let mut det: Complex<f64>;

        // loop that solves the system N times and populates pvalues with the solution vectors
        for ind_sample in 0..n_samples {
            // refill with identity
            identity.fill_with_identity();

            // retrieve the current root
            // root = roots[ind_sample];
            // now root is a simple call and no need to store the information somewhere
            root = output_unity_root(n_samples, ind_sample, inverse_roots);

            // compute (un*1 - m)
            multiply_matrix_complex(&mut identity, root);
            // matrix_subtraction(&mut identity, &matrix);
            matrix_subtraction(&mut identity, matrix);

            // TODO: come back here and read the nalgebra documentation
            //       I think there is something that can be done
            let decomp = identity.clone().lu();

            /*
               in any case know that LU decomposition is the faster, but not the
               most stable, in our case stability is good enough thanks to the
               Fourier approach, thus for decomposition we prioritize speed
            */

            let mut x: DVector<Complex<f64>> = decomp.solve(&right_vector).unwrap(); // usually this is always solvable
            det = decomp.determinant();

            /*
             now we have the x vector that keeps the solution
             and det that have the determinant
             we perform det*x now
            */
            multiply_vector_complex(&mut x, det);

            /*
             the vector x has the sample that we need to store in the pvalues array
             this populate the pvalues array
            */
            for ind_vector in 0..n {
                // note here that we have to cut each complex number in two real values
                pvalues[n_samples * ind_sample + ind_vector] = x[ind_vector].re;
                pvalues[n_samples * ind_sample + ind_vector + 1] = x[ind_vector].im;
            }
        }

        /*
            we finished populate the pvalues array, now we need to compute each polynomial using the
            fftw3 plan
            we have to compute n*(n+1)/2 polynomials
            these variables are to compute the real degree of each polynomial interpolated and
            compute then the bound on the roots
        */

        /*
           each polynomial is not necessarily of degree n-2, to compute the bound (Cauchy bound)
            on the roots we need the real degree of each of those polynomials
        */
        current_max = 1.; // to avoid zero related problems

        for ind_pi in 0..(n - 1) {
            for ind_pj in (ind_pi + 1)..n {
                // populate the in vector
                for ind_sample in 0..n_samples {
                    let real = pvalues[n_samples * ind_sample + ind_pi]
                        - pvalues[n_samples * ind_sample + ind_pj];
                    let imag = pvalues[n_samples * ind_sample + ind_pi + 1]
                        - pvalues[n_samples * ind_sample + ind_pj + 1];

                    // this populates the in_vector for the fast fourier transform
                    in_vector[ind_sample] = c64::new(real, imag);
                }

                /*
                   once the plan is executed the out_vector has the solution, which by our
                   choice of values (the unity roots) has the coefficients of the wanted
                   polynomial
                */
                plan.c2c(&mut in_vector, &mut out_vector).unwrap(); // TODO: is this safe ??
                                                                    // now out_vector has the result
                                                                    // now the answer is stored in out
                                                                    // I'm reusing the current_max double
                                                                    // find the real degree of the polynomial
                                                                    // I think these updates are not necessary ...
                                                                    // tolerance is there to avoid 0 related problems

                // we still need to find the real degree

                // create vector from the real part of out_vector
                // I think I will round every element that I pass to the polynomial
                // rounding is done before passing the polynomial to root finding algorithms
                let mut polynomial: Vec<f64> = out_vector
                    .iter()
                    .map(|x| round_to_15_f64(x.re))
                    .collect::<Vec<f64>>();

                let polynomial_length = polynomial.len();
                let bound_found = find_bound_on_polynomial_roots(
                    &mut polynomial,
                    polynomial_length,
                    tolerance,
                    Method::CauchyCubic,
                );

                /*
                let methods = [
                    // Method::CauchyOriginal,
                    // Method::CauchySquare,
                    Method::CauchyCubic,
                    // Method::CauchyQuad,
                ];
                let polynomial_length = polynomial.len();
                let mut bound_found = 0_f64;
                let mut a_bound_was_found = false;

                // println!("polynomial: {:?}", &polynomial);
                for method in &methods {
                    let new_bound_found = find_bound_on_polynomial_roots(
                        &mut polynomial,
                        polynomial_length,
                        tolerance,
                        *method,
                    );

                    // println!("  --  {:?}: {}", method, new_bound_found);

                    if a_bound_was_found {
                        bound_found = bound_found.min(new_bound_found);
                    } else {
                        bound_found = new_bound_found;
                        a_bound_was_found = true;
                    }
                }
                // println!("");
                // println!("-----------------------------------------------------------------------");

                 */

                // update, don't overwrite

                // println!("current max before update: {}", current_max);
                current_max = current_max.max(bound_found);
                // println!("current max after update: {}", current_max);
            }
        }

        // now unscale (the value was scaled because of the matrix) and deplace to avoid
        // undefined effects near the bound
        // let bound = (current_max + b_translate) * (1. * prov_scale);

        // println!("prebound: {} and scale: {}", current_max, prov_scale);
        let bound = (current_max) * (0.5 + prov_scale);
        // println!("  --  the bound found is : {}", bound);

        Some(bound)
    }
}

/// this is wrapper over the find_bound_complex function, it takes f64 values
/// and construct the necessary DMatrix to compute a bound
pub fn find_bound_complex_wrapper(
    v: Vec<f64>,
    tolerance: f64,
    m_scale: f64,
    b_translate: f64,
    use_concurrency: bool,
) -> Option<f64> {
    let nsquared = v.len();
    let n = (nsquared as f64).sqrt() as usize;

    if n * n != nsquared {
        // always verify the array can be casted to an square matrix
        println!(
            "not an square matrix can be formed, {} is not a perfect square!",
            nsquared
        );
        Option::None
    } else {
        let pre_matrix = remove_clean_facts(&v);

        let nsquared = pre_matrix.len();
        let n = (nsquared as f64).sqrt() as usize;

        let matrix: DMatrix<f64> = DMatrix::from_vec(n, n, pre_matrix);
        let mut matrix: DMatrix<Complex<f64>> = matrix.cast::<Complex<f64>>();

        find_bound_complex_switcher(
            &mut matrix,
            tolerance,
            m_scale,
            b_translate,
            use_concurrency,
        )
    }
}

// type for the numeric adjusters
pub type Adjusters = (f64, f64, f64);
