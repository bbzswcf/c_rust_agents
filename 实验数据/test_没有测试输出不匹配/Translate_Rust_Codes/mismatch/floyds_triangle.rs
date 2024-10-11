use std::io::{self, Write};

fn t(n: i32) {
    let mut i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i; // c is the col where width changes

    #[cfg(feature = "speed_matters")]
    {
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
                inc_numstr(&mut tmp, len);
                let src = tmp.as_bytes();
                let src_len = len - (j < c) as usize;
                unsafe {
                    std::ptr::copy(src.as_ptr().add(1 - (j >= c) as usize), s.as_mut_ptr().add(p), src_len);
                }
                p += src_len;

                s[p] = if i - j != 0 { b' ' } else { b'\n' };
                p += 1;

                if p + len >= 4096 {
                    io::stdout().write_all(&s[..p]).unwrap();
                    p = 0;
                }
            }
        }

        io::stdout().write_all(&s[..p]).unwrap();
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        let mut num = i;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:0width$}{}{}", num, if i - j != 0 { ' ' } else { '\n' }, width = len - (j < c) as usize);
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}