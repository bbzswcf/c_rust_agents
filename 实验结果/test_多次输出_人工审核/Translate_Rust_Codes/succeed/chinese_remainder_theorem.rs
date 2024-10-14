fn mul_inv(mut a: i32, mut b: i32) -> i32 {
    let b0 = b;
    let mut x0 = 0;
    let mut x1 = 1;
    if b == 1 {
        return 1;
    }
    while a > 1 {
        let q = a / b;
        let t = b;
        b = a % b;
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
    let prod = n.iter().product::<i32>();
    let sum: i32 = n.iter().enumerate().map(|(i, &ni)| {
        let p = prod / ni;
        a[i] * mul_inv(p, ni) * p
    }).sum();
    sum % prod
}

fn main() {
    let n = [3, 5, 7];
    let a = [2, 3, 2];
    println!("{}", chinese_remainder(&n, &a));
}