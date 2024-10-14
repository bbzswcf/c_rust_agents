fn main() {
    const N: i32 = 2;
    let base: i32 = 10;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut k: i32;

    for k in 1..(base.pow(N as u32) as i32) {
        c1 += 1;
        if k % (base - 1) == (k * k) % (base - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    println!("\nTring {} numbers instead of {} numbers saves {}%", c2, c1, 100.0 - 100.0 * c2 as f64 / c1 as f64);
}