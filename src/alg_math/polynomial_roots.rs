
pub enum Method {
    OrigCauchy,
}

pub fn find_bound_on_polynomial_roots(polynomial: &[f64], max_coeff: f64, degree: usize, real_degree: usize, method: Method) -> f64 {
    match method {
        Method::OrigCauchy => find_bound_on_polynomial_roots_original_cauchy(polynomial, max_coeff, degree, real_degree),
    }
}

pub fn find_bound_on_polynomial_roots_original_cauchy(polynomial: &[f64], max_coeff: f64, degree: usize, real_degree: usize) -> f64 {
    let mut current_max = 0_f64;

    for i in 0..real_degree {
        let real_value_coeff = -polynomial[i];
        let result = real_value_coeff / max_coeff;
        current_max = current_max.max(result);
    }

    current_max
}