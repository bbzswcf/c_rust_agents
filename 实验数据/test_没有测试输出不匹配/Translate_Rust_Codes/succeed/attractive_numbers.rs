const TRUE: bool = true;
const FALSE: bool = false;
const MAX: i32 = 120;

fn is_prime(n: i32) -> bool {
    let mut d = 5;
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
        d += 2;
        if n % d == 0 {
            return FALSE;
        }
        d += 4;
    }
    return TRUE;
}

fn count_prime_factors(n: i32) -> i32 {
    let count = 0; // Modified: Declare `count` as immutable
    let f = 2; // Modified: Declare `f` as immutable
    if n == 1 {
        return 0;
    }
    if is_prime(n) {
        return 1;
    }
    let mut n_mutable = n; // Added: Declare `n_mutable` as mutable
    let mut f_mutable = f; // Added: Declare `f_mutable` as mutable
    let mut count_mutable = count; // Added: Declare `count_mutable` as mutable
    loop {
        if n_mutable % f_mutable == 0 {
            count_mutable += 1;
            n_mutable /= f_mutable; // Modified: Use `n_mutable` instead of `n`
            if n_mutable == 1 {
                return count_mutable;
            }
            if is_prime(n_mutable) {
                f_mutable = n_mutable; // Modified: Use `f_mutable` instead of `f`
            }
        } else if f_mutable >= 3 {
            f_mutable += 2;
        } else {
            f_mutable = 3;
        }
    }
}

fn main() {
    let count = 0; // Modified: Declare `count` as immutable
    println!("The attractive numbers up to and including {} are:", MAX);
    let mut count_mutable = count; // Added: Declare `count_mutable` as mutable
    for i in 1..=MAX {
        let n = count_prime_factors(i); // Modified: Declare `n` as immutable
        if is_prime(n) {
            print!("{:4}", i);
            count_mutable += 1; // Modified: Use `count_mutable` instead of `count`
            if count_mutable % 20 == 0 {
                println!();
            }
        }
    }
    println!();
}