fn mean(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().sum();
    sum / v.len() as f64
}

fn main() {
    let v = [1.0, 2.0, 2.718, 3.0, 3.142];
    for len in (0..=5).rev() {
        print!("mean[");
        for (i, &val) in v.iter().take(len).enumerate() {
            if i > 0 {
                print!(", {}", val);
            } else {
                print!("{}", val);
            }
        }
        println!("] = {}", mean(&v[..len]));
    }
}