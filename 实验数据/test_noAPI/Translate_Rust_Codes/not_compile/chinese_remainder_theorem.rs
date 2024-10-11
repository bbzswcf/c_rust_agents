fn mul_inv(mut a: i32, b: i32) -> i32 {
    // Declare `t` as mutable to allow reassignment
    let mut t;
    let mut b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;
    if b == 1 {
        return 1;
    }
    while a > 1 {
        let q = a / b;
        // Reassign `t` without shadowing
        t = b;
        b = a % b;
        a = t;
        // Reassign `t` without shadowing
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
    // Declare `prod` as mutable to allow reassignment
    let mut prod = 1;
    // Declare `sum` as mutable to allow reassignment
    let mut sum = 0;

    for &ni in n {
        prod *= ni;
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