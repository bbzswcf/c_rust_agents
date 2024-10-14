fn kprime(n: i32, k: i32) -> bool {
    let mut p = 2;
    let mut f = 0;
    let mut n = n;

    while f < k && p * p <= n {
        while n % p == 0 {
            n /= p;
            f += 1;
        }
        p += 1;
    }

    // Modified: Convert the boolean expression `(n > 1)` to an integer value
    f + if n > 1 { 1 } else { 0 } == k
}

fn main() {
    for k in 1..=5 {
        print!("k = {}:", k);

        let mut i = 2;
        let mut c = 0;

        while c < 10 {
            if kprime(i, k) {
                print!(" {}", i);
                c += 1;
            }
            i += 1;
        }

        print!("\n");
    }
}