fn mul_inv(mut a: i32, mut b: i32) -> i32 {
    let mut b0 = b;
    let mut t;
    let mut q;
    let mut x0 = 0;
    let mut x1 = 1;
    if b == 1 {
        return 1;
    }
    // 这里不需要重新绑定 `a`，因为 `a` 已经是可变的
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

fn main() {
    println!("{}", mul_inv(42, 2017));
}