fn root(base: u64, n: u64) -> u64 {
    let mut n1: u64;
    let mut n2: u64;
    let mut n3: u64;
    let mut c: u64;
    let mut d: u64;
    let mut e: u64;

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
    e = (n3 * d + base / (d as f64).powi(n1 as i32) as u64) / n2;

    while c != d && c != e {
        c = d;
        d = e;
        e = (n3 * e + base / (e as f64).powi(n1 as i32) as u64) / n2;
    }

    if d < e {
        d
    } else {
        e
    }
}

fn main() {
    let b: u64 = 2e18 as u64;

    println!("3rd root of 8 = {}", root(8, 3));
    println!("3rd root of 9 = {}", root(9, 3));
    println!("2nd root of {} = {}", b, root(b, 2));
}