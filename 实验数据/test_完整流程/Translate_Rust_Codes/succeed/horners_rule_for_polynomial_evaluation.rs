fn horner(coeffs: &[f64], x: f64) -> f64 {
    let mut res = 0.0;
    for &coeff in coeffs.iter().rev() {
        res = res * x + coeff;
    }
    res
}

fn main() {
    let coeffs = [-19.0, 7.0, -4.0, 6.0];
    println!("{:5.1}", horner(&coeffs, 3.0));
}