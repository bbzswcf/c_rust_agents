fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    let diff = (value - other).abs();
    let max_value = value.abs().max(other.abs());
    diff <= epsilon || diff <= epsilon * max_value
}

fn test(a: f64, b: f64) {
    let epsilon = 1e-15;
    let max_value = a.abs().max(b.abs()).max(1.0);
    let epsilon = epsilon.min(max_value * 1e-15);
    println!("{:.6}, {:.6} => {}", a, b, approx_equals(a, b, epsilon) as i32);
}

fn main() {
    test(100000000000000.01, 100000000000000.011);
    test(100.01, 100.011);
    test(10000000000000.001 / 10000.0, 1000000000.0000001);
    test(0.001, 0.0010000001);
    test(0.000000000000000000000101, 0.0);
    test(f64::sqrt(2.0) * f64::sqrt(2.0), 2.0);
    test(-f64::sqrt(2.0) * f64::sqrt(2.0), -2.0);
    test(3.14159265358979323846, 3.14159265358979324);
}