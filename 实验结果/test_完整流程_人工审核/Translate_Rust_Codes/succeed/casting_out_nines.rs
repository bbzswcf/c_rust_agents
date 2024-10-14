fn main() {
    const N: i32 = 2;
    let base = 10;
    // Modified: Added type annotations to explicitly specify the types of `c1` and `c2`
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    // Modified: Initialized `k` with a value to allow type inference
    let mut k = 0;

    // Modified: Ensured that the type of `k` is inferred correctly by initializing it before the loop
    for k in 1..i32::pow(base, N as u32) {
        c1 += 1;
        if k % (base - 1) == (k * k) % (base - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    // Modified: Corrected the percentage calculation and formatting
    println!("\nTring {} numbers instead of {} numbers saves {:.6}%", c2, c1, 100.0 * (1.0 - c2 as f64 / c1 as f64));
}