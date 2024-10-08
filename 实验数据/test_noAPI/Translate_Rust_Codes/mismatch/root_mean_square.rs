fn rms(v: &[f64]) -> f64 {
    // Modified: Ensure the vector is not empty to avoid division by zero
    if v.is_empty() {
        return 0.0; // Handle the empty case appropriately
    }

    let sum: f64 = v.iter().map(|&x| x * x).sum();
    // Modified: Ensure the division is performed with sufficient precision
    let mean_square = sum / v.len() as f64;
    mean_square.sqrt()
}

fn main() {
    let v = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    // Modified: Use a more precise comparison method by allowing for a small margin of error (1e-6)
    let result = rms(&v);
    let expected = 6.2048368229954285;
    if (result - expected).abs() < 1e-6 {
        println!("{}", result);
    } else {
        println!("Output mismatch: expected {}, got {}", expected, result);
    }
}