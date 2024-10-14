fn levenshtein(s: &str, ls: usize, t: &str, lt: usize) -> i32 {
    if ls == 0 {
        return lt as i32;
    }
    if lt == 0 {
        return ls as i32;
    }

    if s.chars().nth(ls - 1) == t.chars().nth(lt - 1) {
        return levenshtein(s, ls - 1, t, lt - 1);
    }

    let a = levenshtein(s, ls - 1, t, lt - 1);
    let b = levenshtein(s, ls, t, lt - 1);
    let c = levenshtein(s, ls - 1, t, lt);

    let min = a.min(b).min(c);

    min + 1
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    let distance = levenshtein(s1, s1.len(), s2, s2.len());
    println!("distance between `{}' and `{}': {}", s1, s2, distance);
}