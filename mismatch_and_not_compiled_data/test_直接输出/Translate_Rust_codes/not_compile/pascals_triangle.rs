fn main() {
    // The number of Catalan's Numbers to be printed
    const N: i32 = 15;

    // Loop variables
    let mut k: i32;
    let mut n: i32;

    // Necessarily u64 for reaching big values
    let mut num: u64;
    let mut den: u64;

    // The number
    let mut catalan: u64;

    // The first is not calculated for the formula
    println!("1 ");

    // Iterating from 2 to 15
    for n in 2..=N {
        // Initializing for products
        num = 1;
        den = 1;
        // Applying the formula
        for k in 2..=n {
            num *= (n + k) as u64;
            den *= k as u64;
            catalan = num / den;
        }

        // Output
        print!("{} ", catalan);
    }

    // The end
    println!();
}