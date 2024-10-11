use std::io;

const TRUE: bool = true;
const FALSE: bool = false;
const MAX: i32 = 120;

fn is_prime(n: i32) -> bool {
    let mut d = 5; // Modified: Declared `d` as mutable
    if n < 2 {
        return FALSE;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    while d * d <= n {
        if n % d == 0 {
            return FALSE;
        }
        d += 2; // Modified: Reassignment to mutable `d`
        if n % d == 0 {
            return FALSE;
        }
        d += 4; // Modified: Reassignment to mutable `d`
    }
    return TRUE;
}

fn count_prime_factors(n: i32) -> i32 {
    let mut count = 0; // Modified: Declared `count` as mutable
    let mut f = 2; // Modified: Declared `f` as mutable
    if n == 1 {
        return 0;
    }
    if is_prime(n) {
        return 1;
    }
    loop {
        if n % f == 0 {
            count += 1; // Modified: Reassignment to mutable `count`
            n /= f; // Modified: Reassignment to mutable `n`
            if n == 1 {
                return count;
            }
            if is_prime(n) {
                f = n;
            }
        } else if f >= 3 {
            f += 2; // Modified: Reassignment to mutable `f`
        } else {
            f = 3;
        }
    }
}

fn main() {
    println!("The attractive numbers up to and including {} are:", MAX);
    let mut count = 0; // Modified: Declared `count` as mutable
    for i in 1..=MAX {
        let mut n = count_prime_factors(i); // Modified: Declared `n` as mutable
        if is_prime(n) {
            print!("{:4}", i);
            count += 1; // Modified: Reassignment to mutable `count`
            if count % 20 == 0 {
                println!();
            }
        }
    }
    println!();
}