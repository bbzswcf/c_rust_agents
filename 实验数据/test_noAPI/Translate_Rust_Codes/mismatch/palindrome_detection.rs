// Removed: Unused import
// use std::io;

fn palindrome(s: &str) -> bool {
    let l = s.len();
    for i in 0..l / 2 {
        if s.chars().nth(i) != s.chars().nth(l - i - 1) {
            return false;
        }
    }
    true
}

fn palindrome_r(s: &str, b: usize, e: usize) -> bool {
    if (e - 1) <= b {
        return true;
    }
    if s.chars().nth(b) != s.chars().nth(e - 1) {
        return false;
    }
    palindrome_r(s, b + 1, e - 1)
}

fn main() {
    let t = "ingirumimusnocteetconsumimurigni";
    let l = t.len();

    // Modified: Replaced `template` variable with a string literal
    println!("sequence \"{}\" is{} palindrome\n", t, if palindrome(t) { "" } else { "n't" });
    // Modified: Replaced `template` variable with a string literal
    println!("sequence \"{}\" is{} palindrome\n", t, if palindrome_r(t, 0, l) { "" } else { "n't" });
}