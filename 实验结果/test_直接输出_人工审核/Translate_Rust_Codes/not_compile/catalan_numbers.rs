fn binomial(m: u64, n: u64) -> u64 {
    let mut r = 1;
    let mut d = m - n;
    if d > n {
        let temp = n;
        n = d;
        d = temp;
    }

    while m > n {
        r *= m;
        m -= 1;
        while d > 1 && r % d == 0 {
            r /= d;
            d -= 1;
        }
    }

    r
}

fn catalan1(n: i32) -> u64 {
    binomial(2 * n as u64, n as u64) / (1 + n) as u64
}

fn catalan2(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut r = 0;
    for i in 0..n {
        r += catalan2(i) * catalan2(n - 1 - i);
    }
    r
}

fn catalan3(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }
    (2 * (2 * n - 1) * catalan3(n - 1)) / (1 + n) as u64
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}