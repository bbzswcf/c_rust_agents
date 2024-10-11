fn map_range(a1: f64, a2: f64, b1: f64, b2: f64, s: f64) -> f64 {
    b1 + (s - a1) * (b2 - b1) / (a2 - a1)
}

fn main() {
    println!("Mapping [0,10] to [-1,0] at intervals of 1:");

    for i in 0..=10 {
        // Modified: Ensure the floating-point arithmetic is performed with sufficient precision (f64).
        // Adjust the formatting to display the expected number of decimal places (1 decimal place).
        // Verified the precision and rounding of the floating-point operations to ensure correct mapping.
        println!("f({}) = {:.1}", i, map_range(0.0, 10.0, -1.0, 0.0, i as f64));
    }
}