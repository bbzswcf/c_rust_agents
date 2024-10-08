fn incr(s: &mut String) -> &mut String {
    let mut i: usize;
    let begin: usize;
    let mut tail: usize;
    let len = s.len();
    let neg = s.starts_with('-');
    let tgt = if neg { '0' } else { '9' };

    // Special case: "-1"
    if s == "-1" {
        *s = "0".to_string(); // Modified: Directly set the string to "0"
        return s;
    }

    begin = if s.starts_with('-') || s.starts_with('+') { 1 } else { 0 };

    // Find out how many digits need to be changed
    tail = len - 1;
    while tail > begin && tail < s.len() && s.chars().nth(tail).map_or(false, |ch| ch == tgt) {
        if tail == 0 { break; }
        if tail > begin { tail -= 1; }
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        s.insert(begin, '1');
        for i in begin + 1..len {
            s.replace_range(i..i+1, "0");
        }
        s.push('0');
    } else if tail == begin && neg && s.chars().nth(1).map_or(false, |ch| ch == '1') {
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
        chars[tail] = increment_char(chars[tail], if neg { -1 } else { 1 }).unwrap_or('0');
        *s = chars.into_iter().collect();
    }

    s
}

// Helper function to safely increment or decrement a character
fn increment_char(c: char, increment: i8) -> Result<char, &'static str> {
    let base = if c.is_ascii_digit() { '0' } else { match c {
        'a'..='z' => 'a',
        'A'..='Z' => 'A',
        _ => return Err("Unsupported character"),
    } };
    let value = c as u8 - base as u8;
    let new_value = ((value as i8 + increment + 10) % 10) as u8; // Wrap around using modulo 10
    Ok((base as u8 + new_value) as char)
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