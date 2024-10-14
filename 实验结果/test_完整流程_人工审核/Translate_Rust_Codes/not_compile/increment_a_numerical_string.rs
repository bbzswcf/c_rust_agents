fn incr(s: &mut String) -> &mut String {
    let len = s.len();
    let mut begin = 0;
    let mut neg = false;
    let mut tgt = '9';

    if s.starts_with('-') {
        neg = true;
        tgt = '0';
        begin = 1;
    } else if s.starts_with('+') {
        begin = 1;
    }

    // Special case: "-1"
    if s == "-1" {
        s.clear();
        s.push('0');
        return s;
    }

    // Find out how many digits need to be changed
    let mut tail = len.saturating_sub(1); // Use saturating_sub to prevent underflow

    // Ensure tail does not go out of bounds
    while tail >= begin {
        if let Some(ch) = s.chars().nth(tail) { // Safely handle optional character
            if ch == tgt {
                tail = tail.saturating_sub(1); // Use saturating_sub to prevent underflow
            } else {
                break;
            }
        } else {
            break;
        }
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        s.reserve(1);
        s.insert(0, '1');
        for i in begin..len {
            s.replace_range(i..i+1, "0");
        }
        s.push('0');
    } else if tail == begin && neg && s.len() > 1 {
        // Modified: Handle the Option directly without using unwrap_or
        if let Some(ch) = s.chars().nth(1) {
            if ch == '1' {
                // Special case: -1000..., so string will shrink
                for i in 1..len - begin {
                    s.replace_range(i..i+1, "9");
                }
                s.truncate(len - 1);
            }
        }
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            s.replace_range(i..i+1, if neg { "9" } else { "0" });
        }
        if let Some(digit) = s.chars().nth(tail) { // Safely handle optional character
            let new_digit = if neg {
                // Use checked subtraction to prevent overflow
                (digit as u8).checked_sub(1).map(|d| d as char).unwrap_or('9')
            } else {
                // Use checked addition to prevent overflow
                (digit as u8).checked_add(1).map(|d| d as char).unwrap_or('0')
            };
            s.replace_range(tail..tail+1, &new_digit.to_string());
        }
    }

    s
}

fn string_test(s: &str) {
    let mut ret = String::new(); // Removed unnecessary capacity allocation
    ret.push_str(s);

    println!("text: {}", ret);
    println!("  ->: {}", incr(&mut ret));
}

fn main() {
    string_test("+0");
    string_test("-1");
    string_test("-41");
    string_test("+41");
    string_test("999");
    string_test("+999");
    string_test("109999999999999999999999999999999999999999");
    string_test("-100000000000000000000000000000000000000000000");
}