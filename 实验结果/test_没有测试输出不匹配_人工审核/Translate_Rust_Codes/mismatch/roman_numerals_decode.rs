fn value(x: char) -> i32 {
    let digits: [i32; 26] = [0, 0, 100, 500, 0, 0, 0, 0, 1, 1, 0, 50, 1000, 0, 0, 0, 0, 0, 0, 0, 5, 5, 0, 10, 0, 0];
    digits[(x.to_ascii_uppercase() as u8 - b'A') as usize]
}

fn decode(roman: &str) -> i32 {
    let mut roman = roman.chars();
    let mut arabic = 0;

    while let Some(ch) = roman.next() {
        let current = value(ch);
        let mut bigger = roman.clone();

        while let Some(next_ch) = bigger.next() {
            if value(next_ch) > current {
                break;
            }
        }

        if let Some(next_ch) = bigger.next() {
            arabic += value(next_ch);
            for ch in roman.by_ref().take_while(|&x| x != next_ch) {
                arabic -= value(ch);
            }
        } else {
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