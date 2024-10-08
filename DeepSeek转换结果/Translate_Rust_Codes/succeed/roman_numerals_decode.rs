const DIGITS: [i32; 26] = [ 0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0 ];

fn value(x: char) -> i32 {
    DIGITS[(x.to_ascii_uppercase() as u8 - b'A') as usize]
}

fn decode(roman: &str) -> i32 {
    let mut roman = roman.chars().collect::<Vec<_>>();
    let mut arabic = 0;
    let mut i = 0;

    while i < roman.len() {
        let current = value(roman[i]);

        if i + 1 < roman.len() && value(roman[i + 1]) > current {
            arabic += value(roman[i + 1]) - current;
            i += 2;
        } else {
            arabic += current;
            i += 1;
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