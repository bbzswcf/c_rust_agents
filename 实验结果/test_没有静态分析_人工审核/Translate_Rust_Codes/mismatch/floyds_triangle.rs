use std::io::{self, Write};

fn t(n: i32) {
    // Corrected the calculation of `i` to correctly calculate the sum of the first `n` natural numbers
    let i = n * (n + 1) / 2;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i; // c is the col where width changes

    const SPEED_MATTERS: bool = false;

    if SPEED_MATTERS {
        let mut tmp = format!("{:0width$}", 0, width = len);
        let mut s = vec![0u8; 4096];
        let mut p = 0;

        fn inc_numstr(tmp: &mut String, len: usize) {
            let mut k = len;
            loop {
                if k == 0 {
                    return;
                }
                k -= 1;

                if tmp.as_bytes()[k] == b'9' {
                    // Wrapped unsafe function call in an `unsafe` block
                    unsafe {
                        tmp.as_bytes_mut()[k] = b'0';
                    }
                    continue;
                }

                // Wrapped unsafe function call in an `unsafe` block
                unsafe {
                    tmp.as_bytes_mut()[k] += 1;
                }
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                let src = tmp.as_bytes();
                let src_len = len - (j < c) as usize;
                unsafe {
                    std::ptr::copy_nonoverlapping(src.as_ptr().add(1 - (j >= c) as usize), s.as_mut_ptr().add(p), src_len);
                }
                p += src_len;

                s[p] = if j < i { b' ' } else { b'\n' };
                p += 1;

                if p + len >= 4096 {
                    io::stdout().write_all(&s[..p]).unwrap();
                    p = 0;
                }
            }
        }

        io::stdout().write_all(&s[..p]).unwrap();
    } else {
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                // Corrected the condition to determine when to print a space or a newline
                print!("{:0width$}{}{width}", num, if j < i { ' ' } else { '\n' }, width = len - (j < c) as usize);
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}