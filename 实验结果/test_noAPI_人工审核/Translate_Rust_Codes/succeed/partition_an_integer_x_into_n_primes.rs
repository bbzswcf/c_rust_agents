use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct BitArray {
    size: u32,
    array: *mut u32,
}

impl BitArray {
    fn create(size: u32) -> Option<Self> {
        let array_size = (size + 31) / 32;
        let layout = Layout::array::<u32>(array_size as usize).unwrap();
        let array = unsafe { alloc(layout) as *mut u32 };
        if array.is_null() {
            return None;
        }
        unsafe { ptr::write_bytes(array, 0, array_size as usize) };
        Some(BitArray { size, array })
    }

    fn destroy(&mut self) {
        let array_size = (self.size + 31) / 32;
        let layout = Layout::array::<u32>(array_size as usize).unwrap();
        unsafe { dealloc(self.array as *mut u8, layout) };
        self.array = ptr::null_mut();
    }

    fn set(&mut self, index: u32, value: bool) {
        assert!(index < self.size);
        let p = unsafe { &mut *self.array.add((index >> 5) as usize) };
        let bit = 1 << (index & 31);
        if value {
            *p |= bit;
        } else {
            *p &= !bit;
        }
    }

    fn get(&self, index: u32) -> bool {
        assert!(index < self.size);
        let bit = 1 << (index & 31);
        (unsafe { *self.array.add((index >> 5) as usize) } & bit) != 0
    }
}

struct Sieve {
    limit: u32,
    not_prime: BitArray,
}

impl Sieve {
    fn create(limit: u32) -> Option<Self> {
        let mut not_prime = BitArray::create(limit + 1)?;
        not_prime.set(0, true);
        not_prime.set(1, true);
        for p in 2..=limit {
            if !not_prime.get(p) {
                // Modified: Use checked multiplication to prevent overflow
                if let Some(q) = p.checked_mul(p) {
                    let mut q = q;
                    while q <= limit {
                        not_prime.set(q, true);
                        // Modified: Use checked addition to prevent overflow
                        if let Some(new_q) = q.checked_add(p) {
                            q = new_q;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
        Some(Sieve { limit, not_prime })
    }

    fn destroy(&mut self) {
        self.not_prime.destroy();
    }

    fn is_prime(&self, n: u32) -> bool {
        assert!(n <= self.limit);
        !self.not_prime.get(n)
    }
}

fn find_prime_partition(s: &Sieve, number: u32, count: u32, min_prime: u32, p: &mut [u32]) -> bool {
    if count == 1 {
        if number >= min_prime && s.is_prime(number) {
            p[0] = number;
            return true;
        }
        return false;
    }
    for prime in min_prime..number {
        if !s.is_prime(prime) {
            continue;
        }
        if find_prime_partition(s, number - prime, count - 1, prime + 1, &mut p[1..]) {
            p[0] = prime;
            return true;
        }
    }
    false
}

fn print_prime_partition(s: &Sieve, number: u32, count: u32) {
    assert!(count > 0);
    let mut primes = vec![0; count as usize];
    if !find_prime_partition(s, number, count, 2, &mut primes) {
        println!("{} cannot be partitioned into {} primes.", number, count);
    } else {
        print!("{} = {}", number, primes[0]);
        for i in 1..count {
            print!(" + {}", primes[i as usize]);
        }
        println!();
    }
}

fn main() {
    const LIMIT: u32 = 100000;
    let mut sieve = match Sieve::create(LIMIT) {
        Some(s) => s,
        None => {
            eprintln!("Out of memory");
            return;
        }
    };
    // Modified: Borrow `sieve` mutably to ensure methods called on it require a mutable reference
    print_prime_partition(&mut sieve, 99809, 1);
    print_prime_partition(&mut sieve, 18, 2);
    print_prime_partition(&mut sieve, 19, 3);
    print_prime_partition(&mut sieve, 20, 4);
    print_prime_partition(&mut sieve, 2017, 24);
    print_prime_partition(&mut sieve, 22699, 1);
    print_prime_partition(&mut sieve, 22699, 2);
    print_prime_partition(&mut sieve, 22699, 3);
    print_prime_partition(&mut sieve, 22699, 4);
    print_prime_partition(&mut sieve, 40355, 3);
    sieve.destroy();
}