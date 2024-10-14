// Removed: Unused import
// use std::cmp::Ordering;

fn incr(s: &mut String) -> &mut String {
    let neg = s.starts_with('-');
    let tgt = if neg { '0' } else { '9' };

    // Special case: "-1"
    if s == "-1" {
        s.clear();
        s.push('0');
        return s;
    }

    let len = s.len();
    let begin = if s.starts_with('-') || s.starts_with('+') { 1 } else { 0 };

    // Find out how many digits need to be changed
    let mut tail = len - 1;
    // Modified: Ensure that `tail` does not underflow when subtracting 1
    while tail >= begin && s.chars().nth(tail).unwrap() == tgt {
        if tail == 0 {
            break;
        }
        tail -= 1;
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        if begin == 0 {
            s.reserve(1);
            s.insert(0, '1');
        }
        for i in begin..len {
            s.replace_range(i..i+1, "0");
        }
        s.push('0');
    } else if tail == begin && neg && s.len() > 1 && s.chars().nth(1).unwrap() == '1' {
        // Special case: -1000..., so string will shrink
        for i in 1..len - begin {
            s.replace_range(i..i+1, "9");
        }
        s.truncate(len - 1);
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            s.replace_range(i..i+1, if neg { "9" } else { "0" });
        }
        let mut chars: Vec<char> = s.chars().collect();
        // Modified: Ensure that the addition operation does not cause an overflow
        // Corrected: Increment logic for positive numbers
        chars[tail] = (chars[tail] as u8).saturating_add(if neg { 1u8 } else { 1u8 }) as char;
        *s = chars.into_iter().collect();
    }

    s
}

fn string_test(s: &str) {
    let mut ret = String::with_capacity(s.len() * 2);
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