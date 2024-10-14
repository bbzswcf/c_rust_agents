use std::io;

fn proper_divisors(n: i32, print_flag: bool) -> i32 {
    let mut count = 0;

    for i in 1..n {
        if n % i == 0 {
            count += 1;
            if print_flag {
                print!("{} ", i);
            }
        }
    }

    if print_flag {
        println!();
    }

    count
}

fn count_proper_divisors(n: i32) -> i32 {
    let mut prod = 1;
    let mut count = 0;
    let mut n = n;

    while n % 2 == 0 {
        count += 1;
        n /= 2;
    }

    prod *= 1 + count;

    let mut i = 3;
    while i * i <= n {
        count = 0;

        while n % i == 0 {
            count += 1;
            n /= i;
        }

        prod *= 1 + count;
        i += 2;
    }

    if n > 2 {
        prod *= 2;
    }

    prod - 1
}

fn main() {
    for i in 1..=10 {
        print!("{}: ", i);
        proper_divisors(i, true);
    }

    let mut max = 0;
    let mut max_i = 1;

    for i in 1..=20000 {
        let v = count_proper_divisors(i);
        if v >= max {
            max = v;
            max_i = i;
        }
    }

    println!("{} with {} divisors", max_i, max);
}