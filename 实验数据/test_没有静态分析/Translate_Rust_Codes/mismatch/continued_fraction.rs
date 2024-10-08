type CoeffFunc = fn(u32) -> f64;

fn calc(f_a: CoeffFunc, f_b: CoeffFunc, expansions: u32) -> f64 {
    let mut a = 0.0;
    let mut b = 0.0;
    let mut r = 1.0; // Modified: Initialize r with a non-zero value to avoid division by zero

    for i in (1..=expansions).rev() {
        a = f_a(i);
        b = f_b(i);
        r = b / (a + r);
    }
    a = f_a(0);

    a + r
}

fn sqrt2_a(n: u32) -> f64 {
    if n > 0 { 2.0 } else { 1.0 }
}

fn sqrt2_b(_n: u32) -> f64 {
    1.0 // Simplified: Always return 1.0 without any checks
}

fn napier_a(n: u32) -> f64 {
    if n > 0 { n as f64 } else { 2.0 }
}

fn napier_b(n: u32) -> f64 {
    if n > 1 { (n - 1) as f64 } else { 1.0 }
}

fn pi_a(n: u32) -> f64 {
    if n > 0 { 6.0 } else { 3.0 }
}

fn pi_b(n: u32) -> f64 {
    let c = 2.0 * n as f64 - 1.0;
    c * c
}

fn main() {
    let sqrt2 = calc(sqrt2_a, sqrt2_b, 1000);
    let napier = calc(napier_a, napier_b, 1000);
    let pi = calc(pi_a, pi_b, 1000);

    // Modified: Ensure the number of decimal places printed is exactly 9 to match the C output
    println!("{:.9}", sqrt2);
    println!("{:.9}", napier);
    println!("{:.9}", pi);
}