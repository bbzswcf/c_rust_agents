use std::f64;
type Llong = i64;
struct PrimeArray {
    ptr: Vec<Llong>,
    size: usize,
    capacity: usize,
}
impl PrimeArray {
    fn new() -> Self {
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
            self.capacity = new_capacity;
        }
        self.ptr.push(p);
        self.size += 1;
    }
}
fn main() {
    const CUTOFF: i32 = 200;
    const BIGUN: i32 = 10000;
    const CHUNKS: i32 = 50;
    const LITTLE: i32 = BIGUN / CHUNKS;
    let mut primes = PrimeArray::new();
    let mut c = 0;
    let mut show_each = true;
    let mut u = 0;
    let mut v = 1;
    primes.push_back(3);
    primes.push_back(5);
    println!("The first {} cuban primes:", CUTOFF);
    for i in 1.. {
        u += 6;
        v += u;
        let mx = (v as f64).sqrt().ceil() as Llong;
        let mut found = false;
        for j in 0..primes.size {
            if primes.ptr[j] > mx {
                break;
            }
            if primes.ptr[j] != 0 && v % primes.ptr[j] == 0 {
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
                        if primes.ptr[j] != 0 && z % primes.ptr[j] == 0 {
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
            if c % LITTLE == 0 {
                print!(".");
                if c == BIGUN {
                    break;
                }
            }
        }
    }
    println!("\nThe {}th cuban prime is {}", c, v);
}