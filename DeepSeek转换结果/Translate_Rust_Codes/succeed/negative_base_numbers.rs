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
    let mut ptr = 0; // 修改: 初始化ptr为0，而不是out.len()
    out.clear(); // 修改: 清空out，而不是push '\0'

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            n += 1;
            rem -= base;
        }
        out.insert(ptr, DIGITS.chars().nth(rem as usize).unwrap());
        ptr += 1; // 修改: 增加ptr的值，而不是减少
    }

    *out = out.chars().rev().collect::<String>(); // 修改: 反转字符串
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
    let mut ptr = ns.len() as i64 - 1;

    while ptr >= 0 {
        let ch = ns.chars().nth(ptr as usize).unwrap();
        for i in 0..DIGITS_LEN {
            if ch == DIGITS.chars().nth(i).unwrap() {
                value += i as i64 * bb;
                bb *= base;
                break;
            }
        }
        ptr -= 1;
    }

    value
}

fn driver(n: i64, b: i64) {
    let mut buf = String::with_capacity(64);
    let value: i64;

    encode_negative_base(n, b, &mut buf);
    // 修改: 使用{:>12}格式化输出
    println!("{:>12} encoded in base {:3} = {:>12}", n, b, buf);

    value = decode_negative_base(&buf, b);
    // 修改: 使用{:>12}格式化输出
    println!("{:>12} decoded in base {:3} = {:>12}", buf, b, value);

    println!();
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}