use std::io::{self, Write};

fn lcs<'a>(sa: &'a str, sb: &str, beg: &mut &'a str, end: &mut &'a str) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut max_len = 0;
    let mut max_end = 0;

    for (apos, &a_char) in sa_bytes.iter().enumerate() {
        for (bpos, &b_char) in sb_bytes.iter().enumerate() {
            if a_char == b_char {
                let mut len = 0;
                while apos + len < sa_bytes.len() && bpos + len < sb_bytes.len() && sa_bytes[apos + len] == sb_bytes[bpos + len] {
                    len += 1;
                }
                if len > max_len {
                    max_len = len;
                    max_end = apos + len;
                }
            }
        }
    }

    *beg = &sa[max_end - max_len..max_end];
    *end = &sa[max_end..];
}

fn main() -> io::Result<()> {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg: &str = "";
    let mut end: &str = "";

    lcs(s1, s2, &mut beg, &mut end);

    let result = beg.to_string();
    io::stdout().write_all(result.as_bytes())?;
    println!();

    Ok(())
}