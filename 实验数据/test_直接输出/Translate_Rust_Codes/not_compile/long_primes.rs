use std::alloc::{alloc, Layout};
use std::mem;

const TRUE: bool = true;
const FALSE: bool = false;

fn sieve(limit: usize, primes: &mut Vec<i32>, count: &mut usize) {
    let mut c = vec![FALSE; limit + 1]; // composite = TRUE
    let mut p = 3;
    let mut p2 = p * p;
    while p2 <= limit {
        for i in (p2..=limit).step_by(2 * p) {
            c[i] = TRUE;
        }
        p += 2;
        while c[p] {
            p += 2;
        }
        p2 = p * p;
    }
    for i in (3..=limit).step_by(2) {
        if !c[i] {
            primes.push(i as i32);
        }
    }
    *count = primes.len();
}

fn find_period(n: i32) -> i32 {
    let mut r = 1;
    let mut period = 0;
    for _ in 1..=n + 1 {
        r = (10 * r) % n;
    }
    let rr = r;
    loop {
        r = (10 * r) % n;
        period += 1;
        if r == rr {
            break;
        }
    }
    period
}

fn main() {
    let numbers = vec![500, 1000, 2000, 4000, 8000, 16000, 32000];
    let number_count = numbers.len();

    let mut primes = Vec::with_capacity(6500);
    let mut prime_count = 0;
    sieve(32000, &mut primes, &mut prime_count);

    let mut long_primes = Vec::with_capacity(prime_count);
    let mut long_count = 0;

    for &prime in &primes {
        if find_period(prime) == prime - 1 {
            long_primes.push(prime);
            long_count += 1;
        }
    }

    let mut totals = vec![0; number_count];
    let mut count = 0;
    let mut index = 0;

    for i in 0..long_count {
        if long_primes[i] > numbers[index] {
            totals[index] = count;
            index += 1;
        }
        count += 1;
    }
    totals[number_count - 1] = count;

    println!("The long primes up to {} are:", numbers[0]);
    print!("[");
    for i in 0..totals[0] {
        print!("{} ", long_primes[i]);
    }
    println!("\b]");

    println!("\nThe number of long primes up to:");
    for i in 0..7 {
        println!("  {:5} is {}", numbers[i], totals[i]);
    }
}