fn mul_inv(a: i32, mut b: i32) -> i32 {
    // Modified: Removed `mut` keyword from `b0` since it does not need to be mutable
    let b0 = b;
    let mut t;
    let mut q;
    let mut x0 = 0;
    let mut x1 = 1;
    if b == 1 {
        return 1;
    }
    // Modified: Declare `a` as mutable to allow reassignment
    let mut a = a;
    while a > 1 {
        q = a / b;
        t = b;
        b = a % b;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 {
        x1 += b0;
    }
    x1
}

fn chinese_remainder(n: &[i32], a: &[i32]) -> i32 {
    let len = n.len();
    // Modified: Declare `prod` as mutable to allow reassignment
    let mut prod = 1;
    // Modified: Declare `sum` as mutable to allow reassignment
    let mut sum = 0;

    for i in 0..len {
        prod *= n[i];
    }

    for i in 0..len {
        let p = prod / n[i];
        sum += a[i] * mul_inv(p, n[i]) * p;
    }

    sum % prod
}

fn main() {
    let n = [3, 5, 7];
    let a = [2, 3, 2];

    println!("{}", chinese_remainder(&n, &a));
}