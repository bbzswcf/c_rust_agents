fn incr(s: &mut String) -> &mut String {
    let mut begin: usize;
    let mut tail: usize;
    let len = s.len();
    let neg = s.starts_with('-');
    let tgt = if neg { '0' } else { '9' };

    // Special case: "-1"
    if s == "-1" {
        s.clear();
        s.push('0');
        return s;
    }

    begin = if s.starts_with('-') || s.starts_with('+') { 1 } else { 0 };

    // Find out how many digits need to be changed
    tail = len - 1;
    // Ensure `tail` is within bounds before accessing `s.chars().nth(tail)`
    while tail >= begin && tail < s.len() && s.chars().nth(tail).map_or(false, |c| c == tgt) {
        // Use `saturating_sub` to prevent underflow
        tail = tail.saturating_sub(1);
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        if begin == 0 {
            s.reserve(len + 1);
        }
        // Ensure the position provided to `insert` is valid
        if begin <= s.len() {
            s.insert(0, '1');
        }
        for i in begin..len {
            // Ensure the range is valid before calling `replace_range`
            if i < s.len() && i + 1 <= s.len() {
                s.replace_range(i..i+1, "0");
            }
        }
    } else if tail == begin && neg && s.chars().nth(1) == Some('1') {
        // Special case: -1000..., so string will shrink
        for i in 1..len - begin {
            // Ensure the range is valid before calling `replace_range`
            if i < s.len() && i + 1 <= s.len() {
                s.replace_range(i..i+1, "9");
            }
        }
        // Ensure the length provided to `truncate` is valid
        if len > 1 {
            s.truncate(len - 1);
        }
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            // Ensure the range is valid before calling `replace_range`
            if i < s.len() && i + 1 <= s.len() {
                s.replace_range(i..i+1, if neg { "9" } else { "0" });
            }
        }
        let mut chars: Vec<char> = s.chars().collect();
        let increment = if neg { 255 } else { 1 };
        // Use checked_add to handle overflow explicitly
        chars[tail] = (chars[tail] as u8).checked_add(increment).map_or('0', |val| val as char);
        *s = chars.into_iter().collect();
    }

    s
}

fn string_test(s: &str) {
    let mut ret = s.to_string(); // Directly initialize `ret` with the content of `s`

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