fn rms(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().map(|&x| x * x).sum();
    (sum / v.len() as f64).sqrt()
}

fn main() {
    let v = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    println!("{:.6}", rms(&v));
}