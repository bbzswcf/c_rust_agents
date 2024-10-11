fn binomial(mut m: u64, mut n: u64) -> u64 {
    // Modified: Made arguments `m` and `n` mutable
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
    binomial(2 * n as u64, n as u64) / (1 + n as u64)
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
    // Modified: Converted `i32` values to `u64` to match the expected return type of `u64`
    (2_u64 * (2_u64 * n as u64 - 1) * catalan3(n - 1)) / (1_u64 + n as u64)
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}