fn jos(n: i32, k: i32, m: i32) -> i32 {
    let mut m = m;
    for a in (m + 1)..=n {
        m = (m + k) % a;
    }
    m
}

type Xint = u64;

fn jos_large(n: Xint, k: Xint, m: Xint) -> Xint {
    if k <= 1 {
        return n - m - 1;
    }

    let mut a = m;
    while a < n {
        let q = (a - m + k - 2) / (k - 1);

        let q = if a + q > n {
            n - a
        } else if q == 0 {
            1
        } else {
            q
        };

        m = (m + q * k) % (a + q);
        a += q;
    }

    m
}

fn main() {
    let n: Xint = 41;
    let k: Xint = 3;
    println!("n = {}, k = {}, final survivor: {}", n, k, jos(n as i32, k as i32, 0));

    let n: Xint = 9876543210987654321;
    let k: Xint = 12031;
    print!("n = {}, k = {}, three survivors:", n, k);

    for i in (0..3).rev() {
        print!(" {}", jos_large(n, k, i));
    }
    println!();
}