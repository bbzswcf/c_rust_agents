use std::alloc;
use std::ptr;
use std::convert::TryInto; // Added: Import the `TryInto` trait to make the `try_into` method available.

struct BitArray {
    size: u32,
    array: *mut u32,
}

impl BitArray {
    fn create(b: &mut BitArray, size: u32) -> bool {
        let array_size = (size + 31) / 32;
        let layout = alloc::Layout::array::<u32>(array_size as usize).unwrap();
        let array = unsafe { alloc::alloc(layout) as *mut u32 };
        if array.is_null() {
            return false;
        }
        unsafe { ptr::write_bytes(array, 0, array_size as usize) };
        b.size = size;
        b.array = array;
        true
    }

    fn destroy(&mut self) {
        if !self.array.is_null() {
            // Modified: Ensure the argument passed to `alloc::Layout::array` is of type `usize`.
            // Convert the result of the division to `usize`.
            let layout = alloc::Layout::array::<u32>(((self.size + 31) / 32u32) as usize).unwrap();
            unsafe { alloc::dealloc(self.array as *mut u8, layout) };
            self.array = ptr::null_mut();
        }
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
    fn create(s: &mut Sieve, limit: u32) -> bool {
        if !BitArray::create(&mut s.not_prime, limit + 1) {
            return false;
        }
        s.not_prime.set(0, true);
        s.not_prime.set(1, true);
        for p in 2..=limit {
            if !s.not_prime.get(p) {
                // Modified: Use checked multiplication to prevent overflow
                if let Some(q) = p.checked_mul(p) {
                    let mut q = q;
                    while q <= limit {
                        s.not_prime.set(q, true);
                        // Modified: Use checked addition to prevent overflow
                        if let Some(new_q) = q.checked_add(p) {
                            q = new_q;
                        } else {
                            break; // Handle overflow by breaking the loop
                        }
                    }
                }
            }
        }
        s.limit = limit;
        true
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
    let mut s = Sieve { limit: 0, not_prime: BitArray { size: 0, array: ptr::null_mut() } };
    if !Sieve::create(&mut s, LIMIT) {
        eprintln!("Out of memory");
        return;
    }
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