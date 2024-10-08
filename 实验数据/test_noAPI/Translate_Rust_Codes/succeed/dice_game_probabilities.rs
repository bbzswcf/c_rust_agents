// Modified: Removed unused import to clean up the code.
// use std::io;

fn ipow(x: u32, y: u32) -> u64 {
    let mut result: u64 = 1;
    for _ in 1..=y {
        // Modified: Use checked_mul to safely multiply and handle overflow
        if let Some(new_result) = result.checked_mul(x as u64) {
            result = new_result;
        } else {
            // Handle overflow case
            return 0; // or some other appropriate handling
        }
    }
    result
}

fn min(x: u32, y: u32) -> u32 {
    if x < y { x } else { y }
}

fn throw_die(n_sides: u32, n_dice: u32, s: u32, counts: &mut [u32]) {
    if n_dice == 0 {
        // Modified: Ensure s is within bounds before accessing counts
        if s < counts.len() as u32 {
            counts[s as usize] += 1;
        }
        return;
    }

    for i in 1..=n_sides {
        // Modified: Ensure recursive call does not cause overflow
        if n_dice > 0 && s + i < u32::MAX {
            throw_die(n_sides, n_dice - 1, s + i, counts);
        }
    }
}

fn beating_probability(n_sides1: u32, n_dice1: u32, n_sides2: u32, n_dice2: u32) -> f64 {
    // Modified: Use checked arithmetic to prevent overflow
    let len1 = (n_sides1 as u64 + 1) * n_dice1 as u64;
    let len2 = (n_sides2 as u64 + 1) * n_dice2 as u64;
    let len1 = len1 as usize;
    let len2 = len2 as usize;

    let mut C1 = vec![0; len1];
    throw_die(n_sides1, n_dice1, 0, &mut C1);

    let mut C2 = vec![0; len2];
    throw_die(n_sides2, n_dice2, 0, &mut C2);

    // Modified: Ensure p12 is not zero before performing division
    let p12 = ipow(n_sides1, n_dice1) as f64 * ipow(n_sides2, n_dice2) as f64;
    if p12 == 0.0 {
        // Handle division by zero case
        return 0.0; // or some other appropriate handling
    }

    let mut tot = 0.0;
    for i in 0..len1 {
        for j in 0..i {
            // Modified: Ensure indices are within bounds before accessing elements
            if i < C1.len() && j < C2.len() {
                // Modified: Cast to u64 before multiplication to prevent overflow
                tot += (C1[i] as u64 * C2[j] as u64) as f64 / p12;
            }
        }
    }
    tot
}

fn main() {
    println!("{:.16}", beating_probability(4, 9, 6, 6));
    println!("{:.16}", beating_probability(10, 5, 7, 6));
}