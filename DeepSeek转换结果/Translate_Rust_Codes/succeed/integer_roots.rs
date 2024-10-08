fn root(base: u64, n: u64) -> u64 {
    if base < 2 {
        return base;
    }
    if n == 0 {
        return 1;
    }

    let n1 = n - 1;
    let n2 = n;
    let n3 = n1;
    let mut c = 1;
    let mut d = (n3 + base) / n2;
    let mut e = (n3 * d + base / (d as f64).powf(n1 as f64) as u64) / n2;

    while c != d && c != e {
        c = d;
        d = e;
        e = (n3 * e + base / (e as f64).powf(n1 as f64) as u64) / n2;
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