use std::io;

type Llong = i64;

struct PrimeArray {
    ptr: Vec<Llong>,
    size: usize,
    capacity: usize,
}

impl PrimeArray {
    fn allocate() -> Self {
        let capacity = 10;
        let ptr = Vec::with_capacity(capacity);
        PrimeArray {
            ptr,
            size: 0,
            capacity,
        }
    }

    fn push_back(&mut self, p: Llong) {
        if self.size >= self.capacity {
            let new_capacity = (3 * self.capacity) / 2 + 1;
            // Ensure Llong (i64) implements Clone
            self.ptr.resize(new_capacity, 0_i64); // Explicitly specify the type for clarity
            self.capacity = new_capacity;
        }
        self.ptr[self.size] = p;
        self.size += 1;
    }
}

fn main() {
    const CUTOFF: i32 = 200;
    const BIGUN: i32 = 10000;
    const CHUNKS: i32 = 50;
    const LITTLE: i32 = BIGUN / CHUNKS;

    let mut primes = PrimeArray::allocate();
    let mut c = 0;
    let mut show_each = true;
    let mut u = 0;
    let mut v = 1;

    primes.push_back(3);
    primes.push_back(5);

    println!("The first {} cuban primes:", CUTOFF);
    for i in 1..i64::MAX { // Use i64::MAX instead of Llong::MAX
        let mut found = false;
        // Ensure correct type conversion
        let mx = (((v as f64) + ((u + 6) as f64)).sqrt().ceil()) as Llong; // Cast to f64 for sqrt operation

        for j in 0..primes.size {
            if primes.ptr[j] > mx {
                break;
            }
            if v % primes.ptr[j] == 0 {
                found = true;
                break;
            }
        }
        if !found {
            c += 1;
            if show_each {
                let mut z = primes.ptr[primes.size - 1] + 2;
                while z <= v - 2 {
                    let mut fnd = false;

                    for j in 0..primes.size {
                        if primes.ptr[j] > mx {
                            break;
                        }
                        if z % primes.ptr[j] == 0 {
                            fnd = true;
                            break;
                        }
                    }
                    if !fnd {
                        primes.push_back(z);
                    }
                    z += 2;
                }
                primes.push_back(v);
                print!("{:11}", v);
                if c % 10 == 0 {
                    println!();
                }
                if c == CUTOFF {
                    show_each = false;
                    println!("\nProgress to the {}th cuban prime: ", BIGUN);
                }
            }
            // Cast LITTLE to usize
            if c % (LITTLE as usize) == 0 {
                print!(".");
                // Cast BIGUN to usize
                if c == (BIGUN as usize) {
                    break;
                }
            }
        }
        v += u + 6;
        u += 6;
    }
    // Cast c to Llong (i64) for consistent type in println
    println!("\nThe {}th cuban prime is {}", c as Llong, v);
}