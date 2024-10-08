// Removed: Unused import
// use std::f64::NAN;

fn print_headings() {
    print!("{:2}", "N");
    print!(" {:10}", "Length");
    print!(" {:20}", "Entropy");
    print!(" {:40}", "Word");
    println!();
}

fn calculate_entropy(ones: i32, zeros: i32) -> f64 {
    let total = ones + zeros;
    let p_ones = ones as f64 / total as f64;
    let p_zeros = zeros as f64 / total as f64;

    let result = -p_ones * p_ones.log2() - p_zeros * p_zeros.log2();

    // Handle cases where the word length is too small to produce meaningful entropy
    // Modified: Ensure the entropy calculation correctly handles small word lengths
    if result.is_nan() || total < 2 {
        return 0.0;
    }

    result
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
    print!(" {:20.18}", entropy);
}

fn print_word(n: i32, word: &str) {
    print!("{:2}", n);
    // Modified: Use the length of the word for the word length calculation
    print!(" {:10}", word.len());
    // No additional type annotation needed for `print_entropy` function call
    print_entropy(word);

    // Correctly handle the word display for larger n
    // Modified: Adjust the condition to correctly handle the word display for larger `n`
    if n < 10 || word.len() <= 40 {
        // No additional type annotation needed for `print!` macro
        print!(" {:40}", word);
    } else {
        print!(" {:40}", "...");
    }

    println!();
}

fn main() {
    print_headings();

    // Ensure the types of last_word and current_word are explicitly String
    // Modified: Correct initial word values to match the expected output
    let mut last_word = String::from("0");
    let mut current_word = String::from("1");

    // No additional type annotation needed for `print_word` function call
    print_word(1, &last_word);

    for i in 2..=37 {
        // No additional type annotation needed for `print_word` function call
        print_word(i, &current_word);

        // Ensure the type of next_word is explicitly String
        let mut next_word = current_word.clone();
        // Modified: Modify the word concatenation logic to correctly append the `last_word` to the `current_word`
        next_word.push_str(&last_word);

        last_word = current_word;
        current_word = next_word;
    }
}