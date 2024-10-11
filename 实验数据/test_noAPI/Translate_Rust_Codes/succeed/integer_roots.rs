use std::f64::consts::E;

type Ulong = u64;

fn root(base: Ulong, n: Ulong) -> Ulong {
    let mut n1: Ulong;
    let mut n2: Ulong;
    let mut n3: Ulong;
    let mut c: Ulong;
    let mut d: Ulong;
    let mut e: Ulong;

    if base < 2 {
        return base;
    }
    if n == 0 {
        return 1;
    }

    n1 = n - 1;
    n2 = n;
    n3 = n1;
    c = 1;
    d = (n3 + base) / n2;
    e = (n3 * d + base / (d as f64).powi(n1 as i32) as Ulong) / n2;

    while c != d && c != e {
        c = d;
        d = e;
        e = (n3 * e + base / (e as f64).powi(n1 as i32) as Ulong) / n2;
    }

    if d < e {
        d
    } else {
        e
    }
}

fn main() {
    let b: Ulong = 2e18 as Ulong;

    println!("3rd root of 8 = {}", root(8, 3));
    println!("3rd root of 9 = {}", root(9, 3));
    println!("2nd root of {} = {}", b, root(b, 2));
}