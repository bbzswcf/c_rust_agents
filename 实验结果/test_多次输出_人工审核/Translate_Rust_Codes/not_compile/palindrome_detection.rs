fn is_palindrome(s: &str) -> bool {
    let len = s.len();
    for i in 0..len / 2 {
        if s.chars().nth(i) != s.chars().nth(len - i - 1) {
            return false;
        }
    }
    true
}

fn is_palindrome_recursive(s: &str, b: usize, e: usize) -> bool {
    if (e - 1) <= b {
        return true;
    }
    if s.chars().nth(b) != s.chars().nth(e - 1) {
        return false;
    }
    is_palindrome_recursive(s, b + 1, e - 1)
}

fn main() {
    let t = "ingirumimusnocteetconsumimurigni";
    let template = "sequence \"{}\" is{} palindrome\n";
    let l = t.len();

    println!(
        template,
        t,
        if is_palindrome(t) { "" } else { "n't" }
    );
    println!(
        template,
        t,
        if is_palindrome_recursive(t, 0, l) { "" } else { "n't" }
    );
}