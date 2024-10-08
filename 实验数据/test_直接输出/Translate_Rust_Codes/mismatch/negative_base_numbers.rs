const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64, out: &mut String) {
    if base > -1 || base < -62 {
        *out = "".to_string();
        return;
    }
    if n == 0 {
        *out = "0".to_string();
        return;
    }

    let mut n = n;
    let mut result = String::new();

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            n += 1;
            rem -= base;
        }
        result.push(DIGITS.chars().nth(rem as usize).unwrap());
    }

    *out = result.chars().rev().collect();
}

fn decode_negative_base(ns: &str, base: i64) -> i64 {
    if base < -62 || base > -1 {
        return 0;
    }
    if ns.is_empty() || (ns == "0") {
        return 0;
    }

    let mut value = 0;
    let mut bb = 1;
    let mut ptr = ns.chars().rev();

    while let Some(ch) = ptr.next() {
        for (i, digit) in DIGITS.chars().enumerate() {
            if ch == digit {
                value += i as i64 * bb;
                bb *= base;
                break;
            }
        }
    }

    value
}

fn driver(n: i64, b: i64) {
    let mut buf = String::new();
    let value: i64;

    encode_negative_base(n, b, &mut buf);
    println!("{:12} encoded in base {:3} = {:12}", n, b, buf);

    value = decode_negative_base(&buf, b);
    println!("{:12} decoded in base {:3} = {:12}", buf, b, value);

    println!();
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}