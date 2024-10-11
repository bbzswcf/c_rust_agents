use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

struct BitArray {
    size: u32,
    array: *mut u32,
}

impl BitArray {
    fn create(size: u32) -> Option<Self> {
        // Ensure size is within a safe range before performing the calculation
        assert!(size <= std::u32::MAX - 31, "Size too large for safe calculation");
        let array_size = (size + 31) / 32;
        let layout = Layout::array::<u32>(array_size as usize).unwrap();
        let array = unsafe { alloc(layout) as *mut u32 };
        if array.is_null() {
            None
        } else {
            unsafe { ptr::write_bytes(array, 0, array_size as usize) };
            Some(BitArray { size, array })
        }
    }

    fn destroy(&mut self) {
        unsafe {
            // Ensure size is within a safe range before performing the calculation
            assert!(self.size <= std::u32::MAX - 31, "Size too large for safe calculation");
            let layout = Layout::array::<u32>(((self.size + 31) / 32) as usize).unwrap();
            dealloc(self.array as *mut u8, layout);
        }
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

impl Drop for BitArray {
    fn drop(&mut self) {
        if !self.array.is_null() {
            unsafe {
                // Ensure size is within a safe range before performing the calculation
                assert!(self.size <= std::u32::MAX - 31, "Size too large for safe calculation");
                let layout = Layout::array::<u32>(((self.size + 31) / 32) as usize).unwrap();
                dealloc(self.array as *mut u8, layout);
            }
        }
    }
}

struct Sieve {
    limit: u32,
    not_prime: BitArray,
}

impl Sieve {
    fn create(limit: u32) -> Option<Self> {
        // Ensure limit is within a safe range before performing the addition
        assert!(limit < std::u32::MAX, "Limit too large for safe addition");
        let mut not_prime = BitArray::create(limit + 1)?;
        not_prime.set(0, true);
        not_prime.set(1, true);
        for p in 2..=limit {
            if !not_prime.get(p) {
                // Ensure p is within a safe range before performing the multiplication
                assert!(p != 0 && p.checked_mul(p).is_some(), "p too large for safe multiplication");
                if let Some(mut q) = p.checked_mul(p) { // Modified: Make q mutable
                    while q <= limit {
                        not_prime.set(q, true);
                        // Ensure q and p are within a safe range before performing the addition
                        assert!(q.checked_add(p).is_some(), "q too large for safe addition");
                        if let Some(new_q) = q.checked_add(p) {
                            q = new_q; // Modified: Reassignment to mutable q
                        } else {
                            // Handle the overflow case
                            break;
                        }
                    }
                } else {
                    // Handle the overflow case
                    break;
                }
            }
        }
        Some(Sieve { limit, not_prime })
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
        // Ensure the subtraction and addition do not overflow
        if let Some(new_number) = number.checked_sub(prime) {
            if let Some(new_min_prime) = prime.checked_add(1) {
                if find_prime_partition(s, new_number, count - 1, new_min_prime, &mut p[1..]) {
                    p[0] = prime;
                    return true;
                }
            } else {
                // Handle the overflow case
                break;
            }
        } else {
            // Handle the overflow case
            break;
        }
    }
    false
}

fn print_prime_partition(s: &Sieve, number: u32, count: u32) {
    assert!(count > 0);
    // Ensure the vector initialization does not overflow
    assert!(count <= std::u32::MAX as usize, "Count too large for safe vector initialization");
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
    let s = Sieve::create(LIMIT).expect("Out of memory");

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
}