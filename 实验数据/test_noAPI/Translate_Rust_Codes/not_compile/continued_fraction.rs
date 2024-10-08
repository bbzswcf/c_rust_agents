// Modified: Ensure the `num-bigfloat` crate is correctly added to your `Cargo.toml` file.
// If using Rust 2018 or later, verify that the crate is listed as a dependency.
// If using Rust 2015, ensure you have an `extern crate num_bigfloat;` declaration at the top of your file or module.
// If the crate name is misspelled or the crate is not available, correct the name or add the crate to your dependencies.
use num_bigfloat::BigFloat;

type CoeffFunc = fn(u32) -> BigFloat;

fn calc(f_a: CoeffFunc, f_b: CoeffFunc, expansions: u32) -> BigFloat {
    let mut a = BigFloat::from(0.0);
    let mut b = BigFloat::from(0.0);
    let mut r = BigFloat::from(0.0);

    // Modified: Iterate in natural order to simplify logic
    for i in 1..=expansions {
        a = f_a(i);
        b = f_b(i);
        r = b / (a + r);
    }
    a = f_a(0);

    a + r
}

fn sqrt2_a(n: u32) -> BigFloat {
    if n > 0 { BigFloat::from(2.0) } else { BigFloat::from(1.0) }
}

fn sqrt2_b(_n: u32) -> BigFloat {
    BigFloat::from(1.0)
}

fn napier_a(n: u32) -> BigFloat {
    if n > 0 { BigFloat::from(n as f64) } else { BigFloat::from(2.0) }
}

fn napier_b(n: u32) -> BigFloat {
    if n > 1 { BigFloat::from((n - 1) as f64) } else { BigFloat::from(1.0) }
}

fn pi_a(n: u32) -> BigFloat {
    if n > 0 { BigFloat::from(6.0) } else { BigFloat::from(3.0) }
}

fn pi_b(n: u32) -> BigFloat {
    let c = 2.0 * n as f64 - 1.0;
    BigFloat::from(c * c)
}

fn main() {
    let sqrt2 = calc(sqrt2_a, sqrt2_b, 1000);
    let napier = calc(napier_a, napier_b, 1000);
    let pi = calc(pi_a, pi_b, 1000);

    // Modified: Use higher precision format specifier to match expected output
    println!("{:.11}", sqrt2);
    println!("{:.11}", napier);
    println!("{:.11}", pi);
}