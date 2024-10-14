use std::str::FromStr;

const ONES: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const LLIONS: [&str; 5] = ["", "thousand", "million", "billion", "trillion"];
const MAXILLION: usize = LLIONS.len() * 3 - 3;

fn say_hundred(s: &str, len: usize, depth: usize, mut has_lead: bool) -> bool {
    let mut c = [0; 3];
    for i in 0..3 {
        if len + i >= 3 {
            // Modified: Added check to ensure the index is within bounds
            if let Some(ch) = s.chars().nth(len + i - 3) {
                // Modified: Ensure `ch` is a valid digit before subtraction
                if ch >= '0' && ch <= '9' {
                    c[i] = ch as i32 - '0' as i32;
                } else {
                    c[i] = 0; // Handle non-digit character
                }
            } else {
                c[i] = 0;
            }
        } else {
            c[i] = 0;
        }
    }

    if c[0] + c[1] + c[2] == 0 {
        return false;
    }

    if c[0] != 0 {
        print!("{} hundred", ONES[c[0] as usize]);
        has_lead = true;
    }

    if has_lead && (c[1] != 0 || c[2] != 0) {
        print!(
            "{}",
            if (depth == 0 || c[0] != 0) && (c[0] == 0 || c[1] == 0) { // Modified: Changed !depth to depth == 0
                "and "
            } else if c[0] != 0 {
                " "
            } else {
                ""
            }
        );
    }

    if c[1] < 2 {
        if c[1] != 0 || c[2] != 0 {
            print!("{}", ONES[(c[1] * 10 + c[2]) as usize]);
        }
    } else {
        if c[1] != 0 {
            print!("{}", TENS[c[1] as usize]);
            if c[2] != 0 {
                print!("-");
            }
        }
        if c[2] != 0 {
            print!("{}", ONES[c[2] as usize]);
        }
    }

    true
}

fn say_maxillion(mut s: &str, len: usize, depth: usize, has_lead: bool) -> bool {
    let mut n = len / 3; // Modified: Declared n as mutable
    let r = len % 3;
    let mut e = &s[r..];

    if r == 0 {
        // Modified: Added check to ensure the slice is valid
        if s.len() >= 3 {
            e = &s[3..];
        }
    }

    loop {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        s = e;
        // Modified: Added check to ensure the slice is valid
        if s.len() >= 3 {
            e = &s[3..];
        } else {
            e = "";
        }
        if n == 0 { // Modified: Changed condition to n == 0 to accurately check if n is zero
            break;
        }
        n -= 1; // Modified: Reassigned n
    }

    true
}

fn say_number(s: &str) {
    let mut s = s.trim();
    let mut got_sign = 0;

    // Modified: Added check to ensure the string is not empty before unwrapping
    if !s.is_empty() && (if let Some(ch) = s.chars().nth(0) { ch < '0' || ch > '9' } else { false }) {
        if let Some(ch) = s.chars().nth(0) {
            if ch == '-' {
                got_sign = -1;
            } else if ch == '+' {
                got_sign = 1;
            } else {
                println!("not a number");
                return;
            }
        }
        s = &s[1..];
    } else {
        got_sign = 1;
    }

    while !s.is_empty() && if let Some(ch) = s.chars().nth(0) { ch == '0' } else { false } {
        s = &s[1..];
        if s.is_empty() {
            println!("zero");
            return;
        }
    }

    let len = s.len();
    if len == 0 {
        println!("not a number");
        return;
    }

    for c in s.chars() {
        if c < '0' || c > '9' {
            println!("(not a number)");
            return;
        }
    }

    if got_sign == -1 {
        print!("minus ");
    }

    let mut n = len / MAXILLION; // Modified: Declared n as mutable
    let r = len % MAXILLION;
    let mut end = &s[len - n * MAXILLION..];
    let mut has_lead = false;

    loop {
        if say_maxillion(s, r, n, has_lead) {
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        n -= 1; // Modified: Reassigned n
        s = end;
        // Modified: Added check to ensure the slice is valid
        if s.len() >= MAXILLION {
            end = &s[MAXILLION..];
        } else {
            end = "";
        }
        if n == 0 { // Modified: Changed condition to n == 0 to accurately check if n is zero
            break;
        }
    }

    println!();
}

fn main() {
    say_number("-42");
    say_number("1984");
    say_number("10000");
    say_number("1024");
    say_number("1001001001001");
    say_number("123456789012345678901234567890123456789012345678900000001");
}