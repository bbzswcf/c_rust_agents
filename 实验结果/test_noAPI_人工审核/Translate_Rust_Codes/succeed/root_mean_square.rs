fn rms(v: &[f64]) -> f64 {
    // Modified: Corrected the RMS calculation by including the division by the number of elements before taking the square root
    let sum: f64 = v.iter().map(|&x| x * x).sum();
    let mean_square = sum / v.len() as f64;
    mean_square.sqrt()
}

fn main() {
    let v = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    // Modified: Increased the precision in the format string to ensure six decimal places are displayed
    println!("{:.6}", rms(&v));
}