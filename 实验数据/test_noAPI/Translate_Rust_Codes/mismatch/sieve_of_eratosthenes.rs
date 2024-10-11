use std::mem;

fn eratosthenes(n: usize, c: &mut usize) -> Vec<u8> {
    if n < 2 {
        return vec![];
    }

    *c = n - 1; // primes count
    let m = (n as f64).sqrt() as usize;

    let mut sieve = vec![0; n + 1];
    sieve[0] = 1;
    sieve[1] = 1;
    for i in 2..=m {
        if sieve[i] == 0 {
            for j in (i * i..=n).step_by(i) {
                if sieve[j] == 0 {
                    sieve[j] = 1;
                    *c -= 1;
                }
            }
        }
    }
    sieve
}

fn sieve(a: &mut [i32], n: usize) {
    for i in 2..=n {
        a[i] = 1;
    }

    for i in 2..=n {
        println!("\ni:{}", i);
        if a[i] == 1 {
            for j in i.. {
                let idx = i * j;
                if idx > n {
                    break;
                }
                println!("\nj:{}", j);
                println!("\nBefore a[{}*{}]: {}", i, j, a[idx]);
                a[idx] = 0;
                println!("\nAfter a[{}*{}]: {}", i, j, a[idx]);
            }
        }
    }

    println!("\nPrimes numbers from 1 to {} are : ", n);
    for i in 2..=n {
        if a[i] == 1 {
            print!("{}, ", i);
        }
    }
    println!("\n\n");
}

fn main() {
    let n = 10;
    let mut array = vec![0; n + 1];
    sieve(&mut array, n);
}