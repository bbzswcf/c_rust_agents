use std::mem;
use std::ptr;
use std::str;

fn incr(s: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut i: usize;
    let mut begin: usize;
    let mut tail: usize;
    let len = s.len();
    let neg = s[0] == b'-';
    let tgt = if neg { b'9' } else { b'0' }; // Correctly set tgt based on whether the number is negative or positive

    // Special case: "-1"
    if s == b"-1" {
        s[0] = b'0';
        s.truncate(1);
        return s;
    }

    begin = if s[0] == b'-' || s[0] == b'+' { 1 } else { 0 };

    // Find out how many digits need to be changed
    tail = len - 1;
    while tail >= begin && s[tail] == tgt {
        if tail == 0 {
            break;
        }
        tail -= 1;
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        s.reserve(1); // Moved reserve inside the condition where it is actually needed
        s.insert(0, b'1');
        for i in begin..len {
            s[i] = b'0';
        }
    } else if tail == begin && neg && s[1] == b'0' {
        // Special case: -000..., so string will shrink
        for i in 1..len - begin {
            s[i] = b'9';
        }
        if len > 1 { // Ensure len > 1 before truncating to avoid panic
            s.truncate(len - 1);
        }
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            s[i] = if neg { b'9' } else { b'0' };
        }
        // Use checked_add to handle potential overflow safely
        if let Some(new_value) = s[tail].checked_add(if neg { 9 } else { 1 }) {
            s[tail] = new_value;
        } else {
            // Handle overflow case more gracefully
            s[tail] = if neg { b'0' } else { b'9' };
            // Adjust the previous digit
            if tail > begin {
                s[tail - 1] += if neg { 1 } else { -1 };
            } else if neg {
                s.insert(0, b'-');
            } else {
                s.insert(0, b'1');
            }
        }
    }

    s
}

fn string_test(s: &str) {
    let mut ret = s.as_bytes().to_vec();
    ret.extend(s.as_bytes());

    println!("text: {}", str::from_utf8(&ret).expect("Invalid UTF-8 sequence")); // Use expect with a descriptive error message
    incr(&mut ret);
    println!("  ->: {}", str::from_utf8(&ret).expect("Invalid UTF-8 sequence")); // Use expect with a descriptive error message
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