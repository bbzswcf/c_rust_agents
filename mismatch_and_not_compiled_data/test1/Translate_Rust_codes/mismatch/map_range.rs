fn map_range(a1: f64, a2: f64, b1: f64, b2: f64, s: f64) -> f64 {
    // Corrected: Use the correct formula for mapping ranges
    let result = b1 + (s - a1) * (b2 - b1) / (a2 - a1);
    // Removed: Rounding logic is incorrect for this use case
    result
}

fn main() {
    println!("Mapping [0,10] to [-1,0] at intervals of 1:");

    for i in 0..=10 {
        let mapped_value = map_range(0.0, 10.0, -1.0, 0.0, i as f64);
        // Corrected: Handle the special case where the result is -0.0
        let formatted_value = if mapped_value == -0.0 { 0.0 } else { mapped_value };
        // Corrected: Ensure the output format matches the expected precision
        println!("f({}) = {:.1}", i, formatted_value);
    }
}