use std::io::{self, Write};

// Modified: Changed the type of `sb` to `&'a str` to match the lifetime of `sa`
fn lcs<'a>(sa: &'a str, sb: &'a str, beg: &mut &'a str, end: &mut &'a str) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut len;

    *beg = "";
    *end = "";
    len = 0;

    for apos in 0..sa_bytes.len() {
        for bpos in 0..sb_bytes.len() {
            if sa_bytes[apos] == sb_bytes[bpos] {
                len = 1;
                while apos + len < sa_bytes.len() && bpos + len < sb_bytes.len() && sa_bytes[apos + len] == sb_bytes[bpos + len] {
                    len += 1;
                }
            }

            if len > (*end).len() - (*beg).len() {
                *beg = &sa[apos..apos + len];
                *end = &sa[apos..apos + len];
                len = 0;
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

    for c in beg.chars() {
        io::stdout().write_all(&[c as u8]).expect("Failed to write to stdout");
    }
    println!();
}