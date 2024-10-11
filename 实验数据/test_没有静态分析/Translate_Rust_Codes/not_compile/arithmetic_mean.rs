fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    let sum: f64 = v.iter().sum();
    (sum / v.len() as f64 * 1000.0).round() / 1000.0 // Round to three decimal places
}

fn main() {
    let v = [1.0, 2.0, 2.718, 3.0, 3.142];
    let mut len = 5;

    while len >= 0 { // Change the condition to include len == 0
        print!("mean[");
        for i in 0..len {
            if i > 0 {
                print!(", {}", v[i]);
            } else {
                print!("{}", v[i]);
            }
        }
        let mean_val = mean(&v[..len]);
        let mean_str = if mean_val.is_nan() {
            "-nan".to_string()
        } else {
            format!("{:.3}", mean_val) // Ensure precision is consistent with C output
        };
        println!("] = {}", mean_str);
        len -= 1;
    }
}