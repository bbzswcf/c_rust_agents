fn lcs(sa: &str, sb: &str) -> (usize, usize) {
    let mut apos: usize;
    let mut bpos: usize;
    let mut len: usize;
    let mut max_len = 0;
    let mut start = 0;

    for apos in 0..sa.len() {
        for bpos in 0..sb.len() {
            if sa.chars().nth(apos) == sb.chars().nth(bpos) {
                len = 0;
                while apos + len < sa.len() && bpos + len < sb.len() && sa.chars().nth(apos + len) == sb.chars().nth(bpos + len) {
                    len += 1;
                }

                if len > max_len {
                    max_len = len;
                    start = apos;
                }
            }
        }
    }

    (start, start + max_len)
}

fn main() {
    let s1 = "thisisatest";
    let s2 = "testing123testing";
    let (beg, end) = lcs(s1, s2);

    for ch in s1[beg..end].chars() {
        print!("{}", ch);
    }
    println!();
}