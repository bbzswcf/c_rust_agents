use std::ops::AddAssign;

#[derive(Clone, Debug)]
struct Transition {
    a: u8,
    b: u8,
    c: u32,
}

impl AddAssign for Transition {
    fn add_assign(&mut self, other: Self) {
        self.c += other.c;
    }
}

const TRANSITIONS_SIZE: usize = 100;

fn init(transitions: &mut Vec<Transition>) {
    // Modified: Changed array to Vec to satisfy trait bounds
    for i in 0..10 {
        for j in 0..10 {
            let idx = i * 10 + j;
            transitions[idx].a = i as u8;
            transitions[idx].b = j as u8;
            transitions[idx].c = 0;
        }
    }
}

fn record(transitions: &mut Vec<Transition>, prev: i32, curr: i32) {
    // Modified: Changed array to Vec to satisfy trait bounds
    let pd = (prev % 10) as u8;
    let cd = (curr % 10) as u8;

    for i in 0..100 {
        if transitions[i].a == pd && transitions[i].b == cd {
            transitions[i].c += 1;
            break;
        }
    }
}

fn print_transitions(transitions: &Vec<Transition>, limit: i32, last_prime: i32) {
    // Modified: Changed array to Vec to satisfy trait bounds
    println!("{} primes, last prime considered: {}", limit, last_prime);

    for i in 0..100 {
        if transitions[i].c > 0 {
            println!("{}->{}  count: {:5}  frequency: {:.2}", transitions[i].a, transitions[i].b, transitions[i].c, 100.0 * transitions[i].c as f32 / limit as f32);
        }
    }
}

fn is_prime(n: i32) -> bool {
    // No changes needed here as the function signature is correct
    if n % 2 == 0 { return n == 2; }
    if n % 3 == 0 { return n == 3; }
    if n % 5 == 0 { return n == 5; }
    if n % 7 == 0 { return n == 7; }
    if n % 11 == 0 { return n == 11; }
    if n % 13 == 0 { return n == 13; }
    if n % 17 == 0 { return n == 17; }
    if n % 19 == 0 { return n == 19; }

    let mut t = 23;
    let mut a1 = 96;
    let mut a2 = 216;
    let mut s = t * t;

    while s <= n {
        if n % t == 0 { return false; }

        s += a1;
        t += 2;
        a1 += 24;
        assert!(t * t == s);

        if s <= n {
            if n % t == 0 { return false; }

            s += a2;
            t += 4;
            a2 += 48;
            assert!(t * t == s);
        }
    }

    true
}

fn main() {
    const LIMIT: i32 = 100000;
    let mut transitions = vec![Transition { a: 0, b: 0, c: 0 }; TRANSITIONS_SIZE];
    let mut last_prime = 3;
    let mut n = 5;
    let mut count = 2;

    init(&mut transitions);
    record(&mut transitions, 2, 3);

    while count < LIMIT {
        if is_prime(n) {
            record(&mut transitions, last_prime, n);
            last_prime = n;
            count += 1;
        }
        n += 2;

        if count < LIMIT {
            if is_prime(n) {
                record(&mut transitions, last_prime, n);
                last_prime = n;
                count += 1;
            }
            n += 4;
        }
    }

    print_transitions(&transitions, LIMIT, last_prime);
}