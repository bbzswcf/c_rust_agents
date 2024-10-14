
use std::str;

fn incr(s: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut i: usize;
    let mut begin: usize;
    let mut tail: usize;
    let len = s.len();
    let neg = s[0] == b'-';
    let tgt = if neg { b'0' } else { b'9' };

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
        tail -= 1;
    }

    if tail < begin && !neg {
        // Special case: all 9s, string will grow
        s.reserve(1);
        s.insert(0, b'1');
        for i in begin..len {
            s[i] = b'0';
        }
        s.push(b'\0');
    } else if tail == begin && neg && s[1] == b'1' {
        // Special case: -1000..., so string will shrink
        for i in 1..len - begin {
            s[i] = b'9';
        }
        s.truncate(len - 1);
    } else {
        // Normal case; change tail to all 0 or 9, change prev digit by 1
        for i in (tail + 1..len).rev() {
            s[i] = if neg { b'9' } else { b'0' };
        }
        s[tail] += if neg { -1 } else { 1 };
    }

    s.pop(); // Remove the trailing '\0'
    s
}

fn string_test(s: &str) {
    let mut ret = s.as_bytes().to_vec();
    ret.push(b'\0'); // Null-terminate the string

    println!("text: {}", str::from_utf8(&ret).unwrap());
    incr(&mut ret);
    println!("  ->: {}", str::from_utf8(&ret).unwrap());
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