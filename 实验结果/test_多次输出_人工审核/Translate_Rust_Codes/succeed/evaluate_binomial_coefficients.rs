fn gcd_ui(mut x: u64, mut y: u64) -> u64 {
    if y < x {
        std::mem::swap(&mut x, &mut y);
    }
    while y > 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn binomial(mut n: u64, mut k: u64) -> u64 {
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
    let mut r = 1;
    for d in 1..=k {
        if r >= std::u64::MAX / n {
            let g = gcd_ui(n, d);
            let nr = n / g;
            let dr = d / g;
            let g = gcd_ui(r, dr);
            r /= g;
            if r >= std::u64::MAX / nr {
                return 0;
            }
            r *= nr;
            r /= dr / g;
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