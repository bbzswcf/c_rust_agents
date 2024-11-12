use std::ptr;

#[derive(Clone)]
struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n: n };
    let mut t;
    let mut k;
    print!("{} {} {} {}", 0, 1, 1, n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2.clone();
        f2 = Frac { d: f2.d * k - t.d, n: f2.n * k - t.n };
        print!(" {} {}", f2.d, f2.n);
    }
    println!();
}

fn farey_len(n: i32, cache: &mut Vec<u64>, ccap: &mut usize) -> u64 {
    if n as usize >= *ccap {
        let old = *ccap;
        if *ccap == 0 {
            *ccap = 16; // Modified: Initialize ccap to 16 to match original C code logic
        }
        while *ccap <= n as usize {
            *ccap *= 2; // Modified: Resize ccap correctly based on n
        }
        cache.resize(*ccap, 0);
        unsafe {
            ptr::write_bytes(cache.as_mut_ptr().add(old), 0, *ccap - old);
        }
    } else if cache[n as usize] != 0 {
        return cache[n as usize];
    }
    let mut len = (n as u64) * (n as u64 + 3) / 2;
    let mut p: i32 = 2;
    let mut q: i32;
    while p <= n { // Modified: Corrected loop condition to match original C logic
        q = n / (n / p) + 1; // Modified: Corrected calculation of q
        len -= farey_len(n / p, cache, ccap) * (q - p) as u64;
        p = q;
    }
    cache[n as usize] = len;
    len
}

fn main() {
    let mut cache = Vec::new(); // Modified: Initialize cache to an empty vector
    let mut ccap = 16; // Modified: Initialize ccap to 16 to match original C code logic
    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }
    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n, &mut cache, &mut ccap));
    }
    let n = 10000000;
    println!("\n{}: {} items", n, farey_len(n, &mut cache, &mut ccap));
}