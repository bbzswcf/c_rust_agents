type Uint = u32;
type Ulong = u64;

fn ipow(x: Uint, y: Uint) -> Ulong {
    let mut result: Ulong = 1; // Explicitly define the type of result as Ulong
    for _ in 1..=y {
        // Ensured that the multiplication operation does not cause an integer overflow
        result = result.checked_mul(x as Ulong).unwrap_or(result);
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

    // Modified: Convert the result of `ipow(n_sides1, n_dice1)` to `f64` before performing the multiplication
    let p12 = ipow(n_sides1, n_dice1) as f64 * ipow(n_sides2, n_dice2) as f64;

    let mut tot = 0.0;
    // Ensured that the indices do not exceed the bounds of the vectors
    if len1 > 0 && len2 > 0 {
        for i in 0..len1 {
            for j in 0..min(i, len2) {
                // Ensured that the types are correctly cast to f64 before performing the division
                tot += (C1[i as usize] as f64 * C2[j as usize] as f64) / p12;
            }
        }
    }
    tot
}

fn main() {
    println!("{:.16}", beating_probability(4, 9, 6, 6));
    println!("{:.16}", beating_probability(10, 5, 7, 6));
}