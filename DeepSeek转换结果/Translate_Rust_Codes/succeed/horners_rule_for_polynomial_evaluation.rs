fn horner(coeffs: &[f64], s: i32, x: f64) -> f64 {
    let mut res = 0.0;

    for i in (0..s).rev() {
        res = res * x + coeffs[i as usize];
    }
    res
}

fn main() {
    let coeffs = [-19.0, 7.0, -4.0, 6.0];
    println!("{:5.1}", horner(&coeffs, coeffs.len() as i32, 3.0));
}