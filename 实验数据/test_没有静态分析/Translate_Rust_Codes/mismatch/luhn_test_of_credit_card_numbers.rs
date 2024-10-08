fn luhn(cc: &str) -> bool {
    let mut sum = 0;
    let mut double = false; // Initialize based on the position of the digit

    // Iterate over the string in reverse order without reversing the string
    for &digit in cc.as_bytes().iter().rev() {
        if !digit.is_ascii_digit() {
            return false; // Invalid input, return false immediately
        }
        let digit = (digit - b'0') as i32;
        // Correctly handle the doubling logic for the Luhn algorithm
        sum += if double {
            let doubled = digit * 2;
            if doubled > 9 { doubled - 9 } else { doubled }
        } else {
            digit
        };
        double = !double;
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

    for &card in cc.iter() {
        // Ensure the output formatting matches the expected format
        println!("{:16}\t{}", card, if luhn(card) { "ok" } else { "not ok" });
    }
}