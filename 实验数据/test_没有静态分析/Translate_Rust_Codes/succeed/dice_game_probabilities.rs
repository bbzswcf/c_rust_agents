// Removed: Unused import
// use std::io;

type Uint = u32;
type Ulong = u64;

fn ipow(x: Uint, y: Uint) -> Ulong {
    let mut result: Ulong = 1;
    for _ in 1..=y {
        result *= x as Ulong;
    }
    result
}

fn min(x: Uint, y: Uint) -> Uint {
    if x < y { x } else { y }
}

fn throw_die(n_sides: Uint, n_dice: Uint, s: Uint, counts: &mut [Uint]) {
    if n_dice == 0 {
        counts[s as usize] += 1;
        return;
    }

    for i in 1..=n_sides {
        throw_die(n_sides, n_dice - 1, s + i, counts);
    }
}

fn beating_probability(n_sides1: Uint, n_dice1: Uint, n_sides2: Uint, n_dice2: Uint) -> f64 {
    let len1 = (n_sides1 + 1) * n_dice1;
    let mut C1 = vec![0; len1 as usize];
    throw_die(n_sides1, n_dice1, 0, &mut C1);

    let len2 = (n_sides2 + 1) * n_dice2;
    let mut C2 = vec![0; len2 as usize];
    throw_die(n_sides2, n_dice2, 0, &mut C2);

    // Modified: Cast both results of ipow to f64 before multiplication
    let p12 = ipow(n_sides1, n_dice1) as f64 * ipow(n_sides2, n_dice2) as f64;

    let mut tot = 0.0;
    for i in 0..len1 {
        for j in 0..min(i, len2) {
            tot += (C1[i as usize] * C2[j as usize]) as f64 / p12;
        }
    }
    tot
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:.16}", beating_probability(4, 9, 6, 6));
    println!("{:.16}", beating_probability(10, 5, 7, 6));
    Ok(())
}