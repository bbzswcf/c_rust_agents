fn mean(v: &[f64]) -> f64 {
    let sum: f64 = v.iter().sum();
    sum / v.len() as f64; // Added semicolon to properly terminate the statement
}

fn main() {
    // Using precise constants f64::consts::E and f64::consts::PI
    let v = [1.0, 2.0, f64::consts::E, 3.0, f64::consts::PI];
    let mut len = 5;

    while len > 0 { // Ensured loop condition correctly matches intended behavior
        print!("mean[");
        // Used iterator and enumerate to avoid manual indexing
        for (i, &item) in v.iter().enumerate().take(len) {
            if i > 0 {
                print!(", {}", item);
            } else {
                print!("{}", item);
            }
        }
        println!("] = {}", mean(&v[..len])); // Ensured the mean function is called with the correct slice
        len -= 1; // Ensured len is decremented within the loop body
    }
}