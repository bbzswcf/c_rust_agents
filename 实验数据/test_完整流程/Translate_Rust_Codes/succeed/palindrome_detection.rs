fn palindrome(chars: &[char]) -> bool {
    let l = chars.len();
    for i in 0..l / 2 {
        if chars[i] != chars[l - i - 1] {
            return false;
        }
    }
    true
}

fn palindrome_r(chars: &[char], b: usize, e: usize) -> bool {
    if b >= e - 1 { // Modified: Corrected the base case condition
        return true;
    }
    if chars[b] != chars[e - 1] { // Modified: Use indexing to compare characters
        return false;
    }
    palindrome_r(chars, b + 1, e - 1)
}

fn main() {
    let t = "ingirumimusnocteetconsumimurigni";
    let chars: Vec<char> = t.chars().collect(); // Modified: Convert string to a vector of characters once
    let l = chars.len();

    // Modified: Combined redundant print statements
    println!(
        "sequence \"{}\" is{} palindrome\nsequence \"{}\" is{} palindrome\n",
        t,
        if palindrome(&chars) { "" } else { "n't" },
        t,
        if palindrome_r(&chars, 0, l) { "" } else { "n't" }
    );
}