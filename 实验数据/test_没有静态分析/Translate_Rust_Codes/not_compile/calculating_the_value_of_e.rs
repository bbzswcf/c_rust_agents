// Modified: Added dependency for num-bigfloat in Cargo.toml
// [dependencies]
// num-bigfloat = "0.4"

// Modified: Added dependency for rayon in Cargo.toml
// [dependencies]
// rayon = "1.5"

use num_bigfloat::BigFloat;
use rayon::prelude::*;

fn main() {
    println!("The double precision in C gives about 15 significant digits.\n\
             Values below are presented with 16 digits after the decimal point.\n");

    // The most direct way to compute Euler constant.
    let e = f64::exp(1.0);
    println!("Euler constant e = {:.16}", e);

    // The fast and independent method: e = lim (1 + 1/n)**n
    // Modified: Increased the value of `n` to 65536 to improve precision further
    let n = 65536; // Increased from 32768 to 65536
    let e = (1.0 + 1.0 / n as f64).powf(n as f64);
    println!("Euler constant e = {:.16}", e);

    // Taylor expansion e = 1 + 1/1 + 1/2 + 1/2/3 + 1/2/3/4 + 1/2/3/4/5 + ...
    // Modified: Increased the number of terms in the Taylor expansion to 8000 to improve precision further
    // Modified: Using `num-bigfloat` crate for higher precision arithmetic

    const N: usize = 8000; // Increased from 4000 to 8000
    let mut a = Vec::with_capacity(N); // Modified: Changed vector initialization to use a loop
    for _ in 0..N {
        a.push(BigFloat::from(0.0));
    }
    a[0] = BigFloat::from(1.0);

    // Modified: Used parallel iterators from the `rayon` crate to speed up the computation
    a.par_iter_mut().enumerate().skip(1).for_each(|(i, ai)| {
        *ai = &a[i - 1] / BigFloat::from(i as f64);
    });

    let mut e = BigFloat::from(1.0);
    for i in (1..N).rev() {
        e += &a[i];
    }

    // Modified: Converted BigFloat to string with desired precision before printing
    let e_str = format!("{:.16}", e);
    println!("Euler constant e = {}", e_str);
}