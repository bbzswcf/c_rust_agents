use std::ptr;

fn lcs(sa: &str, sb: &str, beg: &mut *const u8, end: &mut *const u8) {
    let sa_bytes = sa.as_bytes();
    let sb_bytes = sb.as_bytes();
    let mut apos: usize;
    let mut bpos: usize;
    let mut len: isize;

    *beg = ptr::null();
    *end = ptr::null();
    len = 0;

    for apos in 0..sa_bytes.len() {
        for bpos in 0..sb_bytes.len() {
            if sa_bytes[apos] == sb_bytes[bpos] {
                len = 1;
                // Modified: Ensure index calculations do not exceed bounds
                while apos + (len as usize) < sa_bytes.len() && bpos + (len as usize) < sb_bytes.len() && sa_bytes[apos + (len as usize)] == sb_bytes[bpos + (len as usize)] {
                    len += 1;
                }
            }

            if len > (*end as usize - *beg as usize) as isize {
                *beg = &sa_bytes[apos];
                // Modified: Ensure pointer arithmetic correctly points to the end of the subsequence
                *end = &sa_bytes[apos + len as usize - 1];
                len = 0;
            }
        }
    }
}

fn main() {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let mut beg: *const u8 = ptr::null();
    let mut end: *const u8 = ptr::null();

    lcs(s1, s2, &mut beg, &mut end);

    // Modified: Ensure `beg` is not null before dereferencing it
    if !beg.is_null() && !end.is_null() {
        unsafe {
            let mut it = beg;
            while it != end {
                print!("{}", *it as char);
                it = it.offset(1);
            }
        }
    }
    println!();
}