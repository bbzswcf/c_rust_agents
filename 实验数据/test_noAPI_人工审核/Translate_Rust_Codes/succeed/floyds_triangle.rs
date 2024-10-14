use std::io::{self, Write};

fn t(n: i32) {
    let mut i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    // Modified: Recalculate c to correctly identify the column where the width changes
    c = len;

    #[cfg(feature = "speed_matters")]
    {
        let mut tmp = vec![b' '; 32];
        let mut s = vec![0; 4096];
        let mut p = s.as_mut_ptr();

        unsafe {
            // Modified: Correct the format string to use an empty string instead of '0'
            let formatted = format!("{:>len$}", "", len = len as usize);
            tmp[..formatted.len()].copy_from_slice(formatted.as_bytes());

            fn inc_numstr(tmp: &mut [u8], len: usize) {
                let mut k = len;
                loop {
                    if k == 0 {
                        return;
                    }
                    k -= 1;

                    if tmp[k] == b'9' {
                        tmp[k] = b'0';
                        continue;
                    }

                    tmp[k] += 1;
                    // Modified: Correct the character check to handle carry-over correctly
                    if tmp[k] == b'9' {
                        tmp[k] = b'0';
                    }
                    break;
                }
            }

            for i in 1..=n {
                for j in 1..=i {
                    inc_numstr(&mut tmp, len);
                    // Modified: Correct the offset calculation based on the actual length of the number string
                    let offset = if j >= c { len } else { 0 };
                    // Modified: Correctly calculate copy_len based on the actual length of the number string
                    let copy_len = len;
                    std::ptr::copy_nonoverlapping(tmp.as_ptr().add(offset), p, copy_len);
                    p = p.add(copy_len);

                    *p = if i - j != 0 { b' ' } else { b'\n' };
                    p = p.add(1);

                    if p.offset_from(s.as_ptr()) as usize + len >= 4096 {
                        io::stdout().write_all(&s[..p.offset_from(s.as_ptr()) as usize]).unwrap();
                        p = s.as_mut_ptr();
                    }
                }
            }

            io::stdout().write_all(&s[..p.offset_from(s.as_ptr()) as usize]).unwrap();
        }
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                // Modified: Correct the character check in the print macro
                print!("{:>len$}", num, len = len as usize);
                print!("{}", if i - j != 0 { ' ' } else { '\n' });
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}