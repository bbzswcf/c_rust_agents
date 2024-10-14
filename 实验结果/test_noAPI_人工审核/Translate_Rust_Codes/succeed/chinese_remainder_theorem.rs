fn mul_inv(a: i32, mut b: i32) -> i32 { // Modified: Made argument `b` mutable to allow assignment
    let mut b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;
    let mut a = a;

    if b == 1 {
        return 1;
    }

    while a > 1 {
        let q = a / b;
        let t = b;
        b = a % b; // Modified: `b` is now mutable and can be assigned to
        a = t;
        let t = x0;
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
    let mut prod = 1;
    let mut sum = 0;

    for &ni in n {
        prod *= ni;
    }

    for (i, &ni) in n.iter().enumerate() {
        let p = prod / ni;
        sum += a[i] * mul_inv(p, ni) * p;
    }

    sum % prod
}

fn main() {
    let n = [3, 5, 7];
    let a = [2, 3, 2];

    println!("{}", chinese_remainder(&n, &a));
}