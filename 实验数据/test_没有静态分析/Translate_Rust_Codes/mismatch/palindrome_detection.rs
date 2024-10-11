fn palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let l = chars.len();
    for i in 0..l / 2 {
        if chars[i] != chars[l - i - 1] {
            return false;
        }
    }
    true
}

fn palindrome_r(chars: &[char], b: usize, e: usize) -> bool {
    if e <= b {
        return true;
    }
    if chars[b] != chars[e - 1] {
        return false;
    }
    palindrome_r(chars, b + 1, e - 1)
}

fn main() {
    let t = "ingirumimusnocteetconsumimurigni";
    let chars: Vec<char> = t.chars().collect();
    let l = chars.len();

    println!("sequence \"{}\" is{} palindrome", t, if palindrome(&t) { "" } else { " not" });
    println!();
    println!("sequence \"{}\" is{} palindrome", t, if palindrome_r(&chars, 0, l) { "" } else { " not" });
    println!();
}