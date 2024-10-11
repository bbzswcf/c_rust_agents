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

fn say_hundred(s: &str, len: usize, depth: usize, has_lead: &mut bool) -> bool {
    let mut c = [0; 3];
    for i in 0..3 {
        if len > i {
            // Ensure the index is within bounds before accessing `s.chars().nth(len - i - 1)`
            if let Some(ch) = s.chars().nth(len - i - 1) {
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
        *has_lead = true;
    }

    if *has_lead && (c[1] != 0 || c[2] != 0) {
        if (depth == 0 || c[0] != 0) && (c[0] == 0 || c[1] == 0) {
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

fn say_maxillion(s: &mut String, len: usize, depth: usize, has_lead: &mut bool) -> bool {
    let mut n = len / 3;
    let mut r = len % 3;
    let mut e = s.chars().skip(r).collect::<String>();

    if r == 0 {
        r = 3;
    }

    loop {
        if say_hundred(s, r, n, has_lead) && n != 0 {
            *has_lead = true;
            print!(" {}", LLIONS[n]);
            if depth == 0 {
                print!(", ");
            } else {
                print!(" ");
            }
        }
        if e.len() >= 3 {
            let drained_elements: String = e.drain(..3).collect(); // Temporarily store drained elements
            let temp_slice = &drained_elements[..3]; // Create a temporary slice to hold the part of `drained_elements`
            *s = temp_slice.to_string(); // Convert `temp_slice` to `String` and assign it to `s`
        } else {
            break;
        }
        if n == 0 {
            break;
        }
        n -= 1;
    }

    true
}

fn say_number(s: &str) {
    let mut s = s.trim();
    let mut got_sign = 0;

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
            s = &s[1..];
        } else {
            got_sign = 1;
        }
    }

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

    if got_sign == -1 {
        print!("minus ");
    }

    let mut n = len / MAXILLION;
    let mut r = len % MAXILLION;
    let mut end = s.chars().skip(len - n * MAXILLION).collect::<String>();
    let mut has_lead = false;

    loop {
        if say_maxillion(&mut s.to_string(), r, n, &mut has_lead) {
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
        n -= 1;
        r = MAXILLION;
        if end.len() >= r {
            let drained_elements: String = end.drain(..r).collect(); // Temporarily store drained elements
            let temp_slice = &drained_elements[..r]; // Create a temporary slice to hold the part of `drained_elements`
            s = temp_slice; // Use the temporary slice to update `s`
        } else {
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