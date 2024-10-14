fn main() {
    const N: usize = 15;

    print!("1 ");

    for n in 2..=N {
        let mut num = 1;
        let mut den = 1;
        let mut catalan = 0;

        for k in 2..=n {
            num *= n + k;
            den *= k;
            catalan = num / den;
        }

        print!("{} ", catalan);
    }

    println!();
}