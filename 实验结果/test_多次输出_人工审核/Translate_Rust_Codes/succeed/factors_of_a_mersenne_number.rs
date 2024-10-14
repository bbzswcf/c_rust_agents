fn is_prime(n: i32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut d = 5;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 2;
        if n % d == 0 {
            return false;
        }
        d += 4;
    }
    true
}

fn main() {
    let q = 929;
    if !is_prime(q) {
        return;
    }

    let mut r = q;
    while r > 0 {
        r <<= 1;
    }

    let mut d = 2 * q + 1;
    loop {
        let mut p = r;
        let mut i = 1;
        while p != 0 {
            i = ((i as i64 * i as i64) % d as i64) as i32;
            if p < 0 {
                i *= 2;
            }
            if i > d {
                i -= d;
            }
            p <<= 1;
        }
        if i != 1 {
            d += 2 * q;
        } else {
            break;
        }
    }
    println!("2^{} - 1 = 0 (mod {})", q, d);
}