use std::io::{self, Write};

fn lcs(sa: &str, sb: &str, beg: &mut &str, end: &mut &str) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut len: isize = 0;

    *beg = "";
    *end = "";

    for (apos, &a_char) in sa_bytes.iter().enumerate() {
        for (bpos, &b_char) in sb_bytes.iter().enumerate() {
            if a_char == b_char {
                len = 1;
                while apos + len as usize < sa_bytes.len() && bpos + len as usize < sb_bytes.len() && sa_bytes[apos + len as usize] == sb_bytes[bpos + len as usize] {
                    len += 1;
                }
            }

            if len > (*end as *const str as usize - *beg as *const str as usize) as isize {
                *beg = &sa[apos..];
                *end = &sa[apos + len as usize..];
                len = 0;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg = "";
    let mut end = "";

    lcs(s1, s2, &mut beg, &mut end);

    for &c in beg.as_bytes() {
        io::stdout().write_all(&[c])?;
    }
    println!();

    Ok(())
}