type DerivF = fn(f64, f64) -> f64;
const FMT: &str = "{:7.3}"; // Modified: Added a placeholder for the value to be printed

fn ivp_euler(f: DerivF, mut y: f64, step: i32, end_t: i32) {
    let mut t = 0;

    print!(" Step {:2}:", step); // Ensure the step values are formatted correctly
    while t <= end_t {
        if t % 10 == 0 {
            // Modified: Use the `format!` macro directly with the `FMT` string
            print!("{}", format!("{}", FMT).replace("{}", &format!("{:7.3}", y)));
        }
        y += step as f64 * f(t as f64, y);
        t += step;
    }
    println!();
}

fn analytic() {
    print!("    Time: ");
    for t in (0..=100).step_by(10) {
        // Modified: Use the `format!` macro directly with the `FMT` string
        print!("{}", format!("{}", FMT).replace("{}", &format!("{:7.3}", t)));
    }
    println!();
    print!("Analytic: ");

    for t in (0..=100).step_by(10) {
        // Modified: Use the `format!` macro directly with the `FMT` string
        print!("{}", format!("{}", FMT).replace("{}", &format!("{:7.3}", 20.0 + 80.0 * ((-0.07 * t as f64).exp()))));
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