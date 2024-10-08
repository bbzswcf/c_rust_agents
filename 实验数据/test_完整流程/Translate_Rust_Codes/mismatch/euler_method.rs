type DerivF = fn(f64, f64) -> f64;
const FMT: &str = " {:7.3}";

fn ivp_euler(f: DerivF, mut y: f64, step: i32, end_t: i32) {
    let mut t = 0;

    print!(" Step {:2}:", step);
    while t <= end_t {
        if t % 10 == 0 {
            print!("{}{}", FMT, y);
        }
        // Corrected calculation for Euler's method
        y += step as f64 * f(t as f64, y); // Ensure step size is correctly scaled
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
    // Corrected formatting string placement
    print!("Analytic:");

    for t in (0..=100).step_by(10) {
        // Corrected calculation for the analytic solution
        print!("{}{}", FMT, 20.0 + 80.0 * ((-0.07 * t as f64).exp())); // Ensure initial temperature offset is correct
    }
    println!();
}

// Prefixed unused variable `t` with an underscore
fn cooling(_t: f64, temp: f64) -> f64 {
    -0.07 * (temp - 20.0)
}

fn main() {
    analytic();
    // Corrected initial value for `y`
    ivp_euler(cooling, 100.0, 2, 100);
    ivp_euler(cooling, 100.0, 5, 100);
    ivp_euler(cooling, 100.0, 10, 100);
}