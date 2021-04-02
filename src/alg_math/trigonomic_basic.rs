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

// all constants are defined here

use nalgebra::{Complex, DVector};
use std::collections::HashMap;

// this are the inverse of factorial up to 1/21!
const INV_FACT1: f64 = 1.00000000000000000000000000000000000;
const INV_FACT2: f64 = 0.500000000000000000000000000000000000;
const INV_FACT3: f64 = 0.166666666666666666666666666666666659;
const INV_FACT4: f64 = 0.0416666666666666666666666666666666647;
const INV_FACT5: f64 = 0.00833333333333333333333333333333333323;
const INV_FACT6: f64 = 0.00138888888888888888888888888888888893;
const INV_FACT7: f64 = 0.000198412698412698412698412698412698413;
const INV_FACT8: f64 = 2.48015873015873015873015873015873016e-05;
const INV_FACT9: f64 = 2.75573192239858906525573192239858902e-06;
const INV_FACT10: f64 = 2.75573192239858906525573192239858911e-07;
const INV_FACT11: f64 = 2.50521083854417187750521083854417184e-08;
const INV_FACT12: f64 = 2.08767569878680989792100903212014332e-09;
const INV_FACT13: f64 = 1.60590438368216145993923771701549473e-10;
const INV_FACT14: f64 = 1.14707455977297247138516979786821058e-11;
const INV_FACT15: f64 = 7.64716373181981647590113198578807052e-13;
const INV_FACT16: f64 = 4.77947733238738529743820749111754408e-14;
const INV_FACT17: f64 = 2.81145725434552076319894558301032005e-15;
const INV_FACT18: f64 = 1.56192069685862264622163643500573343e-16;
const INV_FACT19: f64 = 8.22063524662432971695598123687228114e-18;
const INV_FACT20: f64 = 4.11031762331216485847799061843614049e-19;
const INV_FACT21: f64 = 1.95729410633912612308475743735054300e-20;

// some other necessary ones
const ROOT_OF_2: f64 = 1.41421356237309504880168872420969798;
const ROOT_OF_2_OVER_2: f64 = 0.707106781186547524400844362104848992;

// my one definition of pi related values
const PI_REDEF: f64 = 3.14159265358979323845999999999999997;
const PI_TIMES_2_REDEF: f64 = 6.28318530717958647691999999999999995;
const PI_OVER_2_REDEF: f64 = 1.57079632679489661922999999999999999;
const PI_OVER_4_REDEF: f64 = 0.785398163397448309614999999999999994;

// this constants are to find relatively good bounds for picking a degree in the taylor polynomial
const FIRST_BOUND_AFTER_ZERO: f64 = 7.45058059692382812500000000000000000e-09;
const SECOND_BOUND_AFTER_ZERO: f64 = 0.392699085424014453269414062499999997;
const FIRST_BOUND_AFTER_PI_OVER_4: f64 = 1.17809724137088216596058593749999994;
const LAST_BOUND_BEFORE_PI_OVER_2: f64 = 1.57079631934431602230617187499999999;

// to rounding
const PRECISION_ROUNDER: f64 = 1000000000000000.;

pub fn compute_cosine(x: f64) -> f64 {
    compute_sine(PI_OVER_2_REDEF - x)
}

pub fn compute_cosine2(x: f64) -> f64 {
    compute_sine2(PI_OVER_2_REDEF - x)
}


pub fn compute_sine(x: f64) -> f64 {
    if x < 0_f64 {
        -compute_sine(-x)
    } else if x == 0_f64 || x == PI_REDEF {
        0_f64
    } else if x == PI_OVER_2_REDEF {
        1_f64
    } else if x > PI_REDEF {
        -compute_sine(x - PI_REDEF)
    } else {
        let x2 = x * x;
        let x3 = x2 * x;
        let x5 = x3 * x2;
        let x7 = x5 * x2;
        let x9 = x7 * x2;
        let x11 = x9 * x2;
        let x13 = x11 * x2;
        let x15 = x13 * x2;
        let x17 = x15 * x2;
        let x19 = x17 * x2;
        let x21 = x19 * x2;

        let upper_half = x - x3 * INV_FACT3 + x5 * INV_FACT5 - x7 * INV_FACT7 + x9 * INV_FACT9
            - x11 * INV_FACT11;
        let lower_half = x13 * INV_FACT13 - x15 * INV_FACT15 + x17 * INV_FACT17 - x19 * INV_FACT19
            + x21 * INV_FACT21;

        upper_half + lower_half
    }
}

pub fn compute_sine2(x: f64) -> f64 {
    if x == 0_f64 || x == PI_REDEF {
        0_f64
    } else if x == PI_OVER_2_REDEF {
        1_f64
    } else if x < 0_f64 {
        -compute_sine2(-x)
    } else if x > PI_TIMES_2_REDEF {
        compute_sine2(x - PI_TIMES_2_REDEF)
    } else if x > PI_REDEF {
        -compute_sine2(x - PI_REDEF)
    } else if PI_OVER_2_REDEF < x && x < PI_REDEF {
        compute_sine_kernel(PI_REDEF - x)
    } else {
        compute_sine_kernel(x)
    }
}

/*
this works as:
0
FIRST_BOUND_AFTER_ZERO
values
SECOND_BOUND_AFTER_ZERO
values
PI_OVER_4_REDEF
values
FIRST_BOUND_AFTER_PI_OVER_4
values
LAST_BOUND_BEFIRE_PI_OVER_2
values
PI_OVER_2_REDEF
 */

// this function only takes arguments in ]0, pi/2[
pub fn compute_sine_kernel(x: f64) -> f64 {
    // we take several levels of approximation:
    // if x if near zero then x - x^3/3! (taylor series of sine at 0)
    // if x is near zero  but not that near --> degree 13 as the original
    // if x is near pi/2 the approximate pi/2 + (pi/2 - x) = pi - x with sine degree 3 at pi/2
    // if x is near pi/2 but not that near degree 13 as the original
    // if x is between 0 and pi/2 then taylor series of degree 15 (21?) at pi/4

    if x < FIRST_BOUND_AFTER_ZERO {
        // x is almost zero x in [0, FIRST_BOUND[
        // we approximate by x-x^3/3!

        let x2 = x * x;
        x - x * x2 * INV_FACT3
    } else if FIRST_BOUND_AFTER_ZERO <= x && x < SECOND_BOUND_AFTER_ZERO {
        // x is bigger that zero but still far away from pi/4
        // here we approximate with taylor series of degree 13

        let x2 = x * x;
        let x3 = x2 * x;
        let x5 = x3 * x2;
        let x7 = x5 * x2;
        let x9 = x7 * x2;
        let x11 = x9 * x2;
        let x13 = x11 * x2;

        let upper_half = x - x3 * INV_FACT3 + x5 * INV_FACT5 - x7 * INV_FACT7;
        let lower_half = x9 * INV_FACT9 - x11 * INV_FACT11 + x13 * INV_FACT13;

        upper_half + lower_half

    } else if SECOND_BOUND_AFTER_ZERO <= x && x < FIRST_BOUND_AFTER_PI_OVER_4 {
        // x is in the interval around pi/4
        // here we need taylor series of sine at pi/4 with precision 15 (21?)

        // definition of polynomial
        // put y = x - pi/4
        // root2/2 * (1 + y - y^2 * 1/2! - y^3 * 1/3! + y^4 * 1/4! + y^5 * 1/5! ... - y^15 * 1/15!)

        let y = x - PI_OVER_4_REDEF;
        let x2 = y * y;
        let x3 = x2 * y;
        let x4 = x2 * x2;
        let x5 = x3 * x2;
        let x6 = x4 * x2;
        let x7 = x5 * x2;
        let x8 = x6 * x2;
        let x9 = x7 * x2;
        let x10 = x8 * x2;
        let x11 = x9 * x2;
        let x12 = x10 * x2;
        let x13 = x11 * x2;
        let x14 = x12 * x2;
        let x15 = x13 * x2;

        let upper_half = y - x2 * INV_FACT2 - x3 * INV_FACT3 + x4 * INV_FACT4 + x5 * INV_FACT5
            - x6 * INV_FACT6
            - x7 * INV_FACT7;
        let lower_half = x8 * INV_FACT8 + x9 * INV_FACT9 - x10 * INV_FACT10 - x11 * INV_FACT11
            + x12 * INV_FACT12
            + x13 * INV_FACT13
            - x14 * INV_FACT14
            - x15 * INV_FACT15;

        let almost = upper_half + lower_half;
        ROOT_OF_2_OVER_2 * (1_f64 + almost)

    } else if FIRST_BOUND_AFTER_PI_OVER_4 <= x && x < LAST_BOUND_BEFORE_PI_OVER_2 {
        // x is after interval around pi/4 but far from pi/2
        // same as the last case but with more precision

        let y = PI_OVER_2_REDEF - x;
        let x2 = y * y;
        let x4 = x2 * x2;
        let x6 = x4 * x2;
        let x8 = x6 * x2;
        let x10 = x8 * x2;
        let x12 = x10 * x2;
        let x14 = x12 * x2;
        let x16 = x14 * x2;

        let upper_half = -x2 * INV_FACT2 + x4 * INV_FACT4 - x6 * INV_FACT6 + x8 * INV_FACT8;
        let lower_half = -x10 * INV_FACT10 + x12 * INV_FACT12 - x14 * INV_FACT12 + x16 * INV_FACT16;

        1_f64 + upper_half + lower_half
    } else {
        // x > LAST_BOUND_BEFORE_PI_OVER_2 and is near pi/2
        // around pi/2 sine is symmetric, then for x = pi/2 - e we compute y = pi/2 + e
        // in the taylor series this is (y - pi/2) = e = pi/2 - x so we compute for this value

        // y = x - pi/2
        // taylor series at pi/2: 1 - y^2 * 1/2! + y^4 * 1/4!...

        let y = PI_OVER_2_REDEF - x;
        let x2 = y * y;
        let x4 = x2 * x2;

        let upper_half = -x2 * INV_FACT2 + x4 * INV_FACT4;

        1_f64 + upper_half
    }
}

#[inline]
pub fn round_to_15_f64(v: f64) -> f64 {
    (v * PRECISION_ROUNDER).round() / PRECISION_ROUNDER
}


// changing to accelerate this function
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

        let mut cashed_values: HashMap<usize, (f64, f64)> = HashMap::new();

        for k in 0..n {
            arg = t * PI_TIMES_2_REDEF * ((k as f64) / n64);
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
