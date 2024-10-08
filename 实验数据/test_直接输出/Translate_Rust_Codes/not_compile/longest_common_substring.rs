use std::io::{self, Write};

fn lcs(sa: &str, sb: &str, beg: &mut &str, end: &mut &str) {
    let mut len: isize = 0;

    *beg = "";
    *end = "";

    for (apos, &achar) in sa.char_indices() {
        for (bpos, &bchar) in sb.char_indices() {
            if achar == bchar {
                len = 1;
                while apos + len as usize < sa.len() && bpos + len as usize < sb.len() && sa.as_bytes()[apos + len as usize] == sb.as_bytes()[bpos + len as usize] {
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

    for it in beg.chars() {
        io::stdout().write_all(&[it as u8])?;
    }
    println!();

    Ok(())
}