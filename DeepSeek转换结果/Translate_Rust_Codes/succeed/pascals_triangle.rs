fn main() {
    const N: i32 = 15;

    let mut k: i32;
    let mut n: i32;

    let mut num: u64;
    let mut den: u64;

    let mut catalan: i32;

    print!("1 ");

    for n in 2..=N {
        num = 1;
        den = 1;
        catalan = 1; // Initialize catalan before use
        for k in 2..=n {
            num *= (n + k) as u64;
            den *= k as u64;
            catalan = (num / den) as i32;
        }

        print!("{} ", catalan);
    }

    println!();
}