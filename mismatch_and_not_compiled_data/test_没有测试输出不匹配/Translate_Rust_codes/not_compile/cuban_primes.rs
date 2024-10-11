use std::f64::consts::SQRT_2;

type Llong = i64;

struct PrimeArray {
    ptr: Vec<Llong>,
    size: usize,
    capacity: usize,
}

fn allocate() -> PrimeArray {
    let capacity = 10;
    PrimeArray {
        ptr: Vec::with_capacity(capacity),
        size: 0,
        capacity,
    }
}

fn push_back(primes: &mut PrimeArray, p: Llong) {
    if primes.size >= primes.capacity {
        let new_capacity = (3 * primes.capacity) / 2 + 1;
        primes.ptr.resize(new_capacity, 0);
        primes.capacity = new_capacity;
    }
    primes.ptr[primes.size] = p;
    primes.size += 1;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CUTOFF: i32 = 200;
    const BIGUN: i32 = 10000;
    const CHUNKS: i32 = 50;
    const LITTLE: i32 = BIGUN / CHUNKS;

    let mut primes = allocate();
    let mut c = 0;
    let mut show_each = true;
    let mut u = 0;
    let mut v = 1;

    push_back(&mut primes, 3);
    push_back(&mut primes, 5);

    println!("The first {} cuban primes:", CUTOFF);
    for i in 1..Llong::MAX {
        let mut found = false;
        v += u + 6;
        u += 6;
        let mx = (v as f64).sqrt().ceil() as Llong;

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
                        push_back(&mut primes, z);
                    }
                    z += 2;
                }
                push_back(&mut primes, v);
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

    Ok(())
}