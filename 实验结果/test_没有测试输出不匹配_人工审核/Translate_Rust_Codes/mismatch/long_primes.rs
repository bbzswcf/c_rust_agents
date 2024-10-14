// Removed unused import
// use std::alloc;

const TRUE: bool = true;
const FALSE: bool = false;

fn sieve(limit: usize, primes: &mut Vec<i32>) -> usize {
    let mut c = vec![FALSE; limit + 1]; // composite = TRUE
    let mut i: usize; // Added type annotation
    let mut p = 3;
    let mut p2;
    let mut n = 0;
    p2 = p * p;
    while p2 <= limit {
        i = p2;
        while i <= limit {
            c[i] = TRUE;
            i += 2 * p;
        }
        p += 2;
        while c[p] {
            p += 2;
        }
        p2 = p * p;
    }
    for i in (3..=limit).step_by(2) {
        if !c[i] {
            primes[n] = i as i32;
            n += 1;
        }
    }
    n
}

fn find_period(n: i32) -> i32 {
    let mut i: i32; // Added type annotation
    let mut r = 1;
    let mut rr;
    let mut period = 0;
    for i in 1..=n + 1 {
        r = (10 * r) % n;
    }
    rr = r;
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
    let mut i: usize; // Added type annotation
    let mut prime;
    let mut count = 0;
    let mut index = 0;
    let mut prime_count;
    let mut long_count = 0;
    let mut number_count;
    let mut primes = vec![0; 6500];
    let numbers = [500, 1000, 2000, 4000, 8000, 16000, 32000];

    number_count = numbers.len();
    let mut totals = vec![0; number_count];
    prime_count = sieve(32000, &mut primes);
    let mut long_primes = vec![0; prime_count];

    for i in 0..prime_count {
        prime = primes[i];
        if find_period(prime) == prime - 1 {
            long_primes[long_count] = prime;
            long_count += 1;
        }
    }
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
    println!(r"\b]"); // Modified to use raw string literal

    println!("\nThe number of long primes up to:");
    for i in 0..7 {
        println!("  {:5} is {}", numbers[i], totals[i]);
    }
}