fn mean(v: &[f64]) -> f64 {
    // Handle the case where the vector is empty by returning f64::NAN
    if v.is_empty() {
        return f64::NAN;
    }
    let sum: f64 = v.iter().sum(); // Ensure the precision of the mean calculation matches the C output
    sum / v.len() as f64 // Ensure the precision of the mean calculation matches the C output
}

fn main() {
    // Use fully qualified path for constants to avoid ambiguity
    let v = [1.0, 2.0, std::f64::consts::E, 3.0, std::f64::consts::PI];
    let mut len = 5;

    while len > 0 {
        print!("mean[");
        for i in 0..len {
            if i > 0 {
                print!(", {}", v[i]);
            } else {
                print!("{}", v[i]);
            }
        }
        // Create a longer-lived value for the result of `mean(&v[..len]).to_string()`
        let mean_str = mean(&v[..len]).to_string();
        // Ensure the output format for NaN is consistent
        println!("] = {}", if mean(&v[..len]).is_nan() { "-nan" } else { &mean_str }); // Ensure the output format for NaN is consistent
        len -= 1;
    }
}