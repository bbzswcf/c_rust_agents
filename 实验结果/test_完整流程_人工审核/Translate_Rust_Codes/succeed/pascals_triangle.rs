fn main() {
    const N: i32 = 15;
    let mut k: i32 = 0; // Initialized to 0
    let mut n: i32 = 0; // Initialized to 0
    let mut num: u64 = 1; // Initialized to 1
    let mut den: u64 = 1; // Initialized to 1
    let mut catalan: u64 = 1; // Initialized to 1

    print!("1 ");

    for n in 2..=N {
        num = 1;
        den = 1;
        for k in 2..=n {
            // Use checked operations to prevent overflow
            if let Some(new_num) = num.checked_mul((n + k) as u64) {
                num = new_num;
            } else {
                panic!("Integer overflow in num calculation");
            }
            if let Some(new_den) = den.checked_mul(k as u64) {
                den = new_den;
            } else {
                panic!("Integer overflow in den calculation");
            }
            if den != 0 {
                catalan = num / den;
            } else {
                panic!("Division by zero in catalan calculation");
            }
        }
        print!("{} ", catalan);
    }

    println!();
}