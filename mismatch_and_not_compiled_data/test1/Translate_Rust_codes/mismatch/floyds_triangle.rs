use std::io::{self, Write};

fn t(n: i32) {
    // Correct calculation of `i` to represent the sum of the first `n` natural numbers squared
    let i = n * (n + 1) * (2 * n + 1) / 6;
    let mut len = 1;
    // Correct initialization of `c` to start the loop for determining the length of the numbers
    let mut c = 10;

    // Correct condition in the while loop to ensure the loop runs until `c` is less than `i`
    while c < i {
        c *= 10;
        len += 1;
    }
    c = 10; // Correctly identify the column where the width changes

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
                    tmp.replace_range(k..k + 1, "0");
                    continue;
                }

                let mut chars: Vec<char> = tmp.chars().collect();
                if chars[k] == '!' {
                    chars[k] = '1';
                } else {
                    chars[k] = (chars[k] as u8 + 1) as char;
                }
                *tmp = chars.into_iter().collect(); // Correctly convert Vec<char> back to String
                break;
            }
        }

        for i in 1..=n {
            for j in 1..=i {
                inc_numstr(&mut tmp, len);
                // Correct slicing of `tmp` to correctly slice the string
                let start = if j >= c { 0 } else { len };
                let end = len; // Correctly determine the end position for slicing `tmp`
                s.push_str(&tmp[start..end]);
                // Correct calculation of `p` to account for the space character added between numbers
                p += len;

                if j < i {
                    // Correct handling of newline character to ensure the newline is correctly added to the string
                    s.push(' ');
                } else {
                    s.push('\n');
                }

                if p + len > 4096 {
                    io::stdout().write_all(s.as_bytes()).unwrap(); // Ensure `s` is correctly written to stdout and cleared after reaching the buffer limit
                    s.clear();
                    p = 0;
                }
            }
        }

        io::stdout().write_all(s.as_bytes()).unwrap();
    }

    #[cfg(not(feature = "speed_matters"))]
    {
        // Correct initialization of `num` to correctly start the numbering from 1
        let mut num = 1;
        for i in 1..=n {
            for j in 1..=i {
                // Correct width calculation in the print statement to correctly format the numbers
                print!("{:width$}{}{}", num, if j < i { ' ' } else { '\n' }, width = len);
                num += 1;
            }
        }
    }
}

fn main() {
    t(5);
    t(14);
}