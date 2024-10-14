fn main() {
    const N: i32 = 2;
    let base: u32 = 10; // Modified: Specify the type for the `base` variable to resolve the ambiguity
    let mut c1 = 0;
    let mut c2 = 0;
    let mut k: u128; // Modified: Specify the type for the `k` variable to resolve the ambiguity

    // Modified: Ensure the loop range does not cause integer overflow by using a larger integer type
    for k in 1..(base.pow(N as u32) as u128) {
        c1 += 1;
        if k % (base as u128 - 1) == (k * k) % (base as u128 - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    // Modified: Explicitly cast numeric literals to f64 to avoid type ambiguity
    println!("\nTring {} numbers instead of {} numbers saves {}%%", c2, c1, 100.0_f64 - 100.0_f64 * (c2 as f64) / (c1 as f64));
}