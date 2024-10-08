const DIGITS: [i32; 7] = [1, 5, 10, 50, 100, 500, 1000];
const ROMAN_CHARS: [char; 7] = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];

fn value(x: char) -> Option<i32> {
    ROMAN_CHARS.iter().position(|&c| c == x).map(|i| DIGITS[i])
}

fn decode(roman: &str) -> Result<i32, String> {
    let mut roman = roman.chars().peekable();
    let mut arabic = 0;
    let mut index = 0;

    while let Some(current_char) = roman.next() {
        index += 1;
        let current = match value(current_char.to_ascii_uppercase()) {
            Some(v) => v,
            None => return Err(format!("Invalid character '{}' at position {} in Roman numeral", current_char, index)),
        };

        if let Some(&next_char) = roman.peek() {
            if let Some(next) = value(next_char.to_ascii_uppercase()) {
                if next > current && (current == 1 || current == 10 || current == 100) && (next / current <= 10) {
                    arabic += next - current;
                    roman.next(); // Consume the next character
                    index += 1;
                    continue;
                }
            }
        }

        arabic += current;
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