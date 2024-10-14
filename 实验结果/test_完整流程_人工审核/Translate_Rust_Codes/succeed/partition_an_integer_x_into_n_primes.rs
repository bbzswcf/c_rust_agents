use std::alloc::{alloc, alloc_zeroed, Layout};
use std::ptr;

struct BitArray {
    size: u32,
    array: *mut u32,
}

impl BitArray {
    fn create(size: u32) -> Option<Self> {
        // Ensure that the calculation of `array_size` does not overflow
        let array_size = (size.checked_add(31).unwrap_or(u32::MAX)) / 32;
        let layout = Layout::array::<u32>(array_size as usize).unwrap();
        // Ensure that the allocated pointer is not null before dereferencing it
        let array = unsafe { alloc_zeroed(layout) as *mut u32 };
        if array.is_null() {
            None
        } else {
            Some(BitArray { size, array })
        }
    }

    fn destroy(&mut self) {
        self.array = ptr::null_mut();
    }

    fn set(&mut self, index: u32, value: bool) {
        assert!(index < self.size);
        let p = unsafe { &mut *self.array.add((index >> 5) as usize) };
        // Ensure that the bitwise shift operation does not overflow
        let bit = 1u32.checked_shl(index & 31).unwrap_or(0);
        if value {
            *p |= bit;
        } else {
            *p &= !bit;
        }
    }

    fn get(&self, index: u32) -> bool {
        assert!(index < self.size);
        // Ensure that the bitwise shift operation does not overflow
        let bit = 1u32.checked_shl(index & 31).unwrap_or(0);
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
                // Ensure that the multiplication `p * p` does not overflow
                if let Some(q) = p.checked_mul(p) {
                    let mut q = q;
                    while q <= limit {
                        not_prime.set(q, true);
                        // Ensure that the addition `q + p` does not overflow
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
    let mut s = Sieve::create(LIMIT).expect("Out of memory");

    print_prime_partition(&s, 99809, 1);
    print_prime_partition(&s, 18, 2);
    print_prime_partition(&s, 19, 3);
    print_prime_partition(&s, 20, 4);
    print_prime_partition(&s, 2017, 24);
    print_prime_partition(&s, 22699, 1);
    print_prime_partition(&s, 22699, 2);
    print_prime_partition(&s, 22699, 3);
    print_prime_partition(&s, 22699, 4);
    print_prime_partition(&s, 40355, 3);

    s.destroy();
}