use std::alloc::{alloc, alloc_zeroed, Layout};
use std::ptr;

fn eratosthenes(n: usize, c: &mut usize) -> *mut u8 {
    if n < 2 {
        return ptr::null_mut();
    }

    *c = n - 1; // primes count
    let m = (n as f64).sqrt() as usize;

    let layout = Layout::array::<u8>(n + 1).unwrap();
    let sieve = unsafe { alloc_zeroed(layout) as *mut u8 };
    unsafe {
        *sieve.add(0) = 1;
        *sieve.add(1) = 1;
        for i in 2..=n {
            *sieve.add(i) = 1;
        }
        for i in 2..=m {
            if *sieve.add(i) == 1 {
                for j in (i*i..=n).step_by(i) {
                    if *sieve.add(j) == 1 {
                        println!("Before a[{}]: 1", j); // Added: Print before marking non-prime
                        *sieve.add(j) = 0;
                        *c -= 1;
                        println!("After a[{}]: 0", j); // Added: Print after marking non-prime
                    }
                }
            }
        }
    }
    sieve
}

fn sieve(a: &mut [i32], n: usize) {
    // Initialize the array to 1 for indices from 0 to n
    for i in 0..=n {
        a[i] = 1;
    }

    // Mark non-prime numbers
    for i in 2..=n {
        if a[i] == 1 {
            for j in (i*i..=n).step_by(i) { // Start from i*i to correctly mark non-prime numbers
                println!("Before a[{}]: 1", j); // Added: Print before marking non-prime
                a[j] = 0; // Mark non-prime numbers as 0
                println!("After a[{}]: 0", j); // Added: Print after marking non-prime
            }
        }
    }

    // Print prime numbers
    println!(); // Added: Newline before printing prime numbers
    println!("Primes numbers from 1 to {} are : ", n);
    for i in 2..=n {
        if a[i] == 1 {
            print!("{}, ", i); // Added: Separator for prime numbers
        }
    }
    println!();
}

fn main() {
    let n = 10;
    let layout = Layout::array::<i32>(n + 1).unwrap();
    let array = unsafe { alloc(layout) as *mut i32 };
    let mut array_slice = unsafe { std::slice::from_raw_parts_mut(array, n + 1) };
    sieve(array_slice, n);
}