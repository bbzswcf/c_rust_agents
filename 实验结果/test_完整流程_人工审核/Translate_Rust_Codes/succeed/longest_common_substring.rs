use std::io::{self, Write};

// Modified: Changed the type of `sb` to `&'a str` to indicate that the function data borrowed from either `sa` or `sb`
fn lcs<'a>(sa: &'a str, sb: &'a str, beg: &mut &'a str, end: &mut &'a str) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut len: usize = 0; // Modified: Explicitly typed `len` to avoid type inference issues

    *beg = "";
    *end = "";

    for (apos, &a_char) in sa_bytes.iter().enumerate() {
        for (bpos, &b_char) in sb_bytes.iter().enumerate() {
            if a_char == b_char {
                len = 0;
                while apos + len < sa_bytes.len() && bpos + len < sb_bytes.len() && sa_bytes[apos + len] == sb_bytes[bpos + len] {
                    len += 1;
                }
            }

            // Modified: Corrected the condition to properly find the longest common substring
            if len > (*end).len() {
                *beg = &sa[apos..apos + len];
                *end = &sa[apos..apos + len];
                // Modified: `len` should not be reset to 0 after updating `beg` and `end`
            }
        }
    }
}

fn main() {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg = "";
    let mut end = "";

    lcs(s1, s2, &mut beg, &mut end);

    // Modified: Correctly handle the output of the longest common substring
    for c in beg.chars() {
        io::stdout().write_all(&[c as u8]).expect("Failed to write to stdout");
    }
    println!();
}