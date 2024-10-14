use std::collections::HashMap;

fn levenshtein(s: &str, t: &str, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let ls = s.len();
    let lt = t.len();

    // Base cases
    if ls == 0 {
        return lt;
    }
    if lt == 0 {
        return ls;
    }

    // Check if the result is already memoized
    if let Some(&result) = memo.get(&(ls, lt)) {
        return result;
    }

    // Direct character comparison
    let cost = if s.chars().last() == t.chars().last() { 0 } else { 1 };

    // Recursive calls with memoization
    let a = levenshtein(&s[..ls - 1], t, memo) + 1;
    let b = levenshtein(s, &t[..lt - 1], memo) + 1;
    let c = levenshtein(&s[..ls - 1], &t[..lt - 1], memo) + cost;

    // Use min function to find the minimum of the three values and return it
    let result = a.min(b).min(c);

    // Memoize the result
    memo.insert((ls, lt), result);

    result
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    let mut memo = HashMap::new();
    println!("distance between `{}' and `{}': {}", s1, s2, levenshtein(s1, s2, &mut memo));
}