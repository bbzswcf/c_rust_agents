fn mean(v: &[f64]) -> f64 {
    // Modified: Return -f64::NAN for an empty slice to match the expected output
    if v.is_empty() {
        return -f64::NAN;
    }
    // Modified: Ensure the sum is of type f64 to avoid type mismatches
    let sum: f64 = v.iter().sum();
    sum / v.len() as f64
}

fn main() {
    // Modified: Use constants std::f64::consts::E and std::f64::consts::PI for accuracy
    let v = [1.0, 2.0, std::f64::consts::E, 3.0, std::f64::consts::PI];
    for len in (0..=5).rev() {
        print!("mean[");
        for (i, &val) in v.iter().enumerate().take(len) {
            if i > 0 {
                print!(", {}", val);
            } else {
                print!("{}", val);
            }
        }
        // Modified: Handle the special case of -nan separately
        let mean_value = mean(&v[..len]);
        if mean_value.is_nan() {
            println!("] = -nan");
        } else {
            println!("] = {}", format!("{:.15}", mean_value));
        }
    }
}