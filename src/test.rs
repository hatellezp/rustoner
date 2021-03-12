mod dl_lite;
mod interface;
mod kb;
mod alg_math;

use crate::alg_math::bounds::dummy;

fn main() {
   let a = dummy();
   println!("{:?}", &a);
}
