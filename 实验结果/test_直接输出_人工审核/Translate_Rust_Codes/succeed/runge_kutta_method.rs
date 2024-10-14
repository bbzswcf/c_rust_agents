fn rk4(f: fn(f64, f64) -> f64, dx: f64, x: f64, y: f64) -> f64 {
    let k1 = dx * f(x, y);
    let k2 = dx * f(x + dx / 2.0, y + k1 / 2.0);
    let k3 = dx * f(x + dx / 2.0, y + k2 / 2.0);
    let k4 = dx * f(x + dx, y + k3);
    y + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn rate(x: f64, y: f64) -> f64 {
    x * f64::sqrt(y)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x0 = 0.0;
    let x1 = 10.0;
    let dx = 0.1;
    let n = 1 + ((x1 - x0) / dx) as usize;
    let mut y = vec![0.0; n];

    y[0] = 1.0;
    for i in 1..n {
        y[i] = rk4(rate, dx, x0 + dx * (i as f64 - 1.0), y[i - 1]);
    }

    println!("x\ty\trel. err.\n------------");
    for i in (0..n).step_by(10) {
        let x = x0 + dx * i as f64;
        let y2 = f64::powf(x * x / 4.0 + 1.0, 2.0);
        println!("{}\t{}\t{}", x, y[i], y[i] / y2 - 1.0);
    }

    Ok(())
}