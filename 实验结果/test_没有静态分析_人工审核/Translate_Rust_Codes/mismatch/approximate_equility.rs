fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    // Modified: Use a combination of absolute and relative epsilon comparisons
    let abs_diff = (value - other).abs();
    // Handle zero values separately to avoid division by zero
    if value == 0.0 || other == 0.0 {
        return abs_diff <= epsilon;
    }
    // Use absolute values consistently to handle negative numbers correctly
    abs_diff <= epsilon || abs_diff <= (value.abs().max(other.abs()) * epsilon)
}

fn test(a: f64, b: f64) {
    // Modified: Dynamically adjust epsilon based on the magnitude of the numbers
    let epsilon = if a.abs() > 1e12 || b.abs() > 1e12 {
        1e-10 * a.abs().max(b.abs()) / 1e12
    } else {
        1e-12
    };
    println!("{}, {} => {}", a, b, approx_equals(a, b, epsilon));
}

fn main() {
    test(100000000000000.01, 100000000000000.011);
    test(100.01, 100.011);
    test(10000000000000.001 / 10000.0, 1000000000.0000001000);
    test(0.001, 0.0010000001);
    test(0.000000000000000000000101, 0.0);
    test(f64::sqrt(2.0) * f64::sqrt(2.0), 2.0);
    test(-f64::sqrt(2.0) * f64::sqrt(2.0), -2.0);
    test(3.14159265358979323846, 3.14159265358979324);
}