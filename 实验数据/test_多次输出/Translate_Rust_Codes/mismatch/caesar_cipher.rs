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
        let base = if ch.is_uppercase() {
            alpha[1].chars().nth(0).unwrap()
        } else {
            alpha[0].chars().nth(0).unwrap()
        };

        let new_ch = if ch.is_uppercase() {
            alpha[1].chars().nth(
                ((ch.to_lowercase().next().unwrap() as i32 - base as i32 + c) % 26) as usize
            ).unwrap()
        } else {
            alpha[0].chars().nth(
                ((ch.to_lowercase().next().unwrap() as i32 - base as i32 + c) % 26) as usize
            ).unwrap()
        };

        str.replace_range(i..=i, &new_ch.to_string());
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