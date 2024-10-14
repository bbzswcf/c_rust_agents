use std::str::Chars;

const TRUE: bool = true;
const FALSE: bool = false;

#[derive(PartialEq)]
enum Cmode {
    Encrypt,
    Decrypt,
}

const L_ALPHABET: &str = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
const R_ALPHABET: &str = "PTLNBQDEOYSFAVZKGJRIHWXUMC";

fn chao(input: &str, output: &mut String, mode: Cmode, show_steps: bool) {
    let mut left: Vec<char> = L_ALPHABET.chars().collect();
    let mut right: Vec<char> = R_ALPHABET.chars().collect();
    let mut temp: Vec<char> = vec!['\0'; 26];
    let len = input.len();

    for (i, c) in input.chars().enumerate() {
        if show_steps {
            // Modified: Use `iter().collect()` to convert `Vec<char>` to `String`
            println!("{}  {}", left.iter().collect::<String>(), right.iter().collect::<String>());
        }
        let index: usize;
        if mode == Cmode::Encrypt {
            index = right.iter().position(|&x| x == c).unwrap();
            output.push(left[index]);
        } else {
            index = left.iter().position(|&x| x == c).unwrap();
            output.push(right[index]);
        }
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
        // Modified: Modify `left` in place instead of cloning `temp`
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
        // Modified: Modify `right` in place instead of cloning `temp`
        right.copy_from_slice(&temp);
    }
}

fn main() {
    let plain_text = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text = String::with_capacity(plain_text.len() + 1);
    let mut plain_text2 = String::with_capacity(plain_text.len() + 1);

    println!("The original plaintext is : {}", plain_text);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(plain_text, &mut cipher_text, Cmode::Encrypt, TRUE);
    println!("\nThe ciphertext is : {}", cipher_text);
    chao(&cipher_text, &mut plain_text2, Cmode::Decrypt, FALSE);
    println!("\nThe recovered plaintext is : {}", plain_text2);
}