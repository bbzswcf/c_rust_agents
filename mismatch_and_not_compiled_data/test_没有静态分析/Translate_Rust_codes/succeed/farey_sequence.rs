use std::ptr;

struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n: n };
    let mut t;
    let mut k;

    print!("0/1 1/{}", n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        // Modified: Reassign `f2` directly after moving `f1` to avoid the move
        f2 = Frac { d: f1.d * k - t.d, n: f1.n * k - t.n };
        print!(" {}/{}", f2.d, f2.n);
    }

    println!();
}

fn farey_len(n: i32, cache: &mut Vec<u64>, ccap: &mut usize) -> u64 {
    if n as usize >= *ccap {
        let old = *ccap;
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
    } else if cache[n as usize] != 0 {
        return cache[n as usize];
    }

    let mut len = (n as u64) * (n as u64 + 3) / 2;
    let mut p = 2;
    let mut q;
    while p <= n {
        q = n / (n / p) + 1;
        len -= farey_len(n / p, cache, ccap) * (q - p) as u64;
        p = q;
    }

    cache[n as usize] = len;
    len
}

fn main() {
    // Modified: The variable `n` is declared as immutable since it is not modified within the scope
    let n;
    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    let mut cache = vec![0; 16];
    let mut ccap = 16;

    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n, &mut cache, &mut ccap));
    }

    n = 10000000;
    println!("\n{}: {} items", n, farey_len(n, &mut cache, &mut ccap));
}