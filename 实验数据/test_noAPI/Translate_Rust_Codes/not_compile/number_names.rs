// use std::str::FromStr;

const ONES: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", 
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", 
    "seventeen", "eighteen", "nineteen"
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
];
const LLIONS: [&str; 5] = [
    "", "thousand", "million", "billion", "trillion"
];
const MAXILLION: usize = LLIONS.len() * 3 - 3;

fn say_hundred(s: &str, len: usize, depth: usize, mut has_lead: bool) -> bool {
    // Modified: Made has_lead mutable to allow reassignment
    let mut c = [0; 3];
    for i in 0..3 {
        // Modified: Ensure len + i does not exceed the bounds of s
        if len + i < s.len() {
            c[i] = s.chars().nth(len + i).map(|ch| ch as i32 - '0' as i32).unwrap_or(0);
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
        // Modified: Convert depth to boolean using depth != 0
        if (depth != 0 || c[0] != 0) && (c[0] == 0 || c[1] == 0) {
            print!("and ");
        } else if c[0] != 0 {
            print!(" ");
        }
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

fn say_maxillion(s: &str, len: usize, depth: usize, has_lead: bool) -> bool {
    let n = len / 3;
    let r = len % 3;
    let mut s = s;
    // Modified: Ensure r is within the bounds of s before slicing
    let mut e = if r < s.len() { &s[r..] } else { "" };

    if r == 0 {
        e = &s[3..];
        s = &s[3..];
    }

    let mut has_lead = has_lead;
    loop {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            has_lead = true;
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        s = e;
        e = &e[3..];
        // Modified: Ensure the loop has a proper exit condition
        if s.is_empty() {
            break;
        }
    }

    true
}

fn say_number(s: &str) {
    let s = s.trim();
    // Removed: Unused variable assignment
    // let mut got_sign = 0;

    // Modified: Use char::is_digit to check if the character is a digit
    if let Some(ch) = s.chars().next() {
        if !ch.is_digit(10) {
            if ch == '-' {
                print!("minus ");
            } else if ch == '+' {
                // Do nothing for positive sign
            } else {
                println!("not a number");
                return;
            }
        }
    } else {
        println!("not a number");
        return;
    }

    // Modified: Ensure s is not empty before accessing its first character
    let mut s = if !s.is_empty() { &s[1..] } else { "" };
    while !s.is_empty() && s.chars().next() == Some('0') {
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

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let mut s = s;
    // Modified: Ensure len is within valid bounds before accessing s
    if len > n * MAXILLION {
        // Modified: Ensure s.len() - n * MAXILLION does not result in an invalid slice
        let mut end = if s.len() >= n * MAXILLION { &s[s.len() - n * MAXILLION..] } else { "" };
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
            s = end;
            end = &end[MAXILLION..];
            if s.is_empty() {
                break;
            }
        }
    }

    println!();
}

fn main() {
    // Modified: Ensure the argument passed to `plus_one` is of type `i32`
    say_number("-42");
    say_number("1984");
    say_number("10000");
    say_number("1024");
    say_number("1001001001001");
    say_number("123456789012345678901234567890123456789012345678900000001");
}