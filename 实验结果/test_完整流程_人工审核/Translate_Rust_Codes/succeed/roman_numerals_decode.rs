fn value(x: char) -> Option<i32> {
    let digits = [0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0];
    let index = (x.to_ascii_uppercase() as u8 - b'A') as usize;
    if index < digits.len() {
        Some(digits[index])
    } else {
        None
    }
}

fn decode(roman: &str) -> Result<i32, &'static str> {
    let mut roman = roman.chars();
    let mut arabic = 0;

    while let Some(ch) = roman.next() {
        let current = match value(ch) {
            Some(v) => v,
            None => return Err("Invalid character in Roman numeral"),
        };

        if let Some(next_ch) = roman.clone().next() {
            let next_value = match value(next_ch) {
                Some(v) => v,
                None => return Err("Invalid character in Roman numeral"),
            };

            if next_value > current {
                arabic += next_value - current;
                roman.next(); // Consume the next character
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

    for roman in romans.iter() {
        match decode(roman) {
            Ok(result) => println!("{}\t{}", roman, result),
            Err(e) => println!("{}\tError: {}", roman, e),
        }
    }
}