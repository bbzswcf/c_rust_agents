use std::f64;

fn print_headings() {
    println!("{:2} {:10} {:20} {:40}", "N", "Length", "Entropy", "Word");
}

fn calculate_entropy(ones: i32, zeros: i32) -> f64 {
    let total = ones + zeros;
    let p_ones = ones as f64 / total as f64;
    let p_zeros = zeros as f64 / total as f64;

    if ones == 0 || zeros == 0 {
        0.0
    } else {
        -p_ones * p_ones.log2() - p_zeros * p_zeros.log2()
    }
}

fn print_entropy(word: &str) {
    let mut ones = 0;
    let mut zeros = 0;

    for c in word.chars() {
        match c {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => (),
        }
    }

    let entropy = calculate_entropy(ones, zeros);
    print!("{:20.18}", entropy);
}

fn print_word(n: i32, word: &str) {
    print!("{:2}", n);

    // Modified: Corrected length calculation to use actual word length
    print!(" {:10}", word.len());

    print_entropy(word);

    // Modified: Ensure the word is truncated correctly if it exceeds 40 characters
    if word.len() > 40 {
        print!(" {:40}", &word[..40]);
    } else {
        print!(" {:40}", word);
    }

    println!();
}

fn main() {
    print_headings();

    // Modified: Initialize last_word to "1" and current_word to "0"
    let mut last_word = String::from("1");
    let mut current_word = String::from("0");

    // Print the initial word for N=1 as "1"
    print_word(1, &last_word);

    // Modified: Concatenate current_word to last_word and then swap them
    for i in 2..=37 {
        print_word(i, &last_word);

        let mut next_word = last_word.clone();
        next_word.push_str(&current_word);

        current_word = last_word;
        last_word = next_word;
    }
}