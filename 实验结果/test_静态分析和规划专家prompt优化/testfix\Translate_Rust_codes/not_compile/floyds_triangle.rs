use std::io::{self, Write};

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
        let mut p = 0;

        fn inc_numstr(tmp: &mut String, len: usize) {
            let mut k = len;
            loop {
                if k == 0 {
                    return;
                }
                k -= 1;
                if tmp.chars().nth(k).unwrap() == '9' {
                    tmp.replace_range(k..k+1, "0");
                    continue;
                }
                let mut chars: Vec<char> = tmp.chars().collect();
                chars[k] = (chars[k] as u8 + 1) as char;
                tmp = chars.into_iter().collect();
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                let start = if j >= c { 1 } else { 0 };
                let end = if j < c { len - 1 } else { len };
                s.push_str(&tmp[start..end]);
                p += end - start;
                if i - j != 0 {
                    s.push(' ');
                } else {
                    s.push('\n');
                }
                p += 1;
                if p + len >= 4096 {
                    io::stdout().write_all(s.as_bytes()).unwrap();
                    s.clear();
                    p = 0;
                }
            }
        }
        io::stdout().write_all(s.as_bytes()).unwrap();
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                print!("{:0width$}", num, width = len - if j < c { 1 } else { 0 });
                if i - j != 0 {
                    print!(" ");
                } else {
                    println!();
                }
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}