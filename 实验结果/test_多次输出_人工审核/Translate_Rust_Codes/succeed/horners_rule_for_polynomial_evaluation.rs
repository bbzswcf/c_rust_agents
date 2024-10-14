fn horner(coeffs: &[f64], x: f64) -> f64 {
    coeffs.iter().rev().fold(0.0, |res, &coeff| res * x + coeff)
}

fn main() {
    let coeffs = [-19.0, 7.0, -4.0, 6.0];
    println!("{:5.1}", horner(&coeffs, 3.0));
}