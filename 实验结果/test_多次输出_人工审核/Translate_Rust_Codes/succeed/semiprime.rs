fn is_semiprime(n: i32) -> bool {
    let mut n = n;
    let mut f = 0;
    let mut p = 2;

    while f < 2 && p * p <= n {
        while n % p == 0 {
            n /= p;
            f += 1;
        }
        p += 1;
    }

    f + (n > 1) as i32 == 2
}

fn main() {
    for i in 2..100 {
        if is_semiprime(i) {
            print!(" {}", i);
        }
    }
    println!();
}