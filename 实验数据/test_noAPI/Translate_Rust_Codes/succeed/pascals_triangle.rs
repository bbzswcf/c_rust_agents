fn main() {
    const N: i32 = 15;
    // Initialize all variables before using them
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut num: u64 = 1;
    let mut den: u64 = 1;
    let mut catalan: u64 = 1;

    print!("1 ");

    for n in 2..=N {
        num = 1;
        den = 1;
        // Reset catalan to 1 at the start of each outer loop iteration
        catalan = 1;

        for k in 2..=n {
            num *= (n + k) as u64;
            den *= k as u64;
            // Update catalan only after the inner loop completes
            if k == n {
                catalan = num / den;
            }
        }

        print!("{} ", catalan);
    }

    println!();
}