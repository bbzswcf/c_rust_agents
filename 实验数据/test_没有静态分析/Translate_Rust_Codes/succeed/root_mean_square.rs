fn rms(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().map(|&x| x * x).sum();
    f64::sqrt(sum / v.len() as f64)
}

fn main() {
    let v = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    let result = rms(&v);
    // Modified: Adjust the number of decimal places to match the precision of the C output
    println!("{:.6}", result); // Ensure the output is rounded to 6 decimal places
}