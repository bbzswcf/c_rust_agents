use std::fmt::Write;

fn rprint(s: &mut String, x: &[i32]) -> usize {
    let mut i = 0;
    let mut j;

    while i < x.len() {
        j = i;
        while j < x.len() - 1 && x[j + 1] == x[j] + 1 { // Corrected the condition to avoid infinite loop.
            j += 1;
        }

        if i + 1 < j {
            write!(s, "{},{}-{}", if i > 0 { "," } else { "" }, x[i], x[j]).expect("Failed to write to string");
            i = j + 1; // Move `i` to the next segment
        } else {
            while i < j { // Ensure single elements are correctly handled.
                write!(s, "{},{}", if i > 0 { "," } else { "" }, x[i]).expect("Failed to write to string");
                i += 1;
            }
            write!(s, "{},{}", if i > 0 { "," } else { "" }, x[i]).expect("Failed to write to string");
            i += 1;
        }
    }
    s.len() // Return the length of the string
}

fn main() {
    let x = [
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let mut s = String::with_capacity(x.len() * 4); // Pre-calculate capacity for better performance.
    rprint(&mut s, &x);
    println!("{}", s);
}