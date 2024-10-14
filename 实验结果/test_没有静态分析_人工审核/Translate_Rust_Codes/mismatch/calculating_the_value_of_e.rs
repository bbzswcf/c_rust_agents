fn main() {
    println!("The double precision in C gives about 15 significant digits.\n\
              Values below are presented with 16 digits after the decimal point.\n");

    // The most direct way to compute Euler constant.
    // Directly use `std::f64::consts::E` to get the Euler's constant.
    let e = std::f64::consts::E;
    println!("Euler constant e = {:.16}", e);

    // The fast and independent method: e = lim (1 + 1/n)^n
    // Correctly compute the power using a loop.
    let n = 1_000_000; // Increased n for higher precision
    let mut e = 1.0;
    let mut term = 1.0;
    for _ in 0..n {
        term /= n as f64;
        e += term;
    }
    println!("Euler constant e = {:.16}", e);

    // Taylor expansion e = 1 + 1/1! + 1/2! + 1/3! + ...
    // Use Kahan summation to improve accuracy.
    //
    const N: usize = 1000;
    let mut a = [1.0; N]; // Initialize the array `a` with `1.0` at all indices.
    for i in 1..N {
        a[i] = a[i - 1] / i as f64; // Correctly compute the factorial terms.
    }
    let mut e = 1.0;
    let mut c = 0.0; // Compensation term for Kahan summation
    // Iterate from `1` to `N-1` instead of reversing the range.
    for i in 1..N {
        let y = a[i] - c; // Compute the term minus the compensation
        let t = e + y; // Add the term to the sum
        c = (t - e) - y; // Compute the new compensation
        e = t; // Update the sum
    }
    println!("Euler constant e = {:.16}", e);
}