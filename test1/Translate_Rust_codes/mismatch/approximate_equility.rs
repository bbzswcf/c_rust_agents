fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    // Modified: Use a more robust method for comparing floating-point numbers
    // This method checks the relative error and handles zero values correctly
    if value == 0.0 && other == 0.0 {
        return true;
    }
    let diff = (value - other).abs();
    let norm = value.abs().max(other.abs());
    diff <= epsilon * norm
}

fn test(a: f64, b: f64) {
    // Modified: Use a fixed epsilon value that is appropriate for the precision required
    let epsilon = 1e-9;
    println!("{}, {} => {}", a, b, approx_equals(a, b, epsilon));
}

fn main() {
    test(1_000_000_000.0, 1_000_000_000.0);
    test(100.01, 100.011);
    test(1000000000.0, 1000000000.0);
    test(0.001, 0.0010000001);
    test(0.0, 0.0);
    test(2.0, 2.0);
    test(-2.0, -2.0);
    test(std::f64::consts::PI, std::f64::consts::PI);
}