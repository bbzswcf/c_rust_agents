use std::sync::Mutex;

type ULONG = u64;

// Modified: Use Mutex to protect shared mutable state
static N_PRIMES: Mutex<i64> = Mutex::new(0);
// Modified: Change the type of `alloc` to `u64` to match the operations being performed
static ALLOC: Mutex<u64> = Mutex::new(0);
// Modified: Use Vec<u64> instead of *mut ULONG to ensure Send and Sync
static PRIMES: Mutex<Vec<ULONG>> = Mutex::new(Vec::new());

fn get_prime(idx: i32) -> ULONG {
    let mut n_primes = N_PRIMES.lock().unwrap();
    let mut alloc = ALLOC.lock().unwrap();
    let mut primes = PRIMES.lock().unwrap();

    // Modified: Convert idx to i64 to match the type of *n_primes
    let idx = i64::from(idx);

    if idx >= *n_primes {
        if *n_primes >= *alloc as i64 {
            // Modified: Use checked_add to handle potential overflow
            if let Some(new_alloc) = alloc.checked_add(16) {
                *alloc = new_alloc;
            } else {
                // Handle overflow gracefully, e.g., log an error and return a default value
                eprintln!("Integer overflow detected");
                return 0u64;
            }
            primes.resize(*alloc as usize, 0u64); // Modified: Resize the Vec with correct default value
        }
        if *n_primes == 0 {
            primes.resize(2, 0u64); // Modified: Resize the Vec with correct default value
            primes[0] = 2;
            primes[1] = 3;
            *n_primes = 2;
        }

        // Modified: Ensure last is of type u64 to match the type of elements in the primes vector
        let mut last: u64 = primes[(*n_primes - 1) as usize];
        while idx >= *n_primes {
            last += 2;
            for i in 0..*n_primes {
                // Modified: Ensure p is of type u64 to match the type of elements in the primes vector
                let p: u64 = primes[i as usize];
                if let Some(p_squared) = p.checked_mul(p) {
                    if p_squared > last {
                        primes[*n_primes as usize] = last;
                        *n_primes += 1;
                        break;
                    }
                } else {
                    // Handle overflow gracefully, e.g., log an error and return a default value
                    eprintln!("Integer overflow detected");
                    break;
                }
                if last % p == 0 {
                    break;
                }
            }
        }
    }
    // Modified: Ensure that the index `idx` is within the bounds of the `primes` vector
    if idx < *n_primes {
        primes[idx as usize]
    } else {
        // Handle out of bounds gracefully by returning a default value
        0u64
    }
}

fn main() {
    for x in 1..1000 {
        print!("{} = ", x);

        let mut n: u64 = x; // Modified: Change n to u64 type
        let mut first = true;
        let mut i: i32 = 0; // Modified: Ensure that the loop variable `i` is of the correct type to match the function parameter type

        loop {
            let p: u64 = get_prime(i); // Modified: Ensure p is of type u64
            while n % p == 0 {
                n /= p;
                if !first {
                    print!(" x ");
                }
                first = false;
                print!("{}", p);
            }
            // Modified: Use checked_mul to handle potential overflow
            if let Some(p_squared) = p.checked_mul(p) {
                if n <= p_squared {
                    break;
                }
            } else {
                // Handle overflow gracefully, e.g., log an error and return a default value
                eprintln!("Integer overflow detected");
                break;
            }
            i += 1;
        }

        if first {
            println!("{}", n);
        } else if n > 1 {
            println!(" x {}", n);
        } else {
            println!();
        }
    }
}