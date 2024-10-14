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
    while tail >= begin && s.chars().nth(tail).map_or(false, |c| c == tgt) {
        if tail > begin { // Ensure tail does not underflow
            tail -= 1;
        } else {
            break;
        }
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        if begin == 0 {
            if s.capacity() < s.len() + 1 { // Ensure capacity increase is necessary
                s.reserve(1);
            }
            s.insert(0, '1');
        } else {
            s.insert(0, '1');
        }
        for i in begin..len {
            if i < s.len() { // Ensure index is within bounds
                s.replace_range(i..i+1, "0");
            }
        }
    } else if tail == begin && neg && s.chars().nth(1).map_or(false, |c| c == '1') {
        // Special case: -1000..., so string will shrink
        for i in 1..len - begin {
            if i < s.len() { // Ensure index is within bounds
                s.replace_range(i..i+1, "9");
            }
        }
        if len > 1 { // Ensure length is valid
            s.truncate(len - 1);
        }
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            if i < s.len() { // Ensure index is within bounds
                s.replace_range(i..i+1, if neg { "9" } else { "0" });
            }
        }
        let mut chars: Vec<char> = s.chars().collect();
        if neg {
            // Convert char to u32 for arithmetic operations, then back to char
            if let Some(new_char) = chars[tail].to_digit(10).and_then(|d| {
                if d > 0 {
                    d.checked_sub(1)
                } else {
                    Some(9) // Handle overflow case, e.g., by setting to '9'
                }
            }).and_then(|d| std::char::from_digit(d, 10)) {
                chars[tail] = new_char;
            } else {
                // Handle invalid digit character
                chars[tail] = '9';
            }
        } else {
            // Convert char to u32 for arithmetic operations, then back to char
            if let Some(new_char) = chars[tail].to_digit(10).and_then(|d| {
                d.checked_add(1)
            }).and_then(|d| std::char::from_digit(d, 10)) {
                chars[tail] = new_char;
            } else {
                // Handle invalid digit character
                chars[tail] = '0';
            }
        }
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