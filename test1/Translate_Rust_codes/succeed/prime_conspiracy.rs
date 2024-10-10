// Removed: Unused import
// use std::assert;

type Byte = u8;

// Modified: Added #[derive(Copy, Clone)] to allow the struct to be copied when initializing the array
#[derive(Copy, Clone)]
struct Transition {
    a: Byte,
    b: Byte,
    c: u32,
}

const TRANSITIONS_SIZE: usize = 100;

fn init(transitions: &mut [Transition; TRANSITIONS_SIZE]) {
    for i in 0..10 {
        for j in 0..10 {
            let idx = i * 10 + j;
            transitions[idx].a = i as Byte;
            transitions[idx].b = j as Byte;
            // Removed: Unnecessary initialization of transitions[idx].c to 0
        }
    }
}

fn record(transitions: &mut [Transition; TRANSITIONS_SIZE], prev: i32, curr: i32) {
    let pd = prev % 10; // Removed: Unnecessary type casting
    let cd = curr % 10; // Removed: Unnecessary type casting

    for i in 0..100 {
        // Modified: Convert u8 values to i32 to match the types of pd and cd
        if i32::from(transitions[i].a) == pd && i32::from(transitions[i].b) == cd {
            transitions[i].c += 1;
            break;
        }
    }
}

fn print_transitions(transitions: &[Transition; TRANSITIONS_SIZE], limit: i32, last_prime: i32) {
    println!("{} primes, last prime considered: {}", limit, last_prime);

    for i in 0..100 {
        if transitions[i].c > 0 {
            println!("{}->{}  count: {:5}  frequency: {:.2}", transitions[i].a, transitions[i].b, transitions[i].c, 100.0 * transitions[i].c as f64 / limit as f64);
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

const LIMIT: i32 = 100000;

fn main() {
    let mut transitions = [Transition { a: 0, b: 0, c: 0 }; TRANSITIONS_SIZE];
    let mut last_prime = 3;
    let mut n = 5;
    let mut count = 2;

    init(&mut transitions);
    record(&mut transitions, 2, 3);

    // Modified: Simplified prime number generation logic
    while count < LIMIT {
        if is_prime(n) {
            record(&mut transitions, last_prime, n);
            last_prime = n;
            count += 1;
        }
        n += 2;
    }

    print_transitions(&transitions, LIMIT, last_prime);
}