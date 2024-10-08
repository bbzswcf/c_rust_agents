fn value(x: char) -> i32 {
    const DIGITS: [i32; 26] = [
        0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0,
    ];
    DIGITS[(x.to_ascii_uppercase() as u8 - b'A') as usize]
}

fn decode(roman: &str) -> i32 {
    let mut arabic = 0;
    let mut roman = roman.chars();

    while let Some(current_char) = roman.next() {
        let current = value(current_char);
        let mut bigger = roman.clone();

        while let Some(next_char) = bigger.next() {
            if value(next_char) > current {
                arabic += value(next_char);
                for _ in 0..(bigger.as_str().len() + 1) {
                    arabic -= value(roman.next().unwrap());
                }
                break;
            }
        }

        if bigger.as_str().is_empty() {
            arabic += current;
        }
    }

    arabic
}

fn main() {
    let romans = ["MCmxC", "MMVIII", "MDClXVI", "MCXLUJ"];

    for &roman in &romans {
        println!("{}\t{}", roman, decode(roman));
    }
}