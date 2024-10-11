use std::io::{self, Write};

fn t(n: i32) {
    let i = n * (n - 1) / 2 + 1;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    c = 10 * (len - 1); // c is the col where width changes

    const SPEED_MATTERS: bool = true;

    if SPEED_MATTERS {
        let mut tmp = format!("{:0width$}", 0, width = usize::try_from(len).unwrap());
        let mut s = vec![0u8; 8192];
        let mut p = 0;

        fn inc_numstr(tmp: &mut String, len: usize) {
            let mut k = len;
            loop {
                if k == 0 {
                    return;
                }
                k -= 1;

                if tmp.as_bytes()[k] == b'9' {
                    tmp.as_bytes_mut()[k] = b'0';
                    continue;
                }

                if tmp.as_bytes_mut()[k] == b'!' {
                    tmp.as_bytes_mut()[k] = b'1';
                } else {
                    tmp.as_bytes_mut()[k] += 1;
                }
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len as usize);
                let src = tmp.as_bytes();
                let src_len = len as usize - (j < c) as usize;
                unsafe {
                    std::ptr::copy_nonoverlapping(src.as_ptr().add(1 - (j >= c) as usize), s.as_mut_ptr().add(p), src_len as usize);
                }
                p += src_len as usize;

                s[p] = if j != i { b' ' } else { b'\n' };
                p += 1;

                if p + usize::try_from(len).unwrap() >= 8192 {
                    io::stdout().write_all(&s[..p]).unwrap();
                    p = 0;
                }
            }
        }

        if p <= s.len() {
            io::stdout().write_all(&s[..p]).unwrap();
        }
    } else {
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:0width$}", num, width = usize::try_from(len).unwrap());
                if j != i {
                    print!(" ");
                } else {
                    print!("\n");
                }
                num += 1;
            }
        }
    }
}

fn main() {
    t(15);
}