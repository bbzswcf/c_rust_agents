fn main() {
    const N: i32 = 15;
    // Initialize all variables before use
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut num: u64 = 0;
    let mut den: u64 = 0;
    let mut catalan: u64 = 0;

    print!("1 ");

    for n in 2..=N {
        num = 1;
        den = 1;
        catalan = 1; // Initialize catalan to 1 for each n

        // Correct calculation of Catalan numbers using binomial coefficients
        for k in 1..=n {
            // Checked multiplication to prevent overflow
            if let Some(result) = num.checked_mul((n + k) as u64) {
                num = result;
            } else {
                // Handle overflow, possibly by breaking the loop or using a different approach
                break;
            }

            // Checked multiplication to prevent overflow
            if let Some(result) = den.checked_mul(k as u64) {
                den = result;
            } else {
                // Handle overflow, possibly by breaking the loop or using a different approach
                break;
            }
        }

        // Ensure that den is not zero before performing the division
        if den != 0 {
            catalan = num / den;
        } else {
            // Handle the case where the denominator is zero
            break;
        }

        print!("{} ", catalan);
    }

    println!();
}