use std::sync::{Mutex, OnceLock};
use once_cell::sync::Lazy;

// Derive Copy and Clone traits for the Transition struct
#[derive(Clone, Copy)]
struct Transition {
    a: u8,
    b: u8,
    c: u32,
}

const TRANSITIONS_SIZE: usize = 100;

// Use OnceLock to lazily initialize the static variable
static TRANSITIONS: Lazy<Mutex<[Transition; TRANSITIONS_SIZE]>> = Lazy::new(|| Mutex::new([Transition { a: 0, b: 0, c: 0 }; TRANSITIONS_SIZE]));

fn record(prev: i32, curr: i32) {
    let pd = (prev % 10) as u8;
    let cd = (curr % 10) as u8;

    let mut transitions = TRANSITIONS.lock().expect("Failed to lock transitions");
    for i in 0..100 {
        if transitions[i].a == pd && transitions[i].b == cd {
            transitions[i].c += 1;
            return;
        }
    }
    // Removed eprintln! for debugging
}

fn print_transitions(limit: i32, last_prime: i32) {
    println!("{} primes, last prime considered: {}", limit, last_prime);

    let transitions = TRANSITIONS.lock().expect("Failed to lock transitions");
    for i in 0..100 {
        if transitions[i].c > 0 {
            // Ensure limit is not zero to avoid division by zero
            if limit != 0 {
                println!("{}->{}  count: {:5}  frequency: {:.2}", transitions[i].a, transitions[i].b, transitions[i].c, 100.0 * transitions[i].c as f64 / limit as f64);
            } else {
                println!("{}->{}  count: {:5}  frequency: N/A", transitions[i].a, transitions[i].b, transitions[i].c);
            }
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut i = 5;
    // Ensure i * i does not overflow by using u64
    while i as u64 * i as u64 <= n as u64 {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

const LIMIT: i32 = 100000;

fn main() {
    let mut last_prime = 3;
    let mut n = 5;
    let mut count = 2;

    // Initialize all transitions with default values
    let mut transitions = TRANSITIONS.lock().expect("Failed to lock transitions");
    for i in 0..10 {
        for j in 0..10 {
            transitions[i * 10 + j] = Transition { a: i as u8, b: j as u8, c: 0 };
        }
    }

    while count < LIMIT {
        if is_prime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1;
        }
        n += 2;
    }

    print_transitions(LIMIT, last_prime);
}