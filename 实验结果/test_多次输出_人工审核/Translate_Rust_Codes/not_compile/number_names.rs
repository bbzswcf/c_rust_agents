const ONES: [&str; 20] = [
    "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
    "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen",
];
const TENS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
const LLIONS: [&str; 5] = ["", "thousand", "million", "billion", "trillion"];
const MAXILLION: usize = LLIONS.len() * 3 - 3;

fn say_hundred(s: &[u8], depth: usize, has_lead: bool) -> bool {
    let mut c = [0; 3];
    for i in 0..3 {
        c[i] = if i < s.len() { s[s.len() - 1 - i] - b'0' } else { 0 };
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
            if (!depth || c[0] != 0) && (c[0] == 0 || c[1] == 0) {
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

fn say_maxillion(s: &[u8], depth: usize, has_lead: bool) -> bool {
    let len = s.len();
    let n = len / 3;
    let r = len % 3;
    let mut s = s;
    let mut e = &s[r..];

    if r == 0 {
        e = &s[3..];
        s = &s[..3];
    }

    loop {
        if say_hundred(s, n, has_lead) && n != 0 {
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
        if e.is_empty() {
            break;
        }
    }

    true
}

fn say_number(s: &str) {
    let s = s.trim();
    let mut got_sign = 0;

    if s.is_empty() {
        println!("not a number");
        return;
    }

    let mut s = s;
    if s.starts_with('-') {
        got_sign = -1;
        s = &s[1..];
    } else if s.starts_with('+') {
        got_sign = 1;
        s = &s[1..];
    } else {
        got_sign = 1;
    }

    s = s.trim_start_matches('0');
    if s.is_empty() {
        println!("zero");
        return;
    }

    let len = s.len();
    if len == 0 {
        println!("not a number");
        return;
    }

    for ch in s.chars() {
        if !ch.is_digit(10) {
            println!("(not a number)");
            return;
        }
    }

    if got_sign == -1 {
        print!("minus ");
    }

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let mut s = s.as_bytes();
    let mut end = &s[len - n * MAXILLION..];
    let mut has_lead = false;

    loop {
        if say_maxillion(s, n, has_lead) {
            has_lead = true;
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        n -= 1;
        s = end;
        end = &end[MAXILLION..];
        if n == 0 {
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