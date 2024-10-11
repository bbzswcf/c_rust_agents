use std::f64::consts::E;

fn main() {
    const N: i32 = 2;
    let base = 10;
    let mut c1 = 0;
    let mut c2 = 0;

    // Ensure that `base.pow(N as u32)` does not exceed the maximum value for the integer type.
    // Using `u64` to handle potential overflow.
    let max_k = base.pow(N as u32) as u64;

    // Cast `base - 1` to `u64` once before the loop to avoid repeated casting
    let base_minus_one = (base - 1) as u64;
    for k in 1..max_k {
        c1 += 1;
        // Ensure that the types of the operands in the modulo operation are consistent
        if k % base_minus_one == (k * k) % base_minus_one {
            c2 += 1;
            print!("{} ", k);
        }
    }

    // Specify the type for the numeric literals to avoid ambiguity
    // Cast `c2` and `c1` to `f64` before performing the arithmetic operations
    println!("\nTring {} numbers instead of {} numbers saves {}%%", c2, c1, 100.0_f64 - 100.0_f64 * (c2 as f64) / (c1 as f64));
}