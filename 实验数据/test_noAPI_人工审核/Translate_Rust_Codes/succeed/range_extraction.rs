use std::ptr;

fn rprint(s: &mut [u8], x: &[i32], len: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut offset = 0;

    while i < len {
        j = i;
        // Ensure j + 1 does not exceed the length of the slice
        while j < len - 1 && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if i + 1 < j {
            // Simplify the format string to ensure correctness
            let formatted_str = format!("{}{}-{}", if offset > 0 { "," } else { "" }, x[i], x[j]);
            let format_len = formatted_str.len();
            // Ensure the buffer has enough space
            if offset + format_len > s.len() {
                return offset; // Modified: Return current offset instead of breaking the loop
            }
            // Copy the bytes from the formatted string to the destination buffer using safe Rust methods
            s[offset..offset + format_len].copy_from_slice(formatted_str.as_bytes());
            offset += format_len; // Modified: Update offset correctly
        } else {
            while i <= j { // Modified: Correct loop condition to avoid skipping the last element
                // Simplify the format string to ensure correctness
                let formatted_str = format!("{}{}{}", if offset > 0 { "," } else { "" }, x[i], if i < j { "," } else { "" });
                let format_len = formatted_str.len();
                // Ensure the buffer has enough space
                if offset + format_len > s.len() {
                    return offset; // Modified: Return current offset instead of breaking the loop
                }
                // Copy the bytes from the formatted string to the destination buffer using safe Rust methods
                s[offset..offset + format_len].copy_from_slice(formatted_str.as_bytes());
                offset += format_len; // Modified: Update offset correctly
                i += 1;
            }
        }
        i = j + 1;
    }

    offset
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = [
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let len = x.len();
    // Calculate the exact size needed based on the actual formatted strings produced by the `rprint` function
    let max_size = len * (10 + 2) + 1; // Modified: Correct calculation of max_size
    let mut s = vec![0u8; max_size];
    let offset = rprint(&mut s, &x, len);

    // Removed redundant check for buffer overflow since `rprint` already performs this check internally
    // Modified: Specify the target type for the error conversion explicitly
    let s_str = std::str::from_utf8(&s[..offset]).map_err(|e| <std::str::Utf8Error as std::convert::Into<Box<dyn std::error::Error>>>::into(e))?;
    println!("{}", s_str);

    Ok(())
}