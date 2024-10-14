fn rms(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().map(|&x| x * x).sum();
    (sum / v.len() as f64).sqrt()
}

fn main() {
    let v = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    println!("{}", rms(&v));
}