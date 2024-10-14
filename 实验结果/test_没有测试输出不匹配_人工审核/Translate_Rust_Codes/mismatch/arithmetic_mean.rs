fn mean(v: &[f64]) -> f64 {
    // Added check to avoid division by zero
    if v.len() > 0 {
        let sum: f64 = v.iter().sum();
        sum / v.len() as f64
    } else {
        0.0 // Default value for empty slice
    }
}

fn main() {
    // Modified: Used `std::f64::consts` module to access constants `E` and `PI`
    let v = [1.0, 2.0, std::f64::consts::E, 3.0, std::f64::consts::PI];
    for len in (0..=5).rev() {
        print!("mean[");
        // Ensured `len` is within valid range to avoid out-of-bounds access
        for (i, &val) in v.iter().enumerate().take(len.min(v.len())) {
            if i > 0 {
                print!(", {}", val);
            } else {
                print!("{}", val);
            }
        }
        // Added semicolon to properly terminate the `println!` macro
        println!("] = {}", mean(&v[..len]));
    }
}