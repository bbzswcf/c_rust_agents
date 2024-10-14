use std::str::from_utf8;

#[derive(PartialEq)]
enum Cmode {
    Encrypt,
    Decrypt,
}

const L_ALPHABET: &str = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
const R_ALPHABET: &str = "PTLNBQDEOYSFAVZKGJRIHWXUMC";

fn chao(input: &str, output: &mut [u8], mode: Cmode, show_steps: bool) -> Result<(), Box<dyn std::error::Error>> {
    let len = input.len();
    let mut left: Vec<char> = L_ALPHABET.chars().collect();
    let mut right: Vec<char> = R_ALPHABET.chars().collect();
    // Modified: Initialize temp with the characters from the left alphabet
    let mut temp: Vec<char> = left.clone();

    // Validate input string to ensure all characters are in the left or right alphabet
    for c in input.chars() {
        if !left.contains(&c) && !right.contains(&c) {
            return Err(format!("Character '{}' not found in left or right alphabet", c).into());
        }
    }

    for i in 0..len {
        if show_steps {
            // Modified: Ensure the characters are valid UTF-8 by constructing the string correctly
            println!("{}  {}", left.iter().map(|&c| c as char).collect::<String>(), right.iter().map(|&c| c as char).collect::<String>());
        }
        let index: usize;
        if mode == Cmode::Encrypt {
            // Modified: Use a match statement within the closure to handle the Option returned by input.chars().nth(i)
            index = right.iter().position(|&c| {
                match input.chars().nth(i) {
                    Some(ch) => c == ch,
                    None => false,
                }
            }).ok_or("Character not found in right alphabet")?;
            output[i] = left[index] as u8;
        } else {
            // Modified: Use a match statement within the closure to handle the Option returned by input.chars().nth(i)
            index = left.iter().position(|&c| {
                match input.chars().nth(i) {
                    Some(ch) => c == ch,
                    None => false,
                }
            }).ok_or("Character not found in left alphabet")?;
            output[i] = right[index] as u8;
        }
        if i == len - 1 {
            break;
        }

        // Simplified permutation logic for left and right alphabets
        // permute left
        for j in index..26 {
            temp[j - index] = left[j];
        }
        for j in 0..index {
            temp[26 - index + j] = left[j];
        }
        let store = temp[1];
        for j in 2..14 {
            temp[j - 1] = temp[j];
        }
        temp[13] = store;
        left.copy_from_slice(&temp);

        // permute right
        for j in index..26 {
            temp[j - index] = right[j];
        }
        for j in 0..index {
            temp[26 - index + j] = right[j];
        }
        let store = temp[0];
        for j in 1..26 {
            temp[j - 1] = temp[j];
        }
        temp[25] = store;
        let store = temp[2];
        for j in 3..14 {
            temp[j - 1] = temp[j];
        }
        temp[13] = store;
        right.copy_from_slice(&temp);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const PLAIN_TEXT: &str = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text = vec![0; PLAIN_TEXT.len() + 1];
    let mut plain_text2 = vec![0; PLAIN_TEXT.len() + 1];

    println!("The original plaintext is : {}", PLAIN_TEXT);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(PLAIN_TEXT, &mut cipher_text, Cmode::Encrypt, true)?;
    // Modified: Convert Vec<char> to Vec<u8> before passing to from_utf8
    let cipher_text_u8: Vec<u8> = cipher_text.iter().map(|&b| b as u8).collect();
    println!("\nThe ciphertext is : {}", from_utf8(&cipher_text_u8)?);
    // Modified: Convert Vec<char> to Vec<u8> before passing to from_utf8
    let cipher_text_str = from_utf8(&cipher_text_u8)?;
    chao(cipher_text_str, &mut plain_text2, Cmode::Decrypt, false)?;
    // Modified: Convert Vec<char> to Vec<u8> before passing to from_utf8
    let plain_text2_u8: Vec<u8> = plain_text2.iter().map(|&b| b as u8).collect();
    println!("\nThe recovered plaintext is : {}", from_utf8(&plain_text2_u8)?);

    Ok(())
}