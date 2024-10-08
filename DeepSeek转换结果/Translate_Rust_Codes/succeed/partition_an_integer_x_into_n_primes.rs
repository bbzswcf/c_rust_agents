#[derive(Debug)]
struct BitArray {
    size: u32,
    array: Vec<u32>,
}

impl BitArray {
    fn create(size: u32) -> Option<Self> {
        let array_size = (size + 31) / 32;
        let array = vec![0; array_size as usize];
        Some(BitArray { size, array })
    }

    fn set(&mut self, index: u32, value: bool) {
        assert!(index < self.size);
        let p = &mut self.array[(index >> 5) as usize];
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
        (self.array[(index >> 5) as usize] & bit) != 0
    }
}

#[derive(Debug)]
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
                let mut q = p.saturating_mul(p);
                while q <= limit {
                    not_prime.set(q, true);
                    q = q.saturating_add(p);
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

fn find_prime_partition(s: &Sieve, number: u32, count: u32, min_prime: u32, primes: &mut [u32]) -> bool {
    if count == 1 {
        if number >= min_prime && s.is_prime(number) {
            primes[0] = number;
            return true;
        }
        return false;
    }
    for prime in min_prime..number {
        if !s.is_prime(prime) {
            continue;
        }
        if find_prime_partition(s, number - prime, count - 1, prime + 1, &mut primes[1..]) {
            primes[0] = prime;
            return true;
        }
    }
    false
}

fn print_prime_partition(s: &Sieve, number: u32, count: u32) {
    assert!(count > 0);
    let mut primes = vec![0; count as usize];
    if find_prime_partition(s, number, count, 2, &mut primes) {
        print!("{} = {}", number, primes[0]);
        for i in 1..count {
            print!(" + {}", primes[i as usize]);
        }
        println!();
    } else {
        println!("{} cannot be partitioned into {} primes.", number, count);
    }
}

fn main() {
    const LIMIT: u32 = 100000;
    let sieve = Sieve::create(LIMIT).expect("Out of memory");

    print_prime_partition(&sieve, 99809, 1);
    print_prime_partition(&sieve, 18, 2);
    print_prime_partition(&sieve, 19, 3);
    print_prime_partition(&sieve, 20, 4);
    print_prime_partition(&sieve, 2017, 24);
    print_prime_partition(&sieve, 22699, 1);
    print_prime_partition(&sieve, 22699, 2);
    print_prime_partition(&sieve, 22699, 3);
    print_prime_partition(&sieve, 22699, 4);
    print_prime_partition(&sieve, 40355, 3);
}