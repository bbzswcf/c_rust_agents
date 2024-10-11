use std::f64::consts::PI;
use std::f64::EPSILON; // Added to handle floating-point precision issues

fn rk4(f: fn(f64, f64) -> f64, dx: f64, x: f64, y: f64) -> f64 {
    let k1 = dx * f(x, y);
    let k2 = dx * f(x + dx / 2.0, y + k1 / 2.0);
    let k3 = dx * f(x + dx / 2.0, y + k2 / 2.0);
    let k4 = dx * f(x + dx, y + k3);
    y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn rate(x: f64, y: f64) -> f64 {
    x * y.sqrt() // Ensure the rate function implementation is consistent with the C code
}

fn main() {
    let x0 = 0.0;
    let x1 = 10.0;
    let dx = 0.1;
    let n = 1 + ((x1 - x0) / dx) as usize;
    let mut y = Vec::with_capacity(n);

    y.push(1.0); // Initial condition set exactly as in the C code
    for i in 1..n {
        let y_prev = y[i - 1];
        let x_prev = x0 + dx * (i - 1) as f64; // Ensure iteration step size is handled with the same precision
        y.push(rk4(rate, dx, x_prev, y_prev));
    }

    println!("x\ty\trel. err.\n------------");
    for i in (0..n).step_by(10) {
        let x = x0 + dx * i as f64;
        let y2 = (x * x) / 4.0 + 1.0; // Ensure the formula for y2 matches the C code
        let rel_err = (y[i] - y2).abs() / (y2.abs() + f64::EPSILON); // Adjust relative error calculation to match precision of C code
        println!("{:.6}\t{:.6}\t{:.6}", x, y[i], rel_err); // Adjust output format to match C code precision
    }
}