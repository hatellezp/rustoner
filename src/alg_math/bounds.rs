use fftw::array::AlignedVec;
use fftw::plan::*;
use fftw::types::*;
use std::f64::consts::PI;

pub fn dummy() -> AlignedVec<Complex> {
    let n = 128;
    let mut plan: C2CPlan64 = C2CPlan::aligned(&[n], Sign::Forward, Flag::MEASURE).unwrap();
    let mut a = AlignedVec::new(n);
    let mut b = AlignedVec::new(n);
    let k0 = 2.0 * PI / n as f64;
    for i in 0..n {
        a[i] = c64::new((k0 * i as f64).cos(), 0.0);
    }
    plan.c2c(&mut a, &mut b).unwrap();

    b
}


