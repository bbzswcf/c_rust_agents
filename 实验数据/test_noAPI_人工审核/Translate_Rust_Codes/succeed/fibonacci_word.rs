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
    let p_ones = ones as f64 / total as f64;
    let p_zeros = zeros as f64 / total as f64;

    let result = -p_ones * p_ones.log2() - p_zeros * p_zeros.log2();

    if result.is_nan() {
        0.0
    } else {
        result
    }
}

fn print_entropy(word: &str) {
    let mut ones = 0;
    let mut zeros = 0;

    for c in word.chars() {
        match c {
            '0' => zeros += 1,
            '1' => ones += 1,
            _ => {}
        }
    }

    let entropy = calculate_entropy(ones, zeros);
    print!(" {:20.18}", entropy);
}

fn print_word(n: i32, word: &str) {
    print!("{:2}", n);
    print!(" {:10}", word.len());
    print_entropy(word);

    if n < 10 {
        print!(" {:40}", word);
    } else {
        print!(" {:40}", "...");
    }

    println!();
}

fn main() {
    print_headings();

    let mut last_word = String::from("1");
    let mut current_word = String::from("0");

    print_word(1, &last_word);

    for i in 2..=37 {
        print_word(i, &current_word);

        let next_word = format!("{}{}", current_word, last_word);
        last_word = mem::replace(&mut current_word, next_word);
    }
}