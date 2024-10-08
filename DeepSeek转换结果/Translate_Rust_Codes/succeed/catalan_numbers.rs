fn binomial(mut m: u64, mut n: u64) -> u64 {
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
    binomial(2 * n as u64, n as u64) / (n as u64 + 1)
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
    let n = n as u64; // 将 n 转换为 u64 类型
    (2 * (2 * n - 1) * catalan3(n as i32 - 1)) / (n + 1)
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}