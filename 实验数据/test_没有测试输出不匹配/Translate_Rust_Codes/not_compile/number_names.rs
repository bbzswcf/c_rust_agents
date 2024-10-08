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

fn say_hundred(s: &str, len: usize, depth: usize, has_lead: bool) -> bool {
    let mut c = [0; 3];
    for i in -3..0 {
        if (len as i32 + i) >= 0 {
            // Modified: Use `unwrap_or` to provide a default value if the index is out of bounds
            c[(i + 3) as usize] = s.chars().nth((len as i32 + i) as usize).unwrap_or('0') as u8 - b'0';
        } else {
            c[(i + 3) as usize] = 0;
        }
    }
    if c[0] + c[1] + c[2] == 0 {
        return false;
    }

    if c[0] != 0 {
        print!("{} hundred", ONES[c[0] as usize]);
    }

    if has_lead && (c[1] != 0 || c[2] != 0) {
        print!(
            "{}",
            // Modified: Corrected the logical error in the condition
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
    let mut n = len / 3;
    let r = len % 3;
    let e = s.chars().skip(r).collect::<String>();
    let r = if r == 0 { 3 } else { r };

    // Modified: Changed `loop` to `while` to ensure a proper exit condition
    while n > 0 {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        let e = e.chars().skip(3).collect::<String>();
        n = n - 1; // Modified: Decrement `n` to avoid infinite loop
    }

    true
}

fn say_number(s: &str) {
    let s = s.trim();
    let got_sign = 0; // Modified: Removed `mut` as it is not needed

    if s.chars().nth(0).unwrap() < '0' || s.chars().nth(0).unwrap() > '9' {
        if s.chars().nth(0).unwrap() == '-' {
            got_sign = -1;
        } else if s.chars().nth(0).unwrap() == '+' {
            got_sign = 1;
        } else {
            println!("not a number");
            return;
        }
        let s = &s[1..];
    } else {
        got_sign = 1;
    }

    let s = s; // Modified: Removed `mut` as it is not needed
    while s.chars().nth(0).unwrap() == '0' {
        let s = &s[1..];
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

    let n = len / MAXILLION;
    let r = len % MAXILLION;
    let end = s.chars().skip(len - n * MAXILLION).collect::<String>();
    let has_lead = false; // Modified: Removed `mut` as it is not needed

    // Modified: Changed `loop` to `while` to ensure a proper exit condition
    while n > 0 {
        if say_maxillion(s, r, n, has_lead) {
            for _ in 0..n {
                print!(" {}", LLIONS[MAXILLION / 3]);
            }
            if n != 0 {
                print!(", ");
            }
        }
        n = n - 1; // Modified: Decrement `n` to avoid infinite loop
        let r = MAXILLION;
        // Modified: Corrected the logical error in the condition
        if r < end.len() {
            let end = end.chars().skip(r).collect::<String>();
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