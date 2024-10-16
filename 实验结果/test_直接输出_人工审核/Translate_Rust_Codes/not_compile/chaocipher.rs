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
    let mut left: Vec<u8> = L_ALPHABET.as_bytes().to_vec();
    let mut right: Vec<u8> = R_ALPHABET.as_bytes().to_vec();
    let mut temp: Vec<u8> = vec![0; 27];
    temp[26] = b'\0';

    for i in 0..len {
        if show_steps {
            println!("{}  {}", str::from_utf8(&left).unwrap(), str::from_utf8(&right).unwrap());
        }
        let index = if mode == Cmode::Encrypt {
            right.iter().position(|&c| c == input.as_bytes()[i]).unwrap()
        } else {
            left.iter().position(|&c| c == input.as_bytes()[i]).unwrap()
        };
        output[i] = left[index];
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
        left.copy_from_slice(&temp[..26]);

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
        right.copy_from_slice(&temp[..26]);
    }
}

fn main() {
    const PLAIN_TEXT: &str = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text: Vec<u8> = vec![0; PLAIN_TEXT.len() + 1];
    let mut plain_text2: Vec<u8> = vec![0; PLAIN_TEXT.len() + 1];

    println!("The original plaintext is : {}", PLAIN_TEXT);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(PLAIN_TEXT, &mut cipher_text, Cmode::Encrypt, TRUE);
    println!("\nThe ciphertext is : {}", str::from_utf8(&cipher_text).unwrap());
    chao(str::from_utf8(&cipher_text).unwrap(), &mut plain_text2, Cmode::Decrypt, FALSE);
    println!("\nThe recovered plaintext is : {}", str::from_utf8(&plain_text2).unwrap());
}