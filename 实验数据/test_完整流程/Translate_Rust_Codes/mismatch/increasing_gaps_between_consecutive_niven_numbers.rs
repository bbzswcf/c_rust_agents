// Ensure the `num_format` crate is added to your `Cargo.toml` file under `[dependencies]`.
// If using Rust 2015, add `extern crate num_format;` at the top of your file.
use num_format::{Locale, ToFormattedString};

fn main() {
    let previous: u64 = 1234;
    let gap: u64 = 678998;
    let mut sum: u64 = 0;
    let mut niven_index: i32 = 56778;
    let mut gap_index: i32 = 1000;

    // Ensure `gap_index` and `niven_index` are non-negative before converting to `u64`.
    // This conversion is safe since both indices are expected to be non-negative.
    let gap_index_formatted = gap_index as u64;
    let niven_index_formatted = niven_index as u64;

    println!("Gap index  Gap    Niven index    Niven number");
    println!(
        "{:>9} {:>4} {:>14} {:>15}",
        gap_index_formatted.to_formatted_string(&Locale::en),
        gap.to_formatted_string(&Locale::en),
        niven_index_formatted.to_formatted_string(&Locale::en),
        previous.to_formatted_string(&Locale::en)
    );
}