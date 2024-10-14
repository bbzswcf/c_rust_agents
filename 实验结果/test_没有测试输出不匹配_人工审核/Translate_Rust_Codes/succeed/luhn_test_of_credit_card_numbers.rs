fn luhn(cc: &str) -> bool {
    let m = [0, 2, 4, 6, 8, 1, 3, 5, 7, 9]; // mapping for rule 3
    let mut odd = true;
    let mut sum = 0;

    for &digit in cc.as_bytes().iter().rev() {
        let digit = (digit - b'0') as usize;
        sum += if odd { digit } else { m[digit] };
        odd = !odd;
    }

    sum % 10 == 0
}

fn main() {
    let cc = [
        "49927398716",
        "49927398717",
        "1234567812345678",
        "1234567812345670",
    ];

    for &card in cc.iter() {
        println!("{:16}\t{}", card, if luhn(card) { "ok" } else { "not ok" });
    }
}