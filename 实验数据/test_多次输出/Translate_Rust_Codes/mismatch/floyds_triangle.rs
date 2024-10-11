fn t(n: i32) {
    let i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;

    while c < i {
        c *= 10;
        len += 1;
    }
    c -= i; // c is the col where width changes

    #[cfg(feature = "speed_matters")]
    {
        let mut tmp = vec![' '; 32];
        let mut s = vec![' '; 4096];
        let mut p = 0;

        format!("{:>width$}", 0, width = len).chars().enumerate().for_each(|(i, ch)| tmp[i] = ch);

        fn inc_numstr(tmp: &mut Vec<char>, len: usize) {
            let mut k = len;
            loop {
                if k == 0 {
                    return;
                }
                k -= 1;

                if tmp[k] == '9' {
                    tmp[k] = '0';
                    continue;
                }

                tmp[k] = if tmp[k] == '!' {
                    '1'
                } else {
                    (tmp[k] as u8 + 1) as char
                };
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                let offset = if j >= c { 1 } else { 0 };
                let num_len = len - if j < c { 1 } else { 0 };
                s[p..p + num_len].copy_from_slice(&tmp[offset..offset + num_len]);
                p += num_len;

                s[p] = if i - j != 0 { ' ' } else { '\n' };
                p += 1;

                if p + len >= 4096 {
                    print!("{}", String::from_utf8_lossy(&s[..p]));
                    p = 0;
                }
            }
        }

        if p > 0 {
            print!("{}", String::from_utf8_lossy(&s[..p]));
        }
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        let mut num = i;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:>width$}{}{}", num, if j < c { ' ' } else { ' ' }, if i - j != 0 { ' ' } else { '\n' }, width = len - if j < c { 1 } else { 0 });
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}