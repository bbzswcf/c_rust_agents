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
        let mut tmp = format!("{:0width$}", 0, width = len);
        let mut s = String::with_capacity(4096);
        let mut p = s.as_mut_str();

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
                let start = if j >= c { 1 } else { 0 };
                let end = len - if j < c { 1 } else { 0 };
                p.push_str(&tmp[start..end]);
                p.push(if i - j != 0 { ' ' } else { '\n' });

                if p.len() + len >= 4096 {
                    print!("{}", p);
                    p = s.as_mut_str();
                }
            }
        }

        print!("{}", p);
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        let mut num = i;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:0width$}{}{}", num, if i - j != 0 { ' ' } else { '\n' }, width = len - if j < c { 1 } else { 0 });
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}