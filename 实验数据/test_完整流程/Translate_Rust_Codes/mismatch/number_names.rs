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
            if let Some(ch) = s.chars().nth(len - 1 - i) {
                c[2 - i] = ch as u8 - b'0';
            } else {
                c[2 - i] = 0;
            }
        } else {
            c[2 - i] = 0;
        }
    }
    if c[0] + c[1] + c[2] == 0 {
        return false;
    }

    if c[0] != 0 {
        print!("{} hundred", ONES[c[0] as usize]);
        has_lead = true;
    }

    // Modified: Ensure "and" is printed correctly
    if has_lead && (c[1] != 0 || c[2] != 0) {
        print!("{}", if depth == 0 && (c[1] != 0 || c[2] != 0) { " and " } else { " " });
    }

    // Modified: Handle tens and ones places correctly
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

fn say_maxillion(mut s: &str, len: usize, depth: usize, mut has_lead: bool) -> bool {
    let mut n = len / 3;
    let r = len % 3;
    let e = if r == 0 { &s[3..] } else { &s[r..] };
    n = if r == 0 { n - 1 } else { n };
    let r = if r == 0 { 3 } else { r };

    loop {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            has_lead = true;
            print!(" {}", LLIONS[n]);
            if depth == 0 && n > 1 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        s = e;
        if n == 0 {
            break;
        }
        n = n - 1;
    }

    true
}

fn say_number(mut s: &str) {
    s = s.trim();

    // Modified: Handle negative numbers correctly
    if let Some(ch) = s.chars().nth(0) {
        if ch == '-' {
            print!("minus ");
            s = &s[1..];
        } else if ch == '+' {
            s = &s[1..];
        }
    }

    // Modified: Trim leading zeros correctly
    while let Some(ch) = s.chars().nth(0) {
        if ch == '0' {
            s = &s[1..];
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

    for c in s.chars() {
        if c < '0' || c > '9' {
            println!("(not a number)");
            return;
        }
    }

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let end = if len >= n * MAXILLION { &s[len - n * MAXILLION..] } else { s };

    loop {
        if say_maxillion(s, r, n, false) {
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        if n == 0 {
            break;
        }
        s = end;
        if len >= MAXILLION {
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