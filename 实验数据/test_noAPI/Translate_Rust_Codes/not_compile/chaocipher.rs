use std::str;

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(PartialEq)]
enum Cmode {
    Encrypt,
    Decrypt,
}

const L_ALPHABET: &str = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
const R_ALPHABET: &str = "PTLNBQDEOYSFAVZKGJRIHWXUMC";

fn chao(input: &str, output: &mut [u8], mode: Cmode, show_steps: bool) {
    let len = input.len();
    let mut left: Vec<char> = L_ALPHABET.chars().collect();
    let mut right: Vec<char> = R_ALPHABET.chars().collect();
    let mut temp: Vec<char> = vec!['\0'; 26];

    for (i, &ch) in input.as_bytes().iter().enumerate() {
        if show_steps {
            // Modified: Use `collect` directly on the iterator to create a String
            println!("{}  {}", left.iter().collect::<String>(), right.iter().collect::<String>());
        }

        // Ensure the input character is within the expected range of the left alphabet
        if !left.contains(&(ch as char)) {
            panic!("Character not found in left alphabet: {}", ch as char);
        }

        let index = if mode == Cmode::Encrypt {
            right.iter().position(|&c| c == ch as char).expect("Character not found in right alphabet")
        } else {
            left.iter().position(|&c| c == ch as char).expect("Character not found in left alphabet")
        };
        output[i] = left[index] as u8;
        if i == len - 1 {
            break;
        }

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
        left = temp.clone();

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
        right = temp.clone();
    }
}

fn main() {
    const PLAIN_TEXT: &str = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text: Vec<u8> = vec![0; PLAIN_TEXT.len() + 1];
    let mut plain_text2: Vec<u8> = vec![0; PLAIN_TEXT.len() + 1];

    println!("The original plaintext is : {}", PLAIN_TEXT);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(PLAIN_TEXT, &mut cipher_text, Cmode::Encrypt, TRUE);
    // Ensure the ciphertext is correctly encoded as UTF-8
    match std::str::from_utf8(&cipher_text) {
        Ok(s) => println!("\nThe ciphertext is : {}", s),
        Err(_) => panic!("Invalid UTF-8 sequence in ciphertext"),
    }
    chao(std::str::from_utf8(&cipher_text).expect("Invalid UTF-8 sequence in ciphertext"), &mut plain_text2, Cmode::Decrypt, FALSE);
    // Ensure the plain_text2 is correctly encoded as UTF-8
    match std::str::from_utf8(&plain_text2) {
        Ok(s) => println!("\nThe recovered plaintext is : {}", s),
        Err(_) => panic!("Invalid UTF-8 sequence in plain_text2"),
    }
}