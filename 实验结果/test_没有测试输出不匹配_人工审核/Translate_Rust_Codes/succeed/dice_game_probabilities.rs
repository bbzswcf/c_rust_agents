use std::ops::{Add, AddAssign, Sub, Mul, MulAssign};

// Define a newtype wrapper for `u32` to avoid orphan rule violation
#[derive(Clone, Copy, Default)]
struct MyUint(u32);

// Implement the Add trait for MyUint
impl Add for MyUint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        MyUint(self.0 + other.0)
    }
}

// Implement the AddAssign trait for MyUint
impl AddAssign for MyUint {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

// Implement the Sub trait for MyUint
impl Sub for MyUint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        MyUint(self.0 - other.0)
    }
}

// Implement the Mul trait for MyUint
impl Mul for MyUint {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        MyUint(self.0 * other.0)
    }
}

// Implement the MulAssign trait for MyUint
impl MulAssign for MyUint {
    fn mul_assign(&mut self, other: Self) {
        self.0 *= other.0;
    }
}

type Uint = MyUint;
type Ulong = u64;

fn ipow(x: Uint, y: Uint) -> Ulong {
    let mut result: Ulong = 1;
    for _ in 1..=y.0 {
        result *= x.0 as Ulong;
    }
    result
}

fn min(x: Uint, y: Uint) -> Uint {
    if x.0 < y.0 { x } else { y }
}

fn throw_die(n_sides: Uint, n_dice: Uint, s: Uint, counts: &mut [Uint]) {
    if n_dice.0 == 0 {
        counts[s.0 as usize] += MyUint(1);
        return;
    }

    for i in 1..=n_sides.0 {
        throw_die(n_sides, MyUint(n_dice.0 - 1), MyUint(s.0 + i), counts);
    }
}

fn beating_probability(n_sides1: Uint, n_dice1: Uint, n_sides2: Uint, n_dice2: Uint) -> f64 {
    let len1 = (n_sides1.0 + 1) * n_dice1.0;
    let mut C1 = vec![Uint::default(); len1 as usize]; // Ensure `Uint` (alias for `MyUint`) implements `Default`
    throw_die(n_sides1, n_dice1, MyUint(0), &mut C1); // Ensure `throw_die` function is correctly defined

    let len2 = (n_sides2.0 + 1) * n_dice2.0;
    let mut C2 = vec![Uint::default(); len2 as usize]; // Ensure `Uint` (alias for `MyUint`) implements `Default`
    throw_die(n_sides2, n_dice2, MyUint(0), &mut C2); // Ensure `throw_die` function is correctly defined

    // Modified: Convert the result of `ipow` to `f64` before performing the multiplication
    let p12 = ipow(n_sides1, n_dice1) as f64 * ipow(n_sides2, n_dice2) as f64;

    let mut tot = 0.0;
    for i in 0..len1 { // Ensure `len1` and `len2` are of type `Uint` (alias for `MyUint`)
        for j in 0..min(MyUint(i), MyUint(len2)).0 {
            tot += (C1[i as usize].0 * C2[j as usize].0) as f64 / p12; // Ensure `C1` and `C2` are vectors of `Uint` (alias for `MyUint`)
        }
    }
    tot
}

fn main() {
    println!("{:.16}", beating_probability(MyUint(4), MyUint(9), MyUint(6), MyUint(6)));
    println!("{:.16}", beating_probability(MyUint(10), MyUint(5), MyUint(7), MyUint(6)));
}