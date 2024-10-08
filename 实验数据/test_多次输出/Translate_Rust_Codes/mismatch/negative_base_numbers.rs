const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64) -> String {
    if base > -1 || base < -62 {
        return String::new();
    }
    if n == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut n = n;

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            n += 1;
            rem -= base;
        }
        result.push(DIGITS.chars().nth(rem as usize).unwrap());
    }

    result.chars().rev().collect()
}

fn decode_negative_base(ns: &str, base: i64) -> i64 {
    if base < -62 || base > -1 {
        return 0;
    }
    if ns.is_empty() || (ns.len() == 1 && ns == "0") {
        return 0;
    }

    let mut value = 0;
    let mut bb = 1;

    for c in ns.chars().rev() {
        for (i, digit) in DIGITS.chars().enumerate() {
            if c == digit {
                value += i as i64 * bb;
                bb *= base;
                break;
            }
        }
    }

    value
}

fn driver(n: i64, b: i64) {
    let encoded = encode_negative_base(n, b);
    println!("{:12} encoded in base {:3} = {:12}", n, b, encoded);

    let decoded = decode_negative_base(&encoded, b);
    println!("{:12} decoded in base {:3} = {:12}", encoded, b, decoded);

    println!();
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}