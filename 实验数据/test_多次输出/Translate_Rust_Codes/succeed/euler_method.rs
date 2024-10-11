fn ivp_euler<F>(f: F, mut y: f64, step: i32, end_t: i32)
where
    F: Fn(f64, f64) -> f64,
{
    let mut t = 0;

    print!(" Step {:2}: ", step);
    while t <= end_t {
        if t % 10 == 0 {
            print!(" {:7.3}", y);
        }
        y += step as f64 * f(t as f64, y);
        t += step;
    }
    println!();
}

fn analytic() {
    print!("    Time: ");
    for t in (0..=100).step_by(10) {
        print!(" {:7}", t);
    }
    println!();

    print!("Analytic: ");
    for t in (0..=100).step_by(10) {
        print!(" {:7.3}", 20.0 + 80.0 * (-0.07 * t as f64).exp());
    }
    println!();
}

fn cooling(t: f64, temp: f64) -> f64 {
    -0.07 * (temp - 20.0)
}

fn main() {
    analytic();
    ivp_euler(cooling, 100.0, 2, 100);
    ivp_euler(cooling, 100.0, 5, 100);
    ivp_euler(cooling, 100.0, 10, 100);
}