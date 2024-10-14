fn is_pangram(s: &str) -> bool {
    let alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut wasused = [false; 26];
    let mut total = 0;

    for ch in s.chars() {
        if let Some(p) = alpha.find(ch) {
            let idx = p % 26;
            if !wasused[idx] {
                total += 1;
                wasused[idx] = true;
                if total == 26 {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    let tests = [
        "The quick brown fox jumps over the lazy dog.",
        "The qu1ck brown fox jumps over the lazy d0g."
    ];

    for test in tests.iter() {
        println!("\"{}\" is {}a pangram", test, if is_pangram(test) { "" } else { "not " });
    }
}