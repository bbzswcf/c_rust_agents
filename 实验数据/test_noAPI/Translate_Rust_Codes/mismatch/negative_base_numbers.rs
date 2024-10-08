const DIGITS: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const DIGITS_LEN: usize = 64;

fn encode_negative_base(n: i64, base: i64) -> Result<String, &'static str> {
    // Modified: Corrected base check to ensure the base is within the valid range for encoding
    if base < -62 || base > -1 {
        return Err("Base out of bounds");
    }
    if n == 0 {
        return Ok("0".to_string());
    }

    let mut result = String::new();
    let mut n = n;

    while n != 0 {
        let mut rem = n % base;
        n /= base;
        // Modified: Ensured `rem` is always non-negative
        if rem < 0 {
            rem += base.abs();
            n += 1; // Adjust `n` to account for the negative base
        }
        // Modified: Added a check to ensure `rem` is within the bounds of `DIGITS`
        if rem < 0 || rem >= DIGITS_LEN as i64 {
            return Err("Remainder out of bounds");
        }
        result.push(DIGITS.chars().nth(rem as usize).unwrap());
    }

    Ok(result.chars().rev().collect())
}

fn decode_negative_base(ns: &str, base: i64) -> Result<i64, &'static str> {
    // Modified: Corrected base check to ensure the base is within the valid range for decoding
    if base < -62 || base > -1 {
        return Err("Base out of bounds");
    }
    // Modified: Corrected the condition to handle the case where `ns` is empty or contains only "0"
    if ns == "0" {
        return Ok(0);
    }

    let mut value = 0;
    let mut bb = 1;

    for c in ns.chars().rev() {
        // Modified: Correctly cast `digit` to `i64`
        let digit = DIGITS.find(c).ok_or("Invalid character in input")? as i64;
        value += digit * bb;
        // Modified: Correctly update `bb` to handle negative bases
        bb *= base;
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