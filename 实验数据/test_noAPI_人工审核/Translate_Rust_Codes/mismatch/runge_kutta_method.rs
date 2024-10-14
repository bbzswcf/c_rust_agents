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
    let dx = 0.01; // Modified: Reduced step size to improve accuracy
    let n = 1 + ((x1 - x0) / dx) as usize;
    let mut y = vec![0.0; n];

    // Modified: Correct initial condition for y at x = 0
    y[0] = 1.0; // Assuming the initial condition should be 1.0 based on the expected output

    // Modified: Start the loop from 1 to avoid negative indexing
    for i in 1..n {
        y[i] = rk4(rate, dx, x0 + dx * i as f64, y[i - 1]);
    }

    println!("x\ty\trel. err.");
    println!("------------");
    for i in (0..n).step_by(10) {
        let x = x0 + dx * i as f64;
        // Modified: Correct formula for y2
        let y2 = (x * x / 4.0 + 1.0).powf(2.0); // Assuming the correct formula based on the expected output
        // Modified: Correct relative error calculation
        let rel_err = if y2.abs() > 1e-10 { // Assuming the correct relative error calculation based on the expected output
            (y[i] - y2).abs() / y2.abs()
        } else {
            (y[i] - y2).abs()
        };
        // Modified: Correct output format
        println!("{:.6}\t{:.6}\t{:.6}", x, y[i], rel_err); // Assuming the correct output format based on the expected output
    }
}