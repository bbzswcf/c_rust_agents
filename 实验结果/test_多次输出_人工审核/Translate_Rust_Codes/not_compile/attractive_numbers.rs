const MAX: i32 = 120;

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut d = 5;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
        if n % d == 0 {
            return false;
        }
        d += 4;
    }
    true
}

fn count_prime_factors(n: i32) -> i32 {
    if n == 1 {
        return 0;
    }
    if is_prime(n) {
        return 1;
    }
    let mut count = 0;
    let mut f = 2;
    loop {
        if n % f == 0 {
            count += 1;
            n /= f;
            if n == 1 {
                return count;
            }
            if is_prime(n) {
                f = n;
            }
        } else if f >= 3 {
            f += 2;
        } else {
            f = 3;
        }
    }
}

fn main() {
    println!("The attractive numbers up to and including {} are:", MAX);
    let mut count = 0;
    for i in 1..=MAX {
        let n = count_prime_factors(i);
        if is_prime(n) {
            print!("{:4}", i);
            count += 1;
            if count % 20 == 0 {
                println!();
            }
        }
    }
    println!();
}