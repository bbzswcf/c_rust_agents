use std::ptr;

type ULONG = u64;

fn get_prime(idx: usize) -> ULONG {
    static mut N_PRIMES: usize = 0;
    static mut ALLOC: usize = 0;
    static mut PRIMES: *mut ULONG = ptr::null_mut();

    unsafe {
        if idx >= N_PRIMES {
            if N_PRIMES >= ALLOC {
                ALLOC += 16; // be conservative
                PRIMES = realloc(PRIMES, ALLOC);
            }
            if N_PRIMES == 0 {
                PRIMES = realloc(PRIMES, 2);
                (*PRIMES.offset(0)) = 2;
                (*PRIMES.offset(1)) = 3;
                N_PRIMES = 2;
            }

            let mut last = (*PRIMES.offset((N_PRIMES - 1) as isize));
            while idx >= N_PRIMES {
                last += 2;
                for i in 0..N_PRIMES {
                    let p = (*PRIMES.offset(i as isize));
                    if p * p > last {
                        PRIMES = realloc(PRIMES, N_PRIMES + 1);
                        (*PRIMES.offset(N_PRIMES as isize)) = last;
                        N_PRIMES += 1;
                        break;
                    }
                    if last % p == 0 {
                        break;
                    }
                }
            }
        }
        (*PRIMES.offset(idx as isize))
    }
}

unsafe fn realloc(ptr: *mut ULONG, size: usize) -> *mut ULONG {
    let new_size = size * std::mem::size_of::<ULONG>();
    if ptr.is_null() {
        std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(new_size, std::mem::align_of::<ULONG>())) as *mut ULONG
    } else {
        std::alloc::realloc(ptr as *mut u8, std::alloc::Layout::from_size_align_unchecked(new_size, std::mem::align_of::<ULONG>()), new_size) as *mut ULONG
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