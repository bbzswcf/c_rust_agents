fn mean(v: &[f64]) -> f64 {
    // Handle the case where the slice is empty to avoid division by zero
    if v.is_empty() {
        return f64::NAN; // Return NaN for an empty slice
    }
    let sum: f64 = v.iter().sum(); // Ensure the sum is of type f64
    sum / (v.len() as f64) // Adjusted mean calculation to ensure correct precision
}

fn main() {
    // Ensure that the constants E and PI used in the array v match the precision used in the C output
    let e = 2.718281828459045;
    let pi = 3.141592653589793;
    let v = [1.0, 2.0, e, 3.0, pi];
    let mut len = 5;

    while len > 0 { // Ensure the while loop correctly iterates over the slice
        print!("mean[");
        // Used iterator and enumerate() to avoid manual indexing
        for (i, &item) in v.iter().enumerate().take(len) {
            if i > 0 {
                print!(", {:.15}", item); // Ensure the output formatting matches the C output exactly
            } else {
                print!("{:.15}", item);
            }
        }
        let mean_value = mean(&v[..len]);
        if mean_value.is_nan() {
            println!("] = -nan"); // Ensure the output format for empty slice matches the C output exactly
        } else {
            println!("] = {:.3}", mean_value); // Adjusted the precision to match the C output exactly
        }
        len -= 1; // Ensured len is correctly decremented
    }
    // Handle the output for the empty slice case
    print!("mean[");
    println!("] = -nan"); // Corrected output for empty slice to match C output
}