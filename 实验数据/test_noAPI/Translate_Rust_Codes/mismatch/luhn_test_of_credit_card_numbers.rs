fn luhn(cc: &str) -> bool {
    let mut sum = 0;
    let mut position = 0; // Start position from 0 to correctly identify every second digit from the right

    for (index, ch) in cc.chars().rev().enumerate() {
        if let Some(digit) = ch.to_digit(10) {
            let digit = digit as i32;
            // Modified: Correctly handle the Luhn algorithm's requirement
            if index % 2 != 0 { // Every second digit from the right (odd position in 0-based index)
                let doubled = digit * 2; // Double the digit
                sum += if doubled > 9 { 
                    doubled - 9 
                } else { 
                    doubled 
                };
            } else {
                sum += digit; // For even positions, simply add the digit to the sum
            }
        } else {
            return false; // Modified: Return false immediately if non-digit character is encountered
        }
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

    for card in cc.iter() { // Correctly iterate over the elements of the array
        println!("{:16}\t{}", card, if luhn(card) { "ok" } else { "not ok" });
    }
}