use std::mem;
use std::ptr;
use std::str;

fn incr(s: &mut Vec<u8>) -> &mut Vec<u8> {
    let len = s.len();
    let neg = s[0] == b'-';
    let tgt = if neg { b'0' } else { b'9' };

    // Special case: "-1"
    if s.len() == 2 && s[0] == b'-' && s[1] == b'1' {
        s[0] = b'0';
        return s;
    }

    let begin = if s[0] == b'-' || s[0] == b'+' { 1 } else { 0 };

    // Find out how many digits need to be changed
    let mut tail = len - 1;
    while tail >= begin && s[tail] == tgt {
        tail = tail.saturating_sub(1);
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        s.reserve(1); // Ensure capacity is sufficient
        unsafe {
            // Ensure capacity is sufficient before copying
            ptr::copy(s.as_ptr(), s.as_mut_ptr().offset(1), len);
            // Ensure the index is within bounds before assigning
            if len > 0 {
                s[0] = b'1';
            }
            for i in 1..=len - begin {
                // Ensure the index is within bounds before assigning
                if i < s.len() {
                    s[i] = b'0';
                }
            }
            s.set_len(len + 1);
        }
    } else if tail == begin && neg && s[1] == b'1' {
        // Special case: -1000..., so string will shrink
        for i in 1..len - begin {
            // Ensure the index is within bounds before assigning
            if i < s.len() {
                s[i] = b'9';
            }
        }
        // Ensure len is greater than 1 before truncating
        if len > 1 {
            s.truncate(len - 1);
        }
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1)..len {
            // Ensure tail + 1 is less than len before entering the loop
            if tail < len - 1 {
                s[i] = if neg { b'9' } else { b'0' };
            }
        }
        // Ensure tail is within bounds before accessing
        if tail < s.len() {
            if neg {
                s[tail] = s[tail].saturating_sub(1);
            } else {
                s[tail] = s[tail].saturating_add(1);
            }
        }
    }

    s
}

fn string_test(s: &str) {
    let mut ret = s.as_bytes().to_vec();

    match str::from_utf8(&ret) {
        Ok(text) => println!("text: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }

    incr(&mut ret);

    match str::from_utf8(&ret) {
        Ok(text) => println!("  ->: {}", text),
        Err(e) => eprintln!("Error: {}", e),
    }
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