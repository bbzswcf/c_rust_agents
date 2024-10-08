fn chao(input: &str, output: &mut String, mode: cmode, show_steps: bool) {
    let l_alphabet = "HXUCZVAMDSLKPEFJRIGTWOBNYQ";
    let r_alphabet = "PTLNBQDEOYSFAVZKGJRIHWXUMC";
    let len = input.len();
    let mut left: Vec<char> = l_alphabet.chars().collect();
    let mut right: Vec<char> = r_alphabet.chars().collect();
    let mut temp: Vec<char> = vec!['\0'; 26];

    for (i, c) in input.chars().enumerate() {
        if show_steps {
            println!("{}  {}", left.iter().collect::<String>(), right.iter().collect::<String>());
        }
        // 修改: 根据mode选择正确的索引
        let index = match mode {
            cmode::ENCRYPT => right.iter().position(|&x| x == c).unwrap(),
            cmode::DECRYPT => left.iter().position(|&x| x == c).unwrap(),
        };
        // 修改: 根据mode选择正确的输出字符
        output.push(match mode {
            cmode::ENCRYPT => left[index],
            cmode::DECRYPT => right[index],
        });
        if i == len - 1 {
            break;
        }

        // permute left
        temp[..26 - index].copy_from_slice(&left[index..]);
        temp[26 - index..].copy_from_slice(&left[..index]);
        let store = temp[1];
        for j in 2..14 {
            temp[j - 1] = temp[j];
        }
        temp[13] = store;
        left.copy_from_slice(&temp);

        // permute right
        temp[..26 - index].copy_from_slice(&right[index..]);
        temp[26 - index..].copy_from_slice(&right[..index]);
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
    let mut cipher_text = String::with_capacity(plain_text.len() + 1);
    let mut plain_text2 = String::with_capacity(plain_text.len() + 1);
    println!("The original plaintext is : {}", plain_text);
    println!("\nThe left and right alphabets after each permutation during encryption are :\n");
    chao(plain_text, &mut cipher_text, cmode::ENCRYPT, true);
    println!("\nThe ciphertext is : {}", cipher_text);
    chao(&cipher_text, &mut plain_text2, cmode::DECRYPT, false);
    println!("\nThe recovered plaintext is : {}", plain_text2);
}

#[derive(PartialEq)]
enum cmode {
    ENCRYPT,
    DECRYPT,
}