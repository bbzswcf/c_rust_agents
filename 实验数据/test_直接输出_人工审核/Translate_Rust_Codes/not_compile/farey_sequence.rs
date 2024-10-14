use std::ptr;

struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n };
    let mut t;
    let mut k;

    println!("{}/{} {}/{}", 0, 1, 1, n);
    while f2.n > 1 {
        k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        f2 = Frac { d: f2.d * k - t.d, n: f2.n * k - t.n };
        print!(" {}/{}", f2.d, f2.n);
    }

    println!();
}

type ULL = u64;

fn farey_len(n: i32, cache: &mut Vec<ULL>, ccap: &mut usize) -> ULL {
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

    let mut len = (n as ULL) * (n as ULL + 3) / 2;
    let mut p = 2;
    let mut q;

    while p <= n {
        q = n / (n / p) + 1;
        len -= farey_len(n / p, cache, ccap) * (q - p) as ULL;
        p = q;
    }

    cache[n as usize] = len;
    len
}

fn main() {
    let mut cache = vec![0; 16];
    let mut ccap = 16;

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