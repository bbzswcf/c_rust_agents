fn mul_inv(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
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

fn main() {
    println!("{}", mul_inv(42, 2017));
}