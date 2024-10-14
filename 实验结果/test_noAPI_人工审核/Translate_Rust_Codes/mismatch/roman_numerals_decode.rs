const DIGITS: [i32; 26] = [
    0, 1, 5, 10, 50, 100, 500, 1000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn value(x: char) -> Result<i32, &'static str> {
    // Modified: Check if the character is a valid Roman numeral character
    match x {
        'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' |
        'i' | 'v' | 'x' | 'l' | 'c' | 'd' | 'm' => {
            let index = (x.to_ascii_uppercase() as u8 - b'A') as usize;
            Ok(DIGITS[index])
        }
        _ => Err("Invalid Roman numeral character"),
    }
}

fn decode(roman: &str) -> Result<i32, &'static str> {
    let mut roman = roman.chars().peekable();
    let mut arabic = 0;

    while let Some(current_char) = roman.next() {
        let current = value(current_char)?;
        if let Some(&next_char) = roman.peek() {
            let next = value(next_char)?;
            // Modified: Correctly handle the subtraction rule
            if next > current {
                arabic -= current;
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
            Ok(arabic) => println!("{}\t{}", roman, arabic),
            Err(e) => println!("{}\tError: {}", roman, e),
        }
    }
}