use std::convert::TryInto; // Modified: Import the `TryInto` trait to bring the `try_into` method into scope

fn sieve(limit: usize, primes: &mut [i32], count: &mut usize) {
    let mut c = vec![false; limit + 1];
    let mut p = 3;
    let mut p2 = p * p;
    while p2 <= limit {
        for i in (p2..=limit).step_by(2 * p) {
            c[i] = true;
        }
        p += 2;
        while c[p] {
            p += 2;
        }
        p2 = p * p;
    }
    let mut n = 0;
    for i in (3..=limit).step_by(2) {
        if !c[i] {
            primes[n] = i as i32;
            n += 1;
        }
    }
    *count = n;
}

fn find_period(n: i32) -> i32 {
    // Modified: Use the `into` method for safe conversion from `i32` to `i64`
    let n: i64 = n.into();
    let mut r: i64 = 1;
    let mut period = 0;
    for _ in 1..=n + 1 {
        r = (10 * r).checked_rem(n).unwrap_or(r);
    }
    let rr = r;
    loop {
        r = (10 * r).checked_rem(n).unwrap_or(r);
        period += 1;
        if r == rr {
            break;
        }
    }
    period
}

fn main() {
    let numbers = [500, 1000, 2000, 4000, 8000, 16000, 32000];
    let number_count = numbers.len();

    let mut primes = vec![0; 6500];
    let mut prime_count = 0;
    sieve(32000, &mut primes, &mut prime_count);

    // Modified: Initialize the `long_primes` vector with the correct capacity to avoid reallocations
    let mut long_primes = Vec::with_capacity(prime_count);
    let mut long_count = 0;

    for i in 0..prime_count {
        let prime = primes[i];
        if find_period(prime) == prime - 1 {
            long_primes.push(prime);
            long_count += 1;
        }
    }

    // Modified: Initialize the `totals` vector with the correct capacity to avoid reallocations
    let mut totals = Vec::with_capacity(number_count);
    let mut count = 0;
    let mut index = 0;

    for i in 0..long_count {
        // Modified: Ensure that `index` does not exceed the bounds of the `numbers` array
        if index < number_count && long_primes[i] > numbers[index] {
            // Modified: Ensure that `index` does not exceed the bounds of the `totals` vector
            if index < totals.len() {
                totals.push(count);
            }
            index += 1;
        }
        count += 1;
    }
    // Modified: The check for `number_count > 0` is redundant since `number_count` is always greater than zero
    totals.push(count);

    println!("The long primes up to {} are:", numbers[0]);
    print!("[");
    for i in 0..totals[0] {
        print!("{} ", long_primes[i]);
    }
    // Modified: Replace `\x08` with a space to ensure the output is correctly formatted
    println!(" ]");

    println!("\nThe number of long primes up to:");
    for i in 0..number_count {
        println!("{:5} is {}", numbers[i], totals[i]);
    }
}