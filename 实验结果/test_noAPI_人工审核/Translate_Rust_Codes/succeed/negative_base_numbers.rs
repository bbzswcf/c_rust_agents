const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64) -> Result<String, &'static str> {
    if base > -1 || base < -62 {
        return Err("Base out of bounds");
    }
    if n == 0 {
        return Ok("0".to_string());
    }

    let mut n = n;
    let mut result = Vec::new();

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        if rem < 0 {
            n += 1;
            rem -= base;
        }
        result.push(DIGITS.chars().nth(rem as usize).unwrap());
    }

    result.reverse();
    Ok(result.into_iter().collect())
}

fn decode_negative_base(ns: &str, base: i64) -> Result<i64, &'static str> {
    if base < -62 || base > -1 {
        return Err("Base out of bounds");
    }
    if ns.is_empty() || (ns.len() == 1 && ns == "0") {
        return Ok(0);
    }

    let mut value = 0;
    let mut bb = 1;
    let mut chars = ns.chars().rev();

    while let Some(c) = chars.next() {
        if let Some(i) = DIGITS.find(c) {
            value += i as i64 * bb;
            bb *= base;
        } else {
            return Err("Invalid character in input string");
        }
    }

    Ok(value)
}

fn driver(n: i64, b: i64) {
    match encode_negative_base(n, b) {
        Ok(encoded) => {
            println!("{:12} encoded in base {:3} = {:12}", n, b, encoded);
            match decode_negative_base(&encoded, b) {
                Ok(decoded) => println!("{:12} decoded in base {:3} = {:12}", encoded, b, decoded),
                Err(e) => eprintln!("Error decoding: {}", e),
            }
        }
        Err(e) => eprintln!("Error encoding: {}", e),
    }
    println!();
}

fn main() {
    driver(10, -2);
    driver(146, -3);
    driver(15, -10);
    driver(12, -62);
}