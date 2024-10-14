const LIMIT: usize = 100000;

#[derive(Copy, Clone)]
struct Transition {
    a: u8,
    b: u8,
    c: u32,
}

static mut TRANSITIONS: [Transition; 100] = [Transition { a: 0, b: 0, c: 0 }; 100];

fn init() {
    for i in 0..10 {
        for j in 0..10 {
            let idx = i * 10 + j;
            unsafe {
                TRANSITIONS[idx].a = i as u8;
                TRANSITIONS[idx].b = j as u8;
                TRANSITIONS[idx].c = 0;
            }
        }
    }
}

fn record(prev: i32, curr: i32) {
    let pd = (prev % 10) as u8;
    let cd = (curr % 10) as u8;

    for i in 0..100 {
        unsafe {
            if TRANSITIONS[i].a == pd && TRANSITIONS[i].b == cd {
                TRANSITIONS[i].c += 1;
                break;
            }
        }
    }
}

fn print_transitions(limit: usize, last_prime: i32) {
    println!("{} primes, last prime considered: {}", limit, last_prime);

    for i in 0..100 {
        unsafe {
            if TRANSITIONS[i].c > 0 {
                println!(
                    "{}->{}  count: {:5}  frequency: {:.2}",
                    TRANSITIONS[i].a,
                    TRANSITIONS[i].b,
                    TRANSITIONS[i].c,
                    100.0 * TRANSITIONS[i].c as f64 / limit as f64
                );
            }
        }
    }
}

fn is_prime(n: i32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    if n % 5 == 0 {
        return n == 5;
    }
    if n % 7 == 0 {
        return n == 7;
    }
    if n % 11 == 0 {
        return n == 11;
    }
    if n % 13 == 0 {
        return n == 13;
    }
    if n % 17 == 0 {
        return n == 17;
    }
    if n % 19 == 0 {
        return n == 19;
    }

    let mut t = 23;
    let mut a1 = 96;
    let mut a2 = 216;
    let mut s = t * t;

    while s <= n {
        if n % t == 0 {
            return false;
        }

        s += a1;
        t += 2;
        a1 += 24;
        assert_eq!(t * t, s);

        if s <= n {
            if n % t == 0 {
                return false;
            }

            s += a2;
            t += 4;
            a2 += 48;
            assert_eq!(t * t, s);
        }
    }

    true
}

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