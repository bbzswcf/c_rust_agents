fn gcd_ui(mut x: u64, mut y: u64) -> u64 {
    let mut t: u64; // Explicitly specify type for t
    if y < x {
        t = x;
        x = y;
        y = t;
    }
    while y > 0 {
        t = y;
        y = x % y;
        x = t;
    }
    x
}

fn binomial(mut n: u64, mut k: u64) -> u64 {
    let mut d: u64; // Explicitly specify type for d
    let mut g: u64; // Explicitly specify type for g
    let mut r: u64 = 1; // Explicitly specify type for r
    if k == 0 {
        return 1;
    }
    if k == 1 {
        return n;
    }
    if k >= n {
        return (k == n) as u64;
    }
    if k > n / 2 {
        k = n - k;
    }
    for d in 1..=k {
        if r >= u64::MAX / n {
            let mut nr: u64; // Explicitly specify type for nr
            let mut dr: u64; // Explicitly specify type for dr
            g = gcd_ui(n, d);
            nr = n / g;
            dr = d / g;
            g = gcd_ui(r, dr);
            r = r / g;
            dr = dr / g;
            if r >= u64::MAX / nr {
                return 0;
            }
            r *= nr;
            r /= dr;
            n -= 1;
        } else {
            r *= n;
            n -= 1;
            r /= d;
        }
    }
    r
}

fn main() {
    println!("{}", binomial(5, 3));
    println!("{}", binomial(40, 19));
    println!("{}", binomial(67, 31));
}