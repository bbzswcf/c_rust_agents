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
        let index = len as i32 - 3 + i as i32;
        if index >= 0 {
            if let Some(ch) = s.chars().nth(index as usize) {
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
        print!("{}", if depth == 0 || c[0] != 0 { "and " } else { " " });
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
    let e = &s[r..];
    if r == 0 {
        let e = &s[3..];
        let s = &s[..3];
    }
    // Modified: Added a termination condition to avoid infinite loop
    for i in 0..n {
        if say_hundred(&s[i * 3..(i + 1) * 3], 3, n, has_lead) && n != 0 {
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
    }
    true
}

fn say_number(s: &str) {
    let s = s.trim();

    if let Some(first_char) = s.chars().nth(0) {
        if first_char < '0' || first_char > '9' {
            if first_char == '-' {
                print!("minus ");
                let s = &s[1..];
            } else if first_char == '+' {
                let s = &s[1..];
            } else {
                println!("not a number");
                return;
            }
        }
    }

    // Modified: Correctly handle leading zeros
    let s = if s.starts_with('0') && s.len() > 1 {
        &s[1..]
    } else {
        s
    };

    if s.is_empty() {
        println!("zero");
        return;
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
    let end = &s[len - n * MAXILLION..];
    let has_lead = false;
    loop {
        if say_maxillion(s, r, n, has_lead) {
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
        let s = end;
        let end = &s[MAXILLION..];
        if end.is_empty() {
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