fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> i32 {
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;

    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    if s.chars().nth(ls - 1) == t.chars().nth(lt - 1) {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    a = levenshtein(s, ls - 1, t, lt - 1);
    b = levenshtein(s, ls, t, lt - 1);
    c = levenshtein(s, ls - 1, t, lt);

    if a > b {
        a = b;
    }
    if a > c {
        a = c;
    }

    a + 1
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    println!(
        "distance between `{}' and `{}': {}",
        s1,
        s2,
        levenshtein(s1, s1.len(), s2, s2.len())
    );
}