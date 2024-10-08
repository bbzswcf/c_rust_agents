fn luhn(cc: &str) -> bool {
    let cc = cc.replace(" ", "").replace("-", ""); // Remove spaces and dashes from the card number
    let mut sum = 0;

    for (i, ch) in cc.chars().rev().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            let digit = digit as i32;
            // Modified: Adjusted the condition to apply doubling to every second digit from the right
            let doubled = if i % 2 == 0 { digit * 2 } else { digit };
            sum += if doubled > 9 { doubled - 9 } else { doubled }; // Adjust sum calculation
        } else {
            return false; // Return false immediately if any non-digit character is encountered
        }
    }

    sum % 10 == 0 // Check if the sum is divisible by 10
}

fn main() {
    let cc = [
        "49927398716",
        "49927398717",
        "1234567812345678",
        "1234567812345670",
    ];

    for &c in cc.iter() {
        println!("{:16}\t{}", c, if luhn(c) { "ok" } else { "not ok" });
    }
}