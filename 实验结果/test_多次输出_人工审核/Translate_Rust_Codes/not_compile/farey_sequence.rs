use std::mem;

struct Frac {
    d: i32,
    n: i32,
}

fn farey(n: i32) {
    let mut f1 = Frac { d: 0, n: 1 };
    let mut f2 = Frac { d: 1, n };
    let mut t;

    print!("{}/{} {}/{}", f1.d, f1.n, f2.d, f2.n);
    while f2.n > 1 {
        let k = (n + f1.n) / f2.n;
        t = f1;
        f1 = f2;
        f2 = Frac { d: f2.d * k - t.d, n: f2.n * k - t.n };
        print!(" {}/{}", f2.d, f2.n);
    }

    println!();
}

fn farey_len(n: i32, cache: &mut Vec<Option<u64>>) -> u64 {
    let n = n as usize;
    if n >= cache.len() {
        let old_len = cache.len();
        cache.resize(n + 1, None);
        for i in old_len..=n {
            cache[i] = None;
        }
    } else if let Some(cached_len) = cache[n] {
        return cached_len;
    }

    let len = (n as u64) * (n as u64 + 3) / 2;
    let mut p = 2;
    let mut q;

    while p <= n {
        q = n / (n / p) + 1;
        let sub_len = farey_len(n / p, cache) * (q - p) as u64;
        cache[n / p] = Some(sub_len);
        len -= sub_len;
        p = q;
    }

    cache[n] = Some(len);
    len
}

fn main() {
    for n in 1..=11 {
        print!("{}: ", n);
        farey(n);
    }

    let mut cache = vec![None; 16];
    for n in (100..=1000).step_by(100) {
        println!("{}: {} items", n, farey_len(n, &mut cache));
    }

    let n = 10_000_000;
    println!("\n{}: {} items", n, farey_len(n, &mut cache));
}