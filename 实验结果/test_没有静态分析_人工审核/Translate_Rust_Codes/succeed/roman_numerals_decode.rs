fn value(x: char) -> i32 {
    let digits = [0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0];
    digits[(x.to_ascii_uppercase() as u8 - b'A') as usize]
}

fn decode(roman: &str) -> i32 {
    let mut roman = roman.chars().peekable();
    let mut arabic = 0;

    while let Some(ch) = roman.next() {
        let current = value(ch);

        // Check if the next character exists and has a greater value
        if let Some(&next_ch) = roman.peek() {
            let next_value = value(next_ch);
            if next_value > current {
                // Subtract the current value because the next value is greater
                arabic -= current;
                continue;
            }
        }

        // Add the current value to the total
        arabic += current;
    }

    arabic
}

fn main() {
    let romans = ["MCmxC", "MMVIII", "MDClXVI", "MCXLUJ"];

    for roman in romans.iter() {
        println!("{}\t{}", roman, decode(roman));
    }
}