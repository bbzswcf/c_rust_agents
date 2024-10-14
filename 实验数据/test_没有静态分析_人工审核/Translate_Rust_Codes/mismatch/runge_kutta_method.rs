use std::f64::consts::SQRT_2;

fn rk4(f: fn(f64, f64) -> f64, dx: f64, x: f64, y: f64) -> (f64, f64) {
    // Ensure all constants are of type f64 to avoid precision loss
    let k1 = dx * f(x, y);
    let k2 = dx * f(x + dx / 2.0_f64, y + k1 / 2.0_f64);
    let k3 = dx * f(x + dx / 2.0_f64, y + k2 / 2.0_f64);
    let k4 = dx * f(x + dx, y + k3);
    (y + (k1 + 2.0_f64 * k2 + 2.0_f64 * k3 + k4) / 6.0_f64, k1)
}

fn rate(x: f64, y: f64) -> f64 {
    x * y.sqrt()
}

fn main() {
    let x0 = 0.0;
    let x1 = 10.0;
    let dx = 0.1;
    let n = 1 + ((x1 - x0) / dx) as usize;
    let mut y = vec![0.0; n + 1]; // Resize the vector `y` to have `n+1` elements

    // Set initial condition for y
    y[0] = 1.0; // Initial condition set to 1.0
    for i in 0..n {
        let (y_new, _) = rk4(rate, dx, x0 + dx * i as f64, y[i]);
        y[i + 1] = y_new; // No longer out of bounds due to resizing
    }

    println!("x\ty\trel. err.\n------------");
    for i in (0..n).step_by(10) {
        let x = x0 + dx * i as f64;
        // Correct calculation of y2
        let y2 = (x * x / 4.0 + 1.0).sqrt(); // Removed multiplication by SQRT_2
        // Use a robust relative error calculation method
        let epsilon = 1e-10;
        let rel_err = if y2.abs() > epsilon { (y[i] - y2).abs() / y2.abs() } else { (y[i] - y2).abs() / epsilon }; // Corrected relative error calculation
        println!("{}\t{}\t{}", x, y[i], rel_err);
    }
}