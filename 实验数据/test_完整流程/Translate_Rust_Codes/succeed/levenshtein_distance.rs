fn levenshtein(s: &str, t: &str) -> usize {
    let ls = s.len();
    let lt = t.len();

    // Create a 2D vector to store intermediate results
    let mut dp = vec![vec![0; lt + 1]; ls + 1];

    // Initialize the base cases
    for i in 0..=ls {
        dp[i][0] = i;
    }
    for j in 0..=lt {
        dp[0][j] = j;
    }

    // Fill the dp table iteratively
    for i in 1..=ls {
        for j in 1..=lt {
            let cost = if s.chars().nth(i - 1) == t.chars().nth(j - 1) {
                0
            } else {
                1
            };
            dp[i][j] = (dp[i - 1][j] + 1) // Deletion
                .min(dp[i][j - 1] + 1)    // Insertion
                .min(dp[i - 1][j - 1] + cost); // Substitution
        }
    }

    // The result is the value in the bottom-right corner of the table
    dp[ls][lt]
}

fn main() {
    let s1 = "rosettacode";
    let s2 = "raisethysword";
    println!("distance between `{}' and `{}': {}", s1, s2, levenshtein(s1, s2));
}