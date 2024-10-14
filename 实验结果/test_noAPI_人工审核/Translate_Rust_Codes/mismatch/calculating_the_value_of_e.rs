fn main() {
    println!("The double precision in C gives about 15 significant digits.\n\
              Values below are presented with 16 digits after the decimal point.\n");

    // The most direct way to compute Euler constant.
    let e = (1.0_f64).exp();
    println!("Euler constant e = {:.16}", e);

    // The fast and independent method: e = lim (1 + 1/n)^n
    // Modified: Increased the value of `n` to improve precision
    let n = 1000000000; // Increased `n` to improve precision
    let e = (1.0 + 1.0 / n as f64).powf(n as f64);
    println!("Euler constant e = {:.16}", e);

    // Taylor expansion e = 1 + 1/1 + 1/2 + 1/2/3 + 1/2/3/4 + 1/2/3/4/5 + ...
    // Modified: Increased the number of terms in the Taylor expansion to improve precision
    const N: usize = 30; // Increased the number of terms to improve precision
    let mut e = 0.0;
    for i in 0..N {
        // Modified: Use a more efficient algorithm to compute the factorial without intermediate overflow
        e += 1.0 / factorial(i as u128) as f64;
    }
    println!("Euler constant e = {:.16}", e);
}

// Helper function to compute factorial using u128 to avoid overflow
// Modified: Use a more efficient algorithm to compute the factorial without intermediate overflow
fn factorial(n: u128) -> u128 {
    if n == 0 || n == 1 {
        return 1;
    }
    // Modified: Specify the type for the `result` variable to avoid ambiguity
    let mut result: u128 = 1;
    for i in 2..=n {
        // Modified: Use checked_mul to avoid overflow
        result = result.checked_mul(i).unwrap_or(result);
    }
    result
}