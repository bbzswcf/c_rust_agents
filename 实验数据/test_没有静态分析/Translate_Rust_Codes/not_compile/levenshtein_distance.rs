fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> i32 {
    // Initialize variables a, b, and c to 0
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    // Handle the Option returned by nth by using unwrap
    if s.chars().nth(ls - 1).unwrap() == t.chars().nth(lt - 1).unwrap() {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    a = levenshtein(s, ls - 1, t, lt - 1);
    b = levenshtein(s, ls, t, lt - 1);
    c = levenshtein(s, ls - 1, t, lt);

    if a > b {
        a = b; // Modified: Ensure `a` is mutable before reassigning
    }
    if a > c {
        a = c; // Modified: Ensure `a` is mutable before reassigning
    }

    a + 1
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    // Modified: Ensure the lengths are correctly passed as `usize` to the `levenshtein` function
    println!("distance between `{}' and `{}': {}", s1, s2, levenshtein(s1, s1.len(), s2, s2.len()));
}