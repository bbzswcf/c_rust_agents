// Removed unused import
// use std::f64::consts::SQRT_2;

// Removed unnecessary type alias
// type Llong = i64;

struct PrimeArray {
    ptr: Vec<i64>,
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

fn push_back(primes: &mut PrimeArray, p: i64) {
    if primes.size >= primes.capacity {
        // Modified: Ensure no integer overflow in new_capacity calculation
        let new_capacity = (3 * primes.capacity).checked_div(2).and_then(|x| x.checked_add(1)).expect("Integer overflow in new_capacity calculation");
        primes.ptr.resize(new_capacity, 0);
        primes.capacity = new_capacity;
    }
    primes.ptr.push(p); // Modified: Use push instead of direct index access
    primes.size += 1;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const CUTOFF: i32 = 200;
    const BIGUN: i32 = 10000;
    const CHUNKS: i32 = 50;
    const LITTLE: i32 = BIGUN / CHUNKS;

    let mut primes = allocate();
    // Removed unused variables
    // let c = 0;
    // let show_each = true;
    // let u = 0;
    // let v = 1;

    push_back(&mut primes, 3);
    push_back(&mut primes, 5);

    println!("The first {} cuban primes:", CUTOFF);
    // Modified: Specify the type for `count` to resolve the ambiguity
    let mut count: i64 = 0; // Renamed shadowed variable `c` to `count`
    // Modified: Specify the type for `new_v` to resolve the ambiguity
    let mut new_v: i64 = 1; // Renamed shadowed variable `v` to `new_v`
    // Modified: Specify the type for `new_u` to resolve the ambiguity
    let mut new_u: i64 = 0; // Renamed shadowed variable `u` to `new_u`

    for i in 1.. { // Modified: Removed Llong::MAX to avoid potential infinite loop
        let mut found = false;
        // Modified: Ensure no integer overflow in new_v and new_u calculations
        new_v = new_v.checked_add(new_u + 6).expect("Integer overflow in new_v calculation");
        new_u = new_u.checked_add(6).expect("Integer overflow in new_u calculation");
        let mx = (new_v as f64).sqrt().ceil() as i64;

        for j in 0..primes.size {
            // Modified: Ensure primes.ptr[j] is not zero before performing the modulus operation
            if primes.ptr[j] != 0 && new_v % primes.ptr[j] == 0 {
                found = true;
                break;
            }
        }
        if !found {
            // Modified: Ensure no integer overflow in count increment
            count = count.checked_add(1).expect("Integer overflow in count increment");
            if true { // Modified: Removed unused variable `show_each`
                // Modified: Ensure no integer overflow in new_z calculation
                if primes.size > 0 {
                    let mut new_z = primes.ptr[primes.size - 1].checked_add(2).expect("Integer overflow in new_z calculation");
                    while new_z <= new_v - 2 {
                        let mut found_z = false; // Renamed shadowed variable `fnd` to `found_z`
                        for j in 0..primes.size {
                            if primes.ptr[j] > mx {
                                break;
                            }
                            if new_z % primes.ptr[j] == 0 {
                                found_z = true;
                                break;
                            }
                        }
                        if !found_z {
                            push_back(&mut primes, new_z);
                        }
                        // Modified: Ensure no integer overflow in new_z increment
                        new_z = new_z.checked_add(2).expect("Integer overflow in new_z increment");
                    }
                    push_back(&mut primes, new_v);
                    print!("{:11}", new_v);
                    if count % 10 == 0 {
                        println!();
                    }
                    // Modified: Remove the generic argument from the `into` method call
                    if count == CUTOFF.into() {
                        break; // Modified: Added break condition to avoid infinite loop
                    }
                } else {
                    // Handle the case where primes.size is zero
                    break;
                }
            }
            // Modified: Remove the generic argument from the `into` method call
            if count % LITTLE.into() == 0 {
                print!(".");
                // Modified: Remove the generic argument from the `into` method call
                if count == BIGUN.into() {
                    break; // Modified: Added break condition to avoid infinite loop
                }
            }
        }
    }
    println!("\nThe {}th cuban prime is {}", count, new_v);

    Ok(())
}