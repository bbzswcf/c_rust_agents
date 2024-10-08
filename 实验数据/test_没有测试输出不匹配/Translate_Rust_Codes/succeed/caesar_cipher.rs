fn rot(c: i32, str: &mut String) {
    let l = str.len();
    let alpha: [&str; 2] = ["abcdefghijklmnopqrstuvwxyz", "ABCDEFGHIJKLMNOPQRSTUVWXYZ"];

    for i in 0..l {
        if let Some(ch) = str.chars().nth(i) {
            if !ch.is_alphabetic() {
                continue;
            }

            if ch.is_uppercase() {
                let lower = ch.to_ascii_lowercase();
                let index = (lower as u8 - 'a' as u8) as i32;
                let new_index = (index + c) % 26;
                if let Some(new_char) = alpha[1].chars().nth(new_index as usize) {
                    str.replace_range(i..=i, &new_char.to_string());
                }
            } else {
                let index = (ch as u8 - 'a' as u8) as i32;
                let new_index = (index + c) % 26;
                if let Some(new_char) = alpha[0].chars().nth(new_index as usize) {
                    str.replace_range(i..=i, &new_char.to_string());
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