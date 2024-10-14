use std::alloc::{alloc, alloc_zeroed, Layout};
use std::mem;

fn eratosthenes(n: usize, c: &mut usize) -> *mut u8 {
    if n < 2 {
        return std::ptr::null_mut();
    }

    *c = n - 1; // primes count
    let m = (n as f64).sqrt() as usize;

    let layout = Layout::array::<u8>(n + 1).unwrap();
    let sieve = unsafe { alloc_zeroed(layout) as *mut u8 };
    unsafe {
        *sieve.add(0) = 1;
        *sieve.add(1) = 1;
    }

    for i in 2..=m {
        if unsafe { *sieve.add(i) } == 0 {
            for j in (i * i..=n).step_by(i) {
                if unsafe { *sieve.add(j) } == 0 {
                    unsafe {
                        *sieve.add(j) = 1;
                        *c -= 1;
                    }
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
    let layout = Layout::array::<i32>(n + 1).unwrap();
    let array = unsafe { alloc(layout) as *mut i32 };
    let array_slice = unsafe { std::slice::from_raw_parts_mut(array, n + 1) };
    sieve(array_slice, n);
}