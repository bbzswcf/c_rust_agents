use std::io;
use std::io::Write;

// Correctly increments the number string by one
fn inc_numstr(tmp: &mut String, len: usize) {
    let mut k = len;
    loop {
        if k == 0 {
            // If we reach the beginning of the string, prepend a '1' to handle carry-over
            tmp.insert(0, '1');
            return;
        }
        k -= 1;

        if tmp.chars().nth(k).unwrap() == '9' {
            // Replace '9' with '0' and continue to handle carry-over
            tmp.replace_range(k..k+1, "0");
            continue;
        }

        // Increment the current digit and break the loop
        let mut chars: Vec<char> = tmp.chars().collect();
        chars[k] = char::from_digit(chars[k].to_digit(10).unwrap() + 1, 10).unwrap();
        tmp.replace_range(k..k+1, &chars[k].to_string());
        break;
    }
}

fn t(n: i32) {
    let i = n * (n - 1) / 2;
    let mut len = 1;
    let mut c = 1;
    while c <= i {
        c *= 10;
        len += 1;
    }
    c /= 10; // c is the column where width changes

    const SPEED_MATTERS: bool = false;

    if SPEED_MATTERS {
        let mut tmp = format!("{:0width$}", 0, width = len);
        let mut s = String::with_capacity(4096);
        let mut p = 0;

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                let start = if j >= c { 1 } else { 0 };
                let end = if j < c { len - 1 } else { len };
                s.push_str(&tmp[start..end]);
                p += len - (j < c) as usize;

                if i - j != 0 {
                    s.push(' ');
                } else {
                    s.push('\n');
                }

                const BUFFER_SIZE: usize = 4096;
                if p + len >= BUFFER_SIZE {
                    io::stdout().write_all(s.as_bytes()).unwrap();
                    s.clear();
                    p = 0;
                }
            }
        }

        io::stdout().write_all(s.as_bytes()).unwrap();
    } else {
        for i in 1..=n {
            for j in 1..=i {
                let num = (i * (i - 1) / 2) + j;
                print!("{:0width$}{}{}", num, if i - j != 0 { ' ' } else { '\n' }, width = len - (j < c) as usize);
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}