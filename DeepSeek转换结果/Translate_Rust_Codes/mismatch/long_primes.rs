use std::alloc::{alloc, Layout};
use std::mem;

fn sieve(limit: i32, primes: &mut Vec<i32>, count: &mut i32) {
    // :  Vec  unsafe 
    let mut c = vec![false; limit as usize + 1];
    let mut i: i32;
    let mut p = 3;
    let mut p2;
    let mut n = 0;
    p2 = p * p;
    while p2 <= limit {
        i = p2;
        while i <= limit {
            c[i as usize] = true;
            i += 2 * p;
        }
        p += 2;
        while c[p as usize] {
            p += 2;
        }
        p2 = p * p;
    }
    i = 3;
    while i <= limit {
        if !c[i as usize] {
            primes[n as usize] = i;
            n += 1;
        }
        i += 2;
    }
    *count = n;
}

fn find_period(n: i32) -> i32 {
    let mut i = 1;
    let mut r = 1;
    let mut period = 0;
    while i <= n + 1 {
        r = (10 * r) % n;
        i += 1;
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
    let mut count = 0;
    let mut index = 0;
    let mut prime_count = 0;
    let mut long_count = 0;
    let mut number_count = 0;
    let numbers = vec![500, 1000, 2000, 4000, 8000, 16000, 32000];

    let mut primes = vec![0; 6500];
    number_count = numbers.len();
    let mut totals = vec![0; number_count];
    sieve(32000, &mut primes, &mut prime_count);
    let mut long_primes = vec![0; prime_count as usize];

    for i in 0..prime_count {
        let prime = primes[i as usize];
        if find_period(prime) == prime - 1 {
            long_primes[long_count as usize] = prime;
            long_count += 1;
        }
    }

    for i in 0..long_count {
        if long_primes[i as usize] > numbers[index] {
            totals[index] = count;
            index += 1;
        }
        count += 1;
    }
    totals[number_count - 1] = count;

    // : C
    print!("The long primes up to 500 are:\n[");
    for i in 0..totals[0] {
        if i > 0 {
            print!(" ");
        }
        print!("{}", long_primes[i as usize]);
    }
    println!(" ]\n"); // 

    println!("The number of long primes up to:");
    for i in 0..7 {
        println!("    {:4} is {}", numbers[i], totals[i]); // {:4}
    }
}