fn rms(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().map(|&x| x * x).sum();
    let result = f64::sqrt(sum / v.len() as f64);
    // Rounding the result to 6 decimal places to match the C output more closely
    let rounded_result = (result * 1000000.0).round() / 1000000.0;
    rounded_result
}

fn main() {
    let v = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    println!("{}", rms(&v));
}