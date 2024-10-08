use std::f64::consts::E;

fn main() {
    println!("The double precision in C gives about 15 significant digits.\n\
              Values below are presented with 16 digits after the decimal point.\n");

    // The most direct way to compute Euler constant.
    let e = E;
    println!("Euler constant e = {:.16}", e);

    // The fast and independent method: e = lim (1 + 1/n)^n
    let n = 8192;
    // Modified: Correctly raise (1 + 1/n) to the power of n using powf for floating-point exponentiation
    let e = (1.0 + 1.0 / n as f64).powf(n as f64);
    println!("Euler constant e = {:.16}", e);

    // Taylor expansion e = 1 + 1/1! + 1/2! + 1/3! + ...
    const N: usize = 1000;
    let mut a = [0.0; N];
    a[0] = 1.0; // Modified: Include the initial term 1.0 (which is 1/0!)
    for i in 1..N {
        // Modified: Correctly compute the factorial terms by dividing by i as f64
        a[i] = a[i - 1] / i as f64;
    }
    // Modified: Correctly sum the terms from 1/0! to 1/N!
    let mut e = 0.0;
    for i in 0..N {
        e += 1.0 / a[i];
    }
    println!("Euler constant e = {:.16}", e);
}