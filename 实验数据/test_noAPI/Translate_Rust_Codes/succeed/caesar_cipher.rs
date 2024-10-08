use std::ascii::AsciiExt;

fn rot(c: i32, str: &mut String) {
    let l = str.len();
    let alpha: [&str; 2] = [
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    ];

    let mut chars: Vec<char> = str.chars().collect();

    for i in 0..l {
        if !chars[i].is_alphabetic() {
            continue;
        }

        if chars[i].is_uppercase() {
            chars[i] = alpha[1].chars().nth(
                ((chars[i].to_ascii_lowercase() as u8 - b'a') as i32 + c) as usize % 26
            ).unwrap();
        } else {
            chars[i] = alpha[0].chars().nth(
                ((chars[i].to_ascii_lowercase() as u8 - b'a') as i32 + c) as usize % 26
            ).unwrap();
        }
    }

    *str = chars.into_iter().collect();
}

fn main() {
    let mut str = String::from("This is a top secret text message!");

    println!("Original: {}", str);
    rot(13, &mut str);
    println!("Encrypted: {}", str);
    rot(13, &mut str);
    println!("Decrypted: {}", str);
}