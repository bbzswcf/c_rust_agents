const DIGITS: [i32; 26] = [0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0];

fn value(x: u8) -> i32 {
    DIGITS[(x & !0x20) as usize - 'A' as usize]
}

fn decode(roman: &str) -> i32 {
    let mut roman = roman.as_bytes();
    let mut arabic = 0;

    while let Some(&current_char) = roman.first() {
        let current = value(current_char);
        let mut bigger = roman;

        while let Some(&next_char) = bigger.first() {
            if value(next_char) > current {
                break;
            }
            bigger = &bigger[1..];
        }

        if bigger.is_empty() {
            arabic += current;
        } else {
            arabic += value(bigger[0]);
            while roman < bigger {
                arabic -= value(roman[0]);
                roman = &roman[1..];
            }
        }

        roman = &roman[1..];
    }

    arabic
}

fn main() {
    let romans = ["MCmxC", "MMVIII", "MDClXVI", "MCXLUJ"];

    for &roman in &romans {
        println!("{}\t{}", roman, decode(roman));
    }
}