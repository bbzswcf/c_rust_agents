fn rot(c: i32, str: &mut String) {
    let l = str.len();
    let alpha: [&str; 2] = ["abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"];

    for i in 0..l {
        if let Some(ch) = str.chars().nth(i) {
            if !ch.is_alphabetic() {
                continue;
            }

            if ch.is_uppercase() {
                if let Some(new_ch) = alpha[1].chars().nth(((ch.to_ascii_lowercase() as u8 - b'a') as i32 + c) as usize % 26) {
                    str.replace_range(i..=i, &new_ch.to_string());
                }
            } else {
                if let Some(new_ch) = alpha[0].chars().nth(((ch.to_ascii_lowercase() as u8 - b'a') as i32 + c) as usize % 26) {
                    str.replace_range(i..=i, &new_ch.to_string());
                }
            }
        }
    }
}

fn main() {
    let mut str = String::from("This is a top secret text message!");

    println!("Original: {}", str);
    rot(13, &mut str);
    println!("Encrypted: {}", str);
    rot(13, &mut str);
    println!("Decrypted: {}", str);
}