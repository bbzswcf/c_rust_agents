const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64, out: &mut String) {
    // Modified: Corrected base check condition
    if base >= -1 || base < -62 {
        *out = "".to_string();
        return;
    }
    if n == 0 {
        *out = "0".to_string();
        return;
    }

    let mut n = n;
    let mut buffer = Vec::new();

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            rem += base.abs();
            n -= 1; // Correct adjustment for negative base
        }
        let index = (rem.abs() as usize) % DIGITS_LEN;
        buffer.push(DIGITS.chars().nth(index).unwrap());
    }

    buffer.reverse();
    // Modified: Corrected buffer conversion to string
    *out = buffer.into_iter().collect::<String>();
}

fn decode_negative_base(ns: &str, base: i64) -> i64 {
    // Modified: Corrected base check condition
    if base <= -1 || base > -62 {
        return 0;
    }
    // Modified: Corrected empty string check condition
    if ns.is_empty() || ns == "0" {
        return 0;
    }

    let mut value = 0;
    let mut bb = 1;
    let mut ptr = ns.chars().rev();

    while let Some(ch) = ptr.next() {
        if let Some(i) = DIGITS.find(ch) {
            value += i as i64 * bb;
            bb *= base; // Correct multiplication by the base
        }
    }

    value
}

fn driver(n: i64, b: i64) {
    let mut buf = String::new();
    let value;

    encode_negative_base(n, b, &mut buf);
    println!("{:<12} encoded in base {:3} = {:<12}", n, b, buf);

    value = decode_negative_base(&buf, b);
    println!("{:<12} decoded in base {:3} = {:<12}", buf, b, value);

    println!();
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}