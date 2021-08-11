use roots::{find_roots_cubic, find_roots_quartic, Roots};

#[derive(Debug, Copy, Clone)]
pub enum Method {
    CauchyOriginal,
    CauchySquare,
    CauchyCubic,
    CauchyQuad,
}

// pub const PRINCIPAL_CUBIC_ROOT: (f64, f64) = (-0.5_f64, 0.5_f64 * 1.73205080757_f64);

/*
   something to know here, there is an assumption made for polynomials as arrays:
   the coefficients for bigger degrees are to the right, for bigger index in the
   array
*/

/*
   All bounds here are optimized for real positive roots
   - first:    if the leading coefficient is negative then we flip all coefficients,
               this is more of a way to normalize across different methods
   - second:   consider only negative coefficients, positive coefficients can be ignored,
               you can prove it yourself that this work,
               or simply go to: https://en.wikipedia.org/wiki/Geometrical_properties_of_polynomial_roots#Bounds_of_positive_real_roots
*/

pub fn find_bound_on_polynomial_roots(
    polynomial: &mut [f64],
    degree: usize,
    tolerance: f64,
    method: Method,
) -> f64 {
    // we still need to find the real degree
    let mut real_degree_op: Option<usize> = Option::None;

    // the tolerance value set what we consider as zero or not
    let mut possible_coeff_real = tolerance / 2.;
    let mut max_coeff = tolerance / 2.;

    for i in 0..(degree) {
        let current_degree = degree - i - 1;
        possible_coeff_real = polynomial[current_degree];

        if possible_coeff_real.abs() > tolerance {
            real_degree_op = Some(current_degree);
            max_coeff = possible_coeff_real;
            break;
        } else {
            continue;
        }
    }

    // real_degree holds the index of the real degree, is an index, thus can be used as an
    // index

    // not real degree found, everything is zero, we return 0 as the polynomial is null
    match real_degree_op {
        None => 0_f64,
        Some(real_degree) => {
            // at this point a real_degree has been found, thus we put the biggest coefficient with
            // a positive sign, regardless of before
            if polynomial[real_degree] < 0_f64 {
                for coeff in polynomial.iter_mut().take(real_degree + 1) {
                    *coeff = -(*coeff);
                }

                max_coeff = polynomial[real_degree]
            }
            // here first condition is satisfied, the leading coefficient is positive

            /*
               the arguments are the following:
                   - polynomial: a ref to an array of doubles
                   - max_coeff, the value of the coefficient of the biggest degree element in polynomial that is not zero
                   - real degree: the real degree of the polynomial stored, an index between 0 and 'degree', it
                     is a real index, you can (must) use it as an index
            */

            // if the polynomial has degree 3 or less we use original cauchy regardless of the
            // method chosen

            if real_degree < 4 {
                return find_bound_on_polynomial_roots_cauchy_original(
                    polynomial,
                    max_coeff,
                    real_degree,
                );
            }

            match method {
                Method::CauchyOriginal => find_bound_on_polynomial_roots_cauchy_original(
                    polynomial,
                    max_coeff,
                    real_degree,
                ),
                Method::CauchySquare => {
                    find_bound_on_polynomial_roots_cauchy_square(polynomial, max_coeff, real_degree)
                }
                Method::CauchyCubic => {
                    find_bound_on_polynomial_roots_cauchy_cubic(polynomial, max_coeff, real_degree)
                }
                Method::CauchyQuad => find_bound_on_polynomial_roots_cauchy_quadratic(
                    polynomial,
                    max_coeff,
                    real_degree,
                ),
            }
        }
    }
}

/*
   remember to always ignore positive coefficients
*/

// this is optimized for positive roots
pub fn find_bound_on_polynomial_roots_cauchy_original(
    polynomial: &[f64],
    max_coeff: f64,
    real_degree: usize,
) -> f64 {
    let mut current_max = 0_f64;

    // this is okay, we do not want to touch the leading coefficient, thus i goes up to
    // (real_degree - 1)
    for coeff in polynomial.iter().take(real_degree) {
        // always ignore positive coefficients
        if coeff < &0_f64 {
            current_max = current_max.max(-(*coeff) / max_coeff);
        }
    }

    1_f64 + current_max
}

pub fn find_bound_on_polynomial_roots_cauchy_square(
    polynomial: &[f64],
    max_coeff: f64,
    real_degree: usize,
) -> f64 {
    /*
       this method comes from: http://titan.princeton.edu/papers/claire/hertz-etal-99.ps
    */

    // first let find B, the max of absolute values for a_{2} to a_{real degree minus 1}

    let mut bound = 0_f64;
    let first_coeff = polynomial[0].min(0_f64).abs();

    // we start at the second coefficient and finish as always before the last one
    // this method also used the leading coefficient differently, no need to access it
    for coeff in polynomial.iter().take(real_degree).skip(1) {
        // always ignore positive coefficients
        if coeff < &0_f64 {
            bound = bound.max(-(*coeff) / max_coeff);
        }
    }

    // now we have b
    // find r and return it
    0.5 * (1_f64 + first_coeff)
        + 0.5
            * f64::sqrt(
                (1_f64 + first_coeff) * (1_f64 + first_coeff) - 4_f64 * (first_coeff - bound),
            )
}

pub fn find_bound_on_polynomial_roots_cauchy_cubic(
    polynomial: &[f64],
    max_coeff: f64,
    real_degree: usize,
) -> f64 {
    let mut bound = 0_f64;

    for coeff in polynomial.iter().take(real_degree - 2) {
        if coeff < &0_f64 {
            bound = bound.max(-(coeff) / max_coeff);
        }
    }

    let a_n1 = (polynomial[real_degree - 1]).min(0_f64).abs();
    let a_n2 = (polynomial[real_degree - 2]).min(0_f64).abs();

    let b = 2_f64 - a_n1;
    let c = 1_f64 - a_n1 - a_n2;

    // TODO: I'm sure this can be implemented by me !!!
    let r = find_roots_cubic(1_f64, b, c, -bound);

    1_f64
        + (match r {
            Roots::One(root) => root[0],
            Roots::Two(root) => (root[0].max(root[1])),
            Roots::Three(root) => (root[0].max(root[1].max(root[2]))),
            _ => 0_f64,
        })
}

pub fn find_bound_on_polynomial_roots_cauchy_quadratic(
    polynomial: &[f64],
    max_coeff: f64,
    real_degree: usize,
) -> f64 {
    let mut bound = 0_f64;

    for coeff in polynomial.iter().take(real_degree - 3) {
        if coeff < &0_f64 {
            bound = bound.max(-(*coeff) / max_coeff);
        }
    }

    let a_n1 = polynomial[real_degree - 1].min(0_f64).abs();
    let a_n2 = polynomial[real_degree - 2].min(0_f64).abs();
    let a_n3 = polynomial[real_degree - 3].min(0_f64).abs();

    let a = 1_f64;
    let b = 3_f64 - a_n1;
    let c = 3_f64 - 2_f64 * a_n1 - a_n2;
    let d = 1_f64 - a_n1 - a_n2 - a_n3;
    let r = find_roots_quartic(a, b, c, d, -bound);

    1_f64
        + (match r {
            Roots::One(root) => root[0],
            Roots::Two(root) => (root[0].max(root[1])),
            Roots::Three(root) => (root[0].max(root[1].max(root[2]))),
            Roots::Four(root) => (root[0].max(root[1].max(root[2].max(root[3])))),
            _ => 0_f64,
        })
}
