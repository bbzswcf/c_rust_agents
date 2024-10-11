const L_ALPHABET: &str = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
const R_ALPHABET: &str = "PTLNBQDEOYSFAVZKGJRIHWXUMC";

#[derive(PartialEq)]
enum Cmode {
    Encrypt,
    Decrypt,
}

fn chao(input: &str, output: &mut [char], mode: Cmode, show_steps: bool) {
    let len = input.len();
    let mut left: Vec<char> = L_ALPHABET.chars().collect();
    let mut right: Vec<char> = R_ALPHABET.chars().collect();
    let mut temp: Vec<char> = vec!['\0'; 26];

    for (i, c) in input.chars().enumerate() {
        if show_steps {
            println!("{}  {}", left.iter().collect::<String>(), right.iter().collect::<String>());
        }
        let index = if mode == Cmode::Encrypt {
            right.iter().position(|&x| x == c).unwrap()
        } else {
            left.iter().position(|&x| x == c).unwrap()
        };
        output[i] = if mode == Cmode::Encrypt { left[index] } else { right[index] };

        if i == len - 1 {
            break;
        }

        // Permute left
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

        // Permute right
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
}

fn main() {
    let plain_text = "WELLDONEISBETTERTHANWELLSAID";
    let mut cipher_text: Vec<char> = vec!['\0'; plain_text.len()];
    let mut plain_text2: Vec<char> = vec!['\0'; plain_text.len()];

    println!("The original plaintext is : {}", plain_text);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(plain_text, &mut cipher_text, Cmode::Encrypt, true);
    println!("\nThe ciphertext is : {}", cipher_text.iter().collect::<String>());
    chao(&cipher_text.iter().collect::<String>(), &mut plain_text2, Cmode::Decrypt, false);
    println!("\nThe recovered plaintext is : {}", plain_text2.iter().collect::<String>());
}