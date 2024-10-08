fn get_prime(idx: i32) -> u64 {
    static mut N_PRIMES: i32 = 0;
    static mut ALLOC: i32 = 0;
    static mut PRIMES: Vec<u64> = Vec::new();

    unsafe {
        if idx >= N_PRIMES {
            if N_PRIMES >= ALLOC {
                ALLOC += 16; // be conservative
                PRIMES.resize(ALLOC as usize, 0);
            }
            if N_PRIMES == 0 {
                PRIMES[0] = 2;
                PRIMES[1] = 3;
                N_PRIMES = 2;
            }

            let mut last = PRIMES[(N_PRIMES - 1) as usize];
            while idx >= N_PRIMES {
                last += 2;
                for i in 0..N_PRIMES {
                    let p = PRIMES[i as usize];
                    if p * p > last {
                        PRIMES[N_PRIMES as usize] = last;
                        N_PRIMES += 1;
                        break;
                    }
                    if last % p == 0 {
                        break;
                    }
                }
            }
        }
        PRIMES[idx as usize]
    }
}

fn main() {
    for x in 1..1000 {
        print!("{} = ", x);

        let mut n = x;
        let mut first = true;

        for i in 0.. {
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