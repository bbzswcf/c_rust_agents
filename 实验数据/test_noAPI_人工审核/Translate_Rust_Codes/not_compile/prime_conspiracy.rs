// Modified: Ensure that the `lazy_static` crate is added to your `Cargo.toml` file.
// [dependencies]
// lazy_static = "1.4.0"  # or the latest version

type Byte = u8;

// Modified: Added `#[derive(Copy, Clone)]` to implement the `Copy` trait
#[derive(Copy, Clone)]
struct Transition {
    a: Byte,
    b: Byte,
    c: u32,
}

const TRANSITIONS_SIZE: usize = 100;

// Modified: Replaced `static mut` with `lazy_static` to ensure safety
use lazy_static::lazy_static;
lazy_static! {
    static ref transitions: [Transition; TRANSITIONS_SIZE] = [Transition { a: 0, b: 0, c: 0 }; TRANSITIONS_SIZE];
}

fn init() {
    for i in 0..10 {
        for j in 0..10 {
            let idx = i * 10 + j;
            // Modified: Dereference `transitions` to access the array elements
            (*transitions)[idx].a = i as Byte;
            (*transitions)[idx].b = j as Byte;
            (*transitions)[idx].c = 0;
        }
    }
}

fn record(prev: i32, curr: i32) {
    let pd = (prev % 10) as Byte;
    let cd = (curr % 10) as Byte;

    for i in 0..100 {
        // Modified: Dereference `transitions` to access the array elements
        if (*transitions)[i].a == pd {
            if (*transitions)[i].b == cd {
                (*transitions)[i].c += 1;
                break;
            }
        }
    }
}

fn print_transitions(limit: i32, last_prime: i32) {
    println!("{} primes, last prime considered: {}", limit, last_prime);

    for i in 0..100 {
        // Modified: Dereference `transitions` to access the array elements
        if (*transitions)[i].c > 0 {
            println!("{}->{}  count: {:5}  frequency: {:.2}", (*transitions)[i].a, (*transitions)[i].b, (*transitions)[i].c, 100.0 * (*transitions)[i].c as f64 / limit as f64);
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
    let mut last_prime = 3;
    let mut n = 5;
    let mut count = 2;

    init();
    record(2, 3);

    while count < LIMIT {
        if is_prime(n) {
            record(last_prime, n);
            last_prime = n;
            count += 1;
        }
        n += 2;

        if count < LIMIT {
            if is_prime(n) {
                record(last_prime, n);
                last_prime = n;
                count += 1;
            }
            n += 4;
        }
    }

    print_transitions(LIMIT, last_prime);
}