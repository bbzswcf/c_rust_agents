use std::f64::consts::E;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 0.0;
    let x1 = 10.0;
    let dx = 0.1;
    let n = 1 + ((x1 - x0) / dx) as usize; // Corrected type casting
    let mut y = vec![0.0; n]; // Corrected vector initialization

    y[0] = 1.0; // Initial condition is correct as per the feedback
    for i in 1..n {
        y[i] = rk4(rate, dx, x0 + dx * (i as f64), y[i - 1]); // Corrected function call and step size
    }

    println!("x\ty\trel. err.");
    println!("------------");
    for i in (0..n).step_by(10) {
        let x = x0 + dx * i as f64;
        let y2 = (x * x / 4.0 + 1.0).powf(2.0);
        // Corrected relative error calculation to handle y2 == 0 and ensure positive error
        let rel_err = if y2 == 0.0 {
            (y[i] - y2).abs()
        } else {
            (y[i] - y2).abs() / y2.abs()
        };
        println!("{:.6}\t{:.6}\t{:.6}", x, y[i], rel_err);
    }

    Ok(())
}