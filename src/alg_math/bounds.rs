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

use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use nalgebra::{DMatrix, DVector};
use nalgebra::Complex;

use crate::alg_math::utilities::{
    create_unity_roots, matrix_is_zero_complex, matrix_subtraction, multiply_matrix_complex,
    multiply_vector_complex, round_to_15_f64,
};

/// will compute a bound for the 'a' value (you should know what it is)
/// such that solutions for the matrix equation given by the matrix  'matrix'
/// are rank equivalent (you should also know what it is)
fn find_bound_complex(
    mut matrix: DMatrix<Complex<f64>>,
    tolerance: f64,
    m_scale: f64,
    b_translate: f64,
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
        if matrix_is_zero_complex(&matrix) {
            return Some(1_f64);
        }

        let n_samples = n - 1; // we need then a n-1 samples
        let mut prov_scale: f64 = 1.; // the matrix need to be scaled, we begin at 1
        let mut current_max: f64; // find the biggest value in the matrix
        let inverse_roots = true; // for the fast fourier transform, we want to go back
                                  // so inverse is set to true

        // from the provisional scale we need to find a better bound so the matrix is
        // not singular when added to the identity matrix
        for i in 0..n {
            current_max = 0.;

            for j in 0..n {
                let complex_abs = (&matrix)[(i, j)].norm_sqr().sqrt();

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
        multiply_matrix_complex(&mut matrix, scale);

        /*
            so at this point the matrix has been scaled (don't forget this)
            and now (a*1-m) will be invertible whenever |a| >= 1
            because the sum of each row of our matrix as the property |sum of row| <= 1/(1.1)
        */

        // create vector for unity roots, this is a fast fourier transform, we need
        // n-1 unity roots to be capable of interpolate
        let mut roots: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 0., im: 0. }; n_samples]);
        create_unity_roots(&mut roots, n_samples, inverse_roots);

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
            root = roots[ind_sample];

            // compute (un*1 - m)
            multiply_matrix_complex(&mut identity, root);
            matrix_subtraction(&mut identity, &matrix);

            // temp indentity to LU decomposition
            let mut temp_identity: DMatrix<Complex<f64>> =
                DMatrix::from_vec(n, n, vec![Complex { re: 0., im: 0. }; n * n]);
            temp_identity.copy_from(&identity);

            // solve the system and compute also the determinant
            let decomp = temp_identity.lu();
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
        let mut possible_coeff_real: f64;
        let mut max_coeff: f64;
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
                let mut real_degree: Option<usize> = Option::None;

                // the tolerance value set what we consider as zero or not
                possible_coeff_real = tolerance / 2.;
                max_coeff = tolerance / 2.;

                for i in 0..n_samples {
                    // this scaling is unnecessary
                    possible_coeff_real = out_vector[n_samples - i - 1].re;

                    /*
                    also here I'm introducing the rounding, this purges rounding errors due
                    to sine and cosine computations
                     */
                    possible_coeff_real = round_to_15_f64(possible_coeff_real);

                    if possible_coeff_real.abs() > tolerance {
                        real_degree = Some(n_samples - i - 1);
                        max_coeff = possible_coeff_real;
                        break;
                    } else {
                        continue;
                    }
                }

                // recast real_degree
                let real_degree = real_degree.unwrap_or(0); // something bad happened here

                // now that we have the real degree we compute the bound
                // where the max comprehend all polynomials
                for i in 0..real_degree {
                    let real_value_coeff: f64 = -out_vector[i].re;
                    let result = real_value_coeff / max_coeff;
                    current_max = current_max.max(result);
                }
            }
        }

        // now unscale (the value was scaled because of the matrix) and deplace to avoid
        // undefined effects near the bound
        let bound = (current_max + b_translate) * (1. * prov_scale);

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
        let matrix: DMatrix<f64> = DMatrix::from_vec(n, n, v);
        let matrix: DMatrix<Complex<f64>> = matrix.cast::<Complex<f64>>();

        find_bound_complex(matrix, tolerance, m_scale, b_translate)
    }
}

// type for the numeric adjusters
pub type Adjusters = (f64, f64, f64);
