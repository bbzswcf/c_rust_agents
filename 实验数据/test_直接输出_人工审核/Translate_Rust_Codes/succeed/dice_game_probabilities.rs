fn ipow(x: u32, y: u32) -> u64 {
    let mut result: u64 = 1;
    for _ in 1..=y {
        result *= x as u64;
    }
    result
}

fn min(x: u32, y: u32) -> u32 {
    if x < y { x } else { y }
}

fn throw_die(n_sides: u32, n_dice: u32, s: u32, counts: &mut [u32]) {
    if n_dice == 0 {
        counts[s as usize] += 1;
        return;
    }

    for i in 1..=n_sides {
        throw_die(n_sides, n_dice - 1, s + i, counts);
    }
}

fn beating_probability(n_sides1: u32, n_dice1: u32, n_sides2: u32, n_dice2: u32) -> f64 {
    let len1 = (n_sides1 + 1) * n_dice1;
    let mut C1 = vec![0u32; len1 as usize];
    throw_die(n_sides1, n_dice1, 0, &mut C1);

    let len2 = (n_sides2 + 1) * n_dice2;
    let mut C2 = vec![0u32; len2 as usize];
    throw_die(n_sides2, n_dice2, 0, &mut C2);

    let p12 = ipow(n_sides1, n_dice1) * ipow(n_sides2, n_dice2);

    let mut tot = 0.0;
    for i in 0..len1 {
        for j in 0..min(i, len2) {
            tot += (C1[i as usize] * C2[j as usize]) as f64 / p12 as f64;
        }
    }
    tot
}

fn main() {
    println!("{:.16}", beating_probability(4, 9, 6, 6));
    println!("{:.16}", beating_probability(10, 5, 7, 6));
}