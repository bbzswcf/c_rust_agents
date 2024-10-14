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
        if len > i {
            c[i] = s.chars().nth(len - i - 1).unwrap() as i32 - '0' as i32;
        } else {
            c[i] = 0;
        }
    }
    // Handle zero values correctly
    if c[0] == 0 && c[1] == 0 && c[2] == 0 {
        return false;
    }

    if c[0] != 0 {
        print!("{} hundred", ONES[c[0] as usize]);
        has_lead = true;
    }

    if has_lead && (c[1] != 0 || c[2] != 0) {
        print!(
            "{}",
            if (depth == 0 || c[0] != 0) && (c[0] == 0 || c[1] == 0) {
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

fn say_maxillion(s: &str, len: usize, depth: usize, has_lead: bool) -> bool {
    let n = len / 3;
    let r = len % 3;
    let mut s = s;
    let mut has_lead = has_lead;

    if r == 0 {
        return false;
    }

    for _ in 0..=n {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            has_lead = true;
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        s = &s[r..];
    }

    true
}

fn say_number(s: &str) {
    let mut s = s.trim();

    // Handle negative numbers correctly
    if s.starts_with('-') {
        print!("minus ");
        s = &s[1..];
    } else if s.starts_with('+') {
        s = &s[1..];
    }

    // Handle zero values correctly
    if s == "0" || s.is_empty() {
        println!("zero");
        return;
    }

    // Remove leading zeros
    while s.starts_with('0') && s.len() > 1 {
        s = &s[1..];
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
    let mut has_lead = false;

    for _ in 0..=n {
        if say_maxillion(s, r, n, has_lead) {
            has_lead = true;
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        s = &s[len - n * MAXILLION..];
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