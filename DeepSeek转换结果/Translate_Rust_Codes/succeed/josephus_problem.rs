fn jos(n: i32, k: i32, mut m: i32) -> i32 {
    // 修改: 直接在函数参数中声明m为可变,避免重复声明
    for a in (m + 1)..=n {
        m = (m + k) % a;
    }
    m
}

fn jos_large(n: u64, k: u64, mut m: u64) -> u64 {
    // 修改: 直接在函数参数中声明m为可变,避免重复声明
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
    let n = 41;
    let k = 3;
    print!("n = {}, k = {}, final survivor: {}\n", n, k, jos(n, k, 0));

    let n = 9876543210987654321;
    let k = 12031;
    print!("n = {}, k = {}, three survivors:", n, k);

    for i in (0..3).rev() {
        print!(" {}", jos_large(n, k, i));
    }
    print!("\n");
}