// Modified: Expanded the mapping of Roman numeral characters to include all valid characters and their respective values
const CORRECT_DIGITS: [i32; 13] = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
const ROMAN_CHARS: [&str; 13] = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

fn value(x: &str) -> Result<i32, &'static str> {
    // Modified: Handle invalid Roman numeral characters gracefully
    match ROMAN_CHARS.iter().position(|&r| r == x) {
        Some(index) => Ok(CORRECT_DIGITS[index]),
        None => Err("Invalid Roman numeral character"),
    }
}

fn decode(roman: &str) -> Result<i32, &'static str> {
    let mut roman = roman.chars().peekable();
    let mut arabic = 0;

    while let Some(ch) = roman.next() {
        let current = match value(&ch.to_string()) {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        // Modified: Correctly handle the subtraction rule in Roman numerals
        if let Some(&next_ch) = roman.peek() {
            let next = match value(&format!("{}{}", ch, next_ch)) {
                Ok(val) => val,
                Err(_) => match value(&next_ch.to_string()) {
                    Ok(val) => val,
                    Err(e) => return Err(e),
                },
            };

            if next > current {
                arabic += next - current;
                roman.next(); // Skip the next character as it has been processed
            } else {
                arabic += current;
            }
        } else {
            arabic += current;
        }
    }

    Ok(arabic)
}

fn main() {
    let romans = ["MCmxC", "MMVIII", "MDClXVI", "MCXLUJ"];

    for &roman in &romans {
        match decode(roman) {
            Ok(result) => println!("{}\t{}", roman, result),
            Err(e) => println!("{}\tError: {}", roman, e),
        }
    }
}