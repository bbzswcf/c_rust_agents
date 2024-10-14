fn main() {
    const N: i32 = 2;
    let base = 10;
    let mut c1 = 0;
    let mut c2 = 0;
    let mut k: i32; // Added type annotation to resolve compilation error

    for k in 1..i32::pow(base, N as u32) {
        c1 += 1;
        if k % (base - 1) == (k * k) % (base - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    // Modified: Ensured the percentage calculation matches the precision of the C output
    // by using the `format!` macro to control the number of decimal places
    println!("\nTring {} numbers instead of {} numbers saves {:.6}%", c2, c1, 100.0 - 100.0 * c2 as f64 / c1 as f64);
}