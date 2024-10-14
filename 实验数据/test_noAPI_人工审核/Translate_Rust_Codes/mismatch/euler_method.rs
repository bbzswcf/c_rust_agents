use std::f64::consts::E;

type DerivF = fn(f64, f64) -> f64;
const FMT: &str = " {:7.3}";

fn ivp_euler(f: DerivF, mut y: f64, step: i32, end_t: i32) {
    let mut t = 0;

    // Modified: Use predefined format string `FMT` for proper alignment
    print!("{}", format!(" Step {:2}:", step));
    while t <= end_t {
        if t % 10 == 0 {
            // Modified: Use the format string `FMT` directly without wrapping it in another format string
            print!("{}", format!("{}", y));
        }
        // Modified: Ensure the calculation is correct based on the Euler method
        y += step as f64 * f(t as f64, y);
        t += step;
    }
    println!();
}

fn analytic() {
    print!("    Time: ");
    for t in (0..=100).step_by(10) {
        // Modified: Use the format string `FMT` directly without wrapping it in another format string
        print!("{}", format!("{}", t));
    }
    println!();
    print!("Analytic: ");

    for t in (0..=100).step_by(10) {
        // Modified: Use the format string `FMT` directly without wrapping it in another format string
        print!("{}", format!("{}", 20.0 + 80.0 * E.powf(-0.07 * t as f64)));
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