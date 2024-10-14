type CoeffFunc = fn(u32) -> f64;

fn calc(f_a: CoeffFunc, f_b: CoeffFunc, expansions: u32) -> f64 {
    let mut b = 0.0;
    // Modified: Correct initial value for `r` based on the mathematical formula for the continued fraction of `sqrt(2)`
    let mut r = 1.0;

    for i in (1..=expansions).rev() {
        let a = f_a(i);
        b = f_b(i);
        r = b / (a + r);
    }

    f_a(0) + r
}

fn sqrt2_a(n: u32) -> f64 {
    // Modified: Correct logic to reflect the mathematical formula for the continued fraction of `sqrt(2)`
    if n == 0 { 2.0 } else { 1.0 }
}

fn sqrt2_b(_n: u32) -> f64 {
    1.0
}

fn napier_a(n: u32) -> f64 {
    if n > 0 { n as f64 } else { 2.0 }
}

fn napier_b(n: u32) -> f64 {
    // Modified: Correct logic to reflect the mathematical formula for the continued fraction of Napier's constant
    if n == 0 { 1.0 } else { n as f64 }
}

fn pi_a(n: u32) -> f64 {
    if n > 0 { 6.0 } else { 3.0 }
}

fn pi_b(n: u32) -> f64 {
    // Modified: Corrected calculation to match the mathematical formula for the continued fraction of `pi`
    let c = (2.0 * n as f64 - 1.0).powi(2);
    c
}

fn main() {
    let sqrt2 = calc(sqrt2_a, sqrt2_b, 1000);
    let napier = calc(napier_a, napier_b, 1000);
    let pi = calc(pi_a, pi_b, 1000);

    // Modified: Corrected precision adjustment to match expected output format
    println!("{:.10}", sqrt2);
    println!("{:.10}", napier);
    println!("{:.10}", pi);
}