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
    let mut c = [0; 3];
    for i in 0..3 {
        // Modified: Ensure the index passed to `nth` is within bounds
        if (len - 3 + i) < s.len() {
            if let Some(ch) = s.chars().nth((len - 3 + i) as usize) {
                c[i] = ch as i32 - '0' as i32;
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
        // Modified: Replace the cast to `bool` with a comparison to zero
        print!("{}", if (depth != 0 || c[0] != 0) && (c[0] == 0 || c[1] == 0) { "and " } else { " " });
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
    let mut s = s; // Modified: Declare `s` as mutable
    let mut e = &s[r..];
    if r == 0 {
        e = &s[3..];
        s = &s[..3];
    }
    for _ in 0..=n {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        if s.is_empty() {
            break;
        }
        s = e;
        e = &e[3..];
    }
    true
}

fn say_number(s: &str) {
    let mut s = s.trim(); // Modified: Declare `s` as mutable
    let mut got_sign = 0;

    // Modified: Ensure the index passed to `nth` is within bounds
    if !s.is_empty() {
        if let Some(ch) = s.chars().nth(0) {
            if ch < '0' || ch > '9' {
                if ch == '-' {
                    got_sign = -1;
                } else if ch == '+' {
                    got_sign = 1;
                } else {
                    println!("not a number");
                    return;
                }
                s = &s[1..]; // Modified: Reassignment to mutable `s`
            } else {
                got_sign = 1;
            }
        }
    } else {
        println!("not a number");
        return;
    }

    while let Some(ch) = s.chars().nth(0) {
        if ch == '0' {
            s = &s[1..]; // Modified: Reassignment to mutable `s`
            if s.is_empty() {
                println!("zero");
                return;
            }
        } else {
            break;
        }
    }

    let len = s.len();
    if len == 0 {
        println!("not a number");
        return;
    }

    // Modified: Ensure the string is not empty before iterating over its characters
    if !s.is_empty() {
        for c in s.chars() {
            if c < '0' || c > '9' {
                println!("(not a number)");
                return;
            }
        }
    }

    if got_sign == -1 {
        print!("minus ");
    }

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let mut s = s; // Modified: Declare `s` as mutable
    let mut end = &s[len - n * MAXILLION..];
    let mut has_lead = false;
    for _ in 0..=n {
        if say_maxillion(s, r, n, has_lead) {
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        s = end; // Modified: Reassignment to mutable `s`
        end = &end[MAXILLION..];
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