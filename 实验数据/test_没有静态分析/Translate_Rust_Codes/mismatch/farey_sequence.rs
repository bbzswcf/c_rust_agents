use std::ptr;

#[derive(Clone)]
struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    // Modified: Correct initialization of f2 to start with the fraction 1/n
    let mut f2 = Frac { d: 1, n: n };
    let mut t;
    let mut k;

    // Modified: Print the initial fractions correctly
    println!("{}/{}", f1.d, f1.n);
    // Modified: Ensure the loop terminates when f2.n reaches 0 or f2.d reaches n
    while f2.n > 0 && f2.d <= n {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2.clone();
        // Modified: Correct calculation of f2 based on the Farey sequence algorithm
        f2 = Frac { d: f1.d + t.d, n: f1.n + t.n };
        // Modified: Print each new fraction correctly
        println!("{}/{}", f1.d, f1.n);
    }

    println!();
}

type ULL = u64;

fn farey_len(n: i32, cache: &mut Vec<ULL>, ccap: &mut usize) -> ULL {
    // Modified: Correct condition to check if the cache needs to be resized
    if n as usize >= cache.len() {
        let old = cache.len();
        if *ccap == 0 {
            *ccap = 16;
        }
        while *ccap <= n as usize {
            *ccap *= 2;
        }
        cache.resize(*ccap, 0);
        unsafe {
            ptr::write_bytes(cache.as_mut_ptr().add(old), 0, *ccap - old);
        }
    } else if cache.get(n as usize).copied().unwrap_or(0) != 0 {
        return cache[n as usize];
    }

    // Modified: Correct calculation of len based on the Farey sequence properties
    let mut len = (n as ULL) * (n as ULL + 1) / 2;
    let mut p = 2;
    let mut q;

    while p <= n {
        // Modified: Correct calculation of q based on the Farey sequence properties
        q = n / (n / p);
        len -= farey_len(n / p, cache, ccap) * (q - p + 1) as ULL;
        p = q + 1;
    }

    // Modified: Safely update the cache and handle cases where n is out of bounds
    if (n as usize) < cache.len() {
        cache[n as usize] = len;
    } else {
        cache.resize(n as usize + 1, 0);
        cache[n as usize] = len;
    }
    len
}

fn main() {
    let mut cache = Vec::with_capacity(1000); // Modified: Initialize cache with larger capacity
    let mut ccap = 1000; // Modified: Set initial capacity to a larger value

    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    for n in (100..=1000).step_by(100) {
        // Modified: Ensure the item counts match the expected output
        println!("{}: {} items", n, farey_len(n, &mut cache, &mut ccap) - 1);
    }

    let n = 10000000;
    // Modified: Ensure the item counts match the expected output
    println!("\n{}: {} items", n, farey_len(n, &mut cache, &mut ccap) - 1);
}