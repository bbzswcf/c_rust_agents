fn main() {
    const N: u32 = 2;
    let base: i32 = 10;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut k: i32;

    // Modified: Corrected the loop range to include the upper bound
    // The loop now iterates from 1 to base.pow(N) - 1 inclusive
    for k in 1..=base.pow(N) - 1 {
        c1 += 1;
        // Modified: Corrected the condition to check if k meets the criteria
        if k % (base - 1) == (k * k) % (base - 1) {
            c2 += 1;
            print!("{} ", k);
        }
    }

    // Modified: Ensured the percentage savings is calculated correctly
    // The formula remains the same, but the output format is adjusted
    // to display two decimal places for accuracy
    println!("\nTring {} numbers instead of {} numbers saves {:.2}%", c2, c1, 100.0 * (c1 - c2) as f64 / c1 as f64);
}