use std::f64::NAN;
use std::mem;

fn print_headings() {
    print!("{:2}", "N");
    print!(" {:10}", "Length");
    print!(" {:20}", "Entropy");
    print!(" {:40}", "Word");
    println!();
}

fn calculate_entropy(ones: i32, zeros: i32) -> f64 {
    let total = ones + zeros;
    
    // Handle single-character words explicitly
    if total == 1 {
        return 0.0; // Modified: Corrected entropy for single-character words to 0.0
    }

    let ones_ratio = ones as f64 / total as f64;
    let zeros_ratio = zeros as f64 / total as f64;

    let result = -ones_ratio * ones_ratio.log2() - zeros_ratio * zeros_ratio.log2();

    if result.is_nan() {
        return 0.0; // Modified: Corrected NaN handling to return 0.0 instead of NAN
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
    print!(" {:10}", word.len());
    print_entropy(word);

    // Truncate based on word length, not index
    if word.len() > 40 {
        print!(" {:40}", &word[..40]); // Modified: Corrected truncation logic to handle words longer than 40 characters
    } else {
        print!(" {:40}", word);
    }

    println!();
}

fn main() {
    print_headings();

    // Correct initial words to match expected output
    let mut last_word = String::from("0"); // Modified: Swapped initial words
    let mut current_word = String::from("1"); // Modified: Swapped initial words

    print_word(1, &last_word);

    for i in 2..=37 {
        print_word(i, &current_word);

        // Correct concatenation logic
        let next_word = format!("{}{}", last_word, current_word); // Modified: Corrected concatenation logic
        last_word = mem::replace(&mut current_word, next_word);
    }
}