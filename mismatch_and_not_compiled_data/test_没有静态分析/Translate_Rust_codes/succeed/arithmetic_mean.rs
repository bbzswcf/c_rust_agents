fn mean(v: &[f64]) -> f64 {
    // Modified: Check if the vector is empty to correctly handle empty vectors
    if v.is_empty() {
        return f64::NAN; // Return NaN if the vector is empty
    }
    let sum: f64 = v.iter().sum();
    // Ensure the division by v.len() is correctly cast to f64 to avoid integer division
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
        // Modified: Ensure the slicing is correctly handled to avoid out-of-bounds errors
        let mean_val = mean(&v[..len.min(v.len())]);
        // Modified: Ensure the output format correctly handles the case where the mean is NaN
        if mean_val.is_nan() {
            print!("] = -nan\n");
        } else {
            print!("] = {}\n", mean_val);
        }
    }
}