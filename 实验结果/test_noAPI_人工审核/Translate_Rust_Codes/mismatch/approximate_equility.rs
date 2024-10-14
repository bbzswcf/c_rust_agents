fn approx_equals(value: f64, other: f64, epsilon: f64) -> bool {
    // Modified: Use a small epsilon to check if values are close enough to zero
    if value.abs() < epsilon && other.abs() < epsilon {
        return true;
    }
    // Modified: Use a relative epsilon approach for floating-point comparisons
    let diff = (value - other).abs();
    let largest = value.abs().max(other.abs());
    diff <= epsilon * largest
}

fn test(a: f64, b: f64) {
    // Modified: Use a dynamic epsilon based on the magnitude of the numbers
    let largest = a.abs().max(b.abs());
    let epsilon = largest * 1e-15;
    println!("{}, {} => {}", a, b, approx_equals(a, b, epsilon));
}

fn main() {
    test(100000000000000.01, 100000000000000.011);
    test(100.01, 100.011);
    test(1000000000.0000001, 1000000000.0000001);
    test(0.001, 0.0010000001);
    test(0.000000000000000000000101, 0.0);
    test((2.0_f64).sqrt() * (2.0_f64).sqrt(), 2.0);
    test(-2.0, -2.0);
    test(3.14159265358979323846, 3.14159265358979324);
    test(1e-15, 1e-15 + 1e-20);
    test(1e15, 1e15 + 1e-10);
    test(1e-15, 1e-15 + 1e-20);
}