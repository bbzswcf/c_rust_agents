fn print_headings() {
    print!("{:2}", "N");
    print!(" {:10}", "Length");
    print!(" {:20}", "Entropy");
    print!(" {:40}", "Word");
    println!(); // 确保换行
}

fn calculate_entropy(ones: i32, zeros: i32) -> f64 {
    let total = ones + zeros;
    let ones_ratio = ones as f64 / total as f64;
    let zeros_ratio = zeros as f64 / total as f64;

    let mut result = 0.0;
    // 修改: 使用 ones_ratio.log2() 计算熵
    if ones_ratio > 0.0 {
        result -= ones_ratio * ones_ratio.log2();
    }
    // 修改: 使用 zeros_ratio.log2() 计算熵
    if zeros_ratio > 0.0 {
        result -= zeros_ratio * zeros_ratio.log2();
    }

    if result.is_nan() {
        result = 0.0;
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

    // 修改: 当 n >= 10 时, 打印 "..." 而不是完整的单词
    if n < 10 {
        print!(" {:40}", word);
    } else {
        print!(" {:40}", "...");
    }

    println!(); // 确保换行
}

fn main() {
    print_headings();

    let mut last_word = String::from("1");
    let mut current_word = String::from("0");

    print_word(1, &last_word);

    for i in 2..=37 {
        print_word(i, &current_word);

        // 修改: 使用 format! 生成下一个单词
        let next_word = format!("{}{}", current_word, last_word);

        last_word = current_word;
        current_word = next_word;
    }
}