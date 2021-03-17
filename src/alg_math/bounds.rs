use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;

use crate::alg_math::utilities::{
    create_unity_roots, matrix_is_zero_complex, matrix_subtraction, multiply_matrix_complex,
    multiply_vector_complex,
};
use nalgebra::Complex;
use nalgebra::{DMatrix, DVector};

fn find_bound_complex(
    mut matrix: DMatrix<Complex<f64>>,
    tolerance: f64,
    m_scaler: f64,
    b_translate: f64,
) -> Option<f64> {
    let rows = matrix.nrows();
    let cols = matrix.ncols();

    if rows != cols {
        println!("matrix is not square!");
        Option::None
    } else {
        // this is the hard function
        // the matrix is size n, the polynomial is at most of degree n-2,
        // we need then a n-1 samples
        // we need then N = n-1 samples, or vectors each with n dimension
        let n = rows;

        // set to zero the diagonal
        for i in 0..n {
            matrix[(i, i)] = Complex { re: 0., im: 0. };
        }

        // if the matrix is null the bound is 1.
        if matrix_is_zero_complex(&matrix) {
            return Some(1_f64);
        }

        // we can begin computation
        // we need then a n-1 samples
        // we need then N = n-1 samples, or vectors each with n dimension
        let n_samples = n - 1;
        let mut prov_scaler: f64 = 1.;
        let mut current_max: f64;
        let inverse_roots = true;

        // find the real value for scaler
        for i in 0..n {
            current_max = 0.;

            for j in 0..n {
                let complex_abs = (&matrix)[(i, j)].norm_sqr().sqrt();

                current_max += complex_abs;
            }
            prov_scaler = current_max.max(prov_scaler);
        }

        // what is this magic number ???
        prov_scaler += m_scaler; // OK, this number guaranteed that 1/prov_scaler < 1 strictly
                                 // inverse and to complex
        let scaler = Complex {
            re: (1. / prov_scaler),
            im: 0.,
        };

        // scale the matrix to guaranteed invertibility
        multiply_matrix_complex(&mut matrix, scaler);

        /*
            so at this point the matrix has been scaled (don't forget this)
            and now (a*1-m) will be invertible whenever |a| <= 1
            because the sum of each row of our matrix as the property |sum of row| <= 1/(1.1)
        */

        // create vector for unity roots
        let mut roots: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 0., im: 0. }; n_samples]);
        create_unity_roots(&mut roots, n_samples, inverse_roots);

        // vector that stores every result
        // N samples
        // n dimension
        // 2 fields (for real and complex)
        let mut pvalues: Vec<f64> = vec![0.; n_samples * n * 2];

        // initialize the plan;
        let mut in_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
        let mut out_vector: AlignedVec<c64> = AlignedVec::new(n_samples);
        let mut plan: C2CPlan64 =
            C2CPlan::aligned(&[n_samples], Sign::Backward, Flag::MEASURE).unwrap();

        /*
             so now we make a loop where for each unity root un we compute
             det(un*1 - m) * [(un*1 - m)^(-1) (1,...1)]
             this is vector, and we stored in the pvalues array
        */
        let _ind_vector: usize;
        let mut x: DVector<Complex<f64>> = DVector::from_vec(vec![Complex { re: 0., im: 0. }; n]);
        let mut identity: DMatrix<Complex<f64>> =
            DMatrix::from_vec(n, n, vec![Complex { re: 0., im: 0. }; n * n]);
        identity.fill_with_identity();
        let mut root: Complex<f64>;
        let right_vector: DVector<Complex<f64>> =
            DVector::from_vec(vec![Complex { re: 1., im: 0. }; n]);
        let mut det: Complex<f64>;

        // loop that solves the system N times and populates pvalues
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
            x = decomp.solve(&right_vector).unwrap(); // usually this is always solvable
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
                pvalues[n_samples * ind_sample + ind_vector] = x[ind_vector].re;
                pvalues[n_samples * ind_sample + ind_vector + 1] = x[ind_vector].im;
            }
        }

        /*
            we finished populate the pvalues array, now we need to compute each polynomial using the fftw3 plan
        */
        // we have to compute n*(n+1)/2 polynomials
        // these variables are to compute the real degree of each polynomial interpolated and compute then the bound
        // on the roots

        // apparently I must put real_degree as signed integer
        let _real_degree: isize; // there, solved it, real_degree is signed now

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
                    in_vector[ind_sample] = c64::new(real, imag);
                }

                // execute the plan
                plan.c2c(&mut in_vector, &mut out_vector).unwrap(); // TODO: is this safe ??
                                                                    // not out_vector has the result

                // now the answer is stored in out
                // I'm reusing the current_max double
                // find the real degree of the polynomial
                // I think these updates are not necessary ...
                // tolerance is there to avoid 0 related problems
                let mut real_degree: Option<usize> = Option::None;

                possible_coeff_real = tolerance / 2.;
                max_coeff = tolerance / 2.;

                for i in 0..n_samples {
                    possible_coeff_real = out_vector[n_samples - i - 1].re / (n_samples as f64);

                    if possible_coeff_real.abs() > tolerance {
                        real_degree = Some(n_samples - i - 1);
                        max_coeff = possible_coeff_real;
                        break;
                    } else {
                        continue;
                    }
                }

                // recast real_degree
                let real_degree = real_degree.unwrap_or(0); // something bad happenend here

                // now that we have the real degree we compute the bound
                // where the max comprehend all polynomials
                for i in 0..real_degree {
                    let real_value_coeff: f64 = -out_vector[i].re;
                    current_max = current_max.max(real_value_coeff / max_coeff);
                }
            }
        }

        // now unscale and deplace
        let bound = (current_max + b_translate) * (1. * prov_scaler);

        Some(bound)
    }
}

pub fn find_bound_complex_wrapper(
    v: Vec<f64>,
    tolerance: f64,
    m_scaler: f64,
    b_translate: f64,
) -> Option<f64> {
    let nsquared = v.len();
    let n = (nsquared as f64).sqrt() as usize;

    if n * n != nsquared {
        println!(
            "not an square matrix can be formed, {} is not a perfect square!",
            nsquared
        );
        Option::None
    } else {
        let matrix: DMatrix<f64> = DMatrix::from_vec(n, n, v);
        let matrix: DMatrix<Complex<f64>> = matrix.cast::<Complex<f64>>();

        find_bound_complex(matrix, tolerance, m_scaler, b_translate)
    }
}
