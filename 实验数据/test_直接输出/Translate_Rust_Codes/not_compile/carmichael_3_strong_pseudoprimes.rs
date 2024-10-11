fn mod_op(n: i32, m: i32) -> i32 {
    (((n % m) + m) % m)
}

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        return true;
    }
}

fn carmichael3(p1: i32) {
    if !is_prime(p1 as u32) {
        return;
    }

    let mut h3;
    let mut d;
    let mut p2;
    let mut p3;

    for h3 in 1..p1 {
        for d in 1..(h3 + p1) {
            if ((h3 + p1) * (p1 - 1)) % d == 0 && mod_op(-p1 * p1, h3) == d % h3 {
                p2 = 1 + ((p1 - 1) * (h3 + p1) / d);
                if !is_prime(p2 as u32) {
                    continue;
                }
                p3 = 1 + (p1 * p2 / h3);
                if !is_prime(p3 as u32) || (p2 * p3) % (p1 - 1) != 1 {
                    continue;
                }
                println!("{} {} {}", p1, p2, p3);
            }
        }
    }
}

fn main() {
    for p1 in 2..62 {
        carmichael3(p1);
    }
}