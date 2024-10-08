use std::f64::consts::SQRT_2;

fn rk4(f: fn(f64, f64) -> f64, dx: f64, x: f64, y: f64) -> f64 {
    let k1 = dx * f(x, y);
    let k2 = dx * f(x + dx / 2.0, y + k1 / 2.0);
    let k3 = dx * f(x + dx / 2.0, y + k2 / 2.0);
    let k4 = dx * f(x + dx, y + k3);
    y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn rate(x: f64, y: f64) -> f64 {
    x * y.sqrt()
}

fn main() {
    let x0 = 0.0;
    let x1 = 10.0;
    let dx = 1.0; // Step size should be 1.0 to match the C code
    let n = 1 + ((x1 - x0) / dx) as usize;
    let mut y = vec![0.0; n];

    y[0] = 1.0; // Initial condition is correct
    for i in 1..n { // Loop should start from 1 and go up to n-1
        y[i] = rk4(rate, dx, x0 + dx * (i as f64), y[i - 1]); // Corrected the x value calculation
    }

    println!("x\ty\trel. err.\n------------");
    for i in 0..=10 { // Loop should iterate over integer values of x from 0 to 10
        let x = x0 + dx * i as f64;
        let y2 = (x * x / 4.0 + 1.0).powf(2.0); // Correct formula for y2
        let epsilon = 1e-10; // Small epsilon value to compare against zero
        let rel_err = if y2.abs() > epsilon { (y[i] - y2) / y2 } else { 0.0 }; // Correct relative error calculation
        println!("{:.6}\t{:.6}\t{:.6}", x, y[i], rel_err); // Correct output format
    }
}