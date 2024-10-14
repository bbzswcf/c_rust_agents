fn rot(c: i32, str: &mut String) {
    let l = str.len();
    let alpha = [
        "abcdefghijklmnopqrstuvwxyz",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    ];

    for i in 0..l {
        if !str.chars().nth(i).unwrap().is_alphabetic() {
            continue;
        }

        let ch = str.chars().nth(i).unwrap();
        let index = (ch.to_ascii_lowercase() as u8 - b'a') as i32;
        let new_index = (index + c) % 26;

        if ch.is_uppercase() {
            str.replace_range(i..=i, &alpha[1].chars().nth(new_index as usize).unwrap().to_string());
        } else {
            str.replace_range(i..=i, &alpha[0].chars().nth(new_index as usize).unwrap().to_string());
        }
    }
}

fn caesar(str: &mut String) {
    rot(13, str);
}

fn decaesar(str: &mut String) {
    rot(13, str);
}

fn decrypt_rot(x: i32, str: &mut String) {
    rot(26 - x, str);
}

fn main() {
    let mut str = String::from("This is a top secret text message!");

    println!("Original: {}", str);
    caesar(&mut str);
    println!("Encrypted: {}", str);
    decaesar(&mut str);
    println!("Decrypted: {}", str);
}