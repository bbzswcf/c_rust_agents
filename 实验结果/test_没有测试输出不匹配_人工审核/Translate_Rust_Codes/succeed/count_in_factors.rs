use std::mem;

type ULONG = u64;

fn get_prime(idx: usize) -> ULONG {
    static mut N_PRIMES: usize = 0;
    static mut ALLOC: usize = 0;
    static mut PRIMES: Vec<ULONG> = Vec::new();

    unsafe {
        if idx >= N_PRIMES {
            if N_PRIMES >= ALLOC {
                ALLOC += 16; // be conservative
                PRIMES.resize(ALLOC, 0);
            }
            if N_PRIMES == 0 {
                PRIMES[0] = 2;
                PRIMES[1] = 3;
                N_PRIMES = 2;
            }

            let mut last = PRIMES[N_PRIMES - 1];
            while idx >= N_PRIMES {
                last += 2;
                for i in 0..N_PRIMES {
                    let p = PRIMES[i];
                    if p * p > last {
                        PRIMES[N_PRIMES] = last;
                        N_PRIMES += 1;
                        break;
                    }
                    if last % p == 0 {
                        break;
                    }
                }
            }
        }
        PRIMES[idx]
    }
}

fn main() {
    for x in 1..1000 {
        print!("{} = ", x);

        let mut n = x;
        let mut first = true;
        let mut i = 0;

        loop {
            let p = get_prime(i);
            while n % p == 0 {
                n /= p;
                if !first {
                    print!(" x ");
                }
                first = false;
                print!("{}", p);
            }
            if n <= p * p {
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