use std::alloc;
use std::ptr;

fn eratosthenes(n: usize, c: &mut usize) -> *mut i32 {
    if n < 2 {
        *c = 0; // Initialize prime count to 0 if n < 2
        return ptr::null_mut();
    }

    *c = n - 2; // Initialize prime count correctly by considering that 0 and 1 are not prime numbers
    let m = (n as f64).sqrt() as usize;

    let layout = alloc::Layout::array::<i32>(n + 1).unwrap();
    let sieve = unsafe { alloc::alloc_zeroed(layout) as *mut i32 };

    // Initialize the sieve array to 1 for all indices from 2 to n, except for indices 0 and 1 which should be initialized to 0
    unsafe {
        for i in 2..=n {
            *sieve.add(i) = 1;
        }
        *sieve.add(0) = 0;
        *sieve.add(1) = 0;

        for i in 2..=m {
            if *sieve.add(i) == 1 {
                // The inner loop should start from `i * i` to avoid unnecessary iterations
                for j in (i * i..=n).step_by(i) {
                    if *sieve.add(j) == 1 {
                        *sieve.add(j) = 0; // Set non-prime numbers to 0
                        *c -= 1;
                    }
                }
            }
        }
    }

    sieve
}

fn sieve(a: &mut [i32], n: usize) {
    // Initialize the array to 1 for all indices from 2 to n, except for indices 0 and 1 which should be initialized to 0
    for i in 2..=n {
        a[i] = 1;
    }
    a[0] = 0;
    a[1] = 0;

    for i in 2..=n {
        if a[i] == 1 {
            // The inner loop should start from `i * i` to avoid unnecessary iterations
            for j in (i * i..=n).step_by(i) {
                a[j] = 0; // Set non-prime numbers to 0
            }
        }
    }

    // Add a newline character at the end of the output to match the expected format
    println!("Primes numbers from 1 to {} are : \n", n);
    for i in 2..=n {
        if a[i] == 1 {
            print!("{}, ", i);
        }
    }
    println!();
}

fn main() {
    let n = 10;
    let layout = alloc::Layout::array::<i32>(n + 1).unwrap();
    let array = unsafe { alloc::alloc_zeroed(layout) as *mut i32 };

    unsafe {
        sieve(std::slice::from_raw_parts_mut(array, n + 1), n);
        // Ensure that the allocated memory is properly deallocated to avoid memory leaks
        alloc::dealloc(array as *mut u8, layout);
    }
}