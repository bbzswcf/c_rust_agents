use std::mem;
use std::ptr;

fn eratosthenes(n: i32, c: &mut i32) -> Option<Box<[u8]>> {
    if n < 2 {
        return None;
    }

    *c = n - 1; // primes count
    let m = (n as f64).sqrt() as i32;

    let mut sieve = vec![0u8; (n + 1) as usize].into_boxed_slice();
    sieve[0] = 1;
    sieve[1] = 1;

    for i in 2..=m {
        if sieve[i as usize] == 0 {
            for j in (i * i..=n).step_by(i as usize) {
                if sieve[j as usize] == 0 {
                    sieve[j as usize] = 1;
                    *c -= 1;
                }
            }
        }
    }

    Some(sieve)
}

fn sieve(a: &mut [i32], n: i32) {
    for i in 2..=n {
        a[i as usize] = 1;
    }

    for i in 2..=n {
        println!("\ni:{}", i);
        if a[i as usize] == 1 {
            for j in i.. {
                let index = i * j;
                if index > n {
                    break;
                }
                println!("\nj:{}", j);
                println!("\nBefore a[{}*{}]: {}", i, j, a[index as usize]);
                a[index as usize] = 0;
                println!("\nAfter a[{}*{}]: {}", i, j, a[index as usize]);
            }
        }
    }

    println!("\nPrimes numbers from 1 to {} are : ", n);
    for i in 2..=n {
        if a[i as usize] == 1 {
            print!("{}, ", i);
        }
    }
    println!("\n\n");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n = 10;
    let mut array = vec![0i32; (n + 1) as usize].into_boxed_slice();
    sieve(&mut array, n);
    Ok(())
}