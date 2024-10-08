fn kprime(n: i32, k: i32) -> bool {
    let mut p = 2;
    let mut f = 0;
    let mut n = n;

    // Ensure the loop condition is correct for the intended logic
    while f < k && p * p <= n {
        while n % p == 0 {
            n /= p;
            f += 1;
        }
        p += 1;
    }

    // Ensure the return statement correctly reflects the intended logic
    // Modified: Convert boolean result to integer using ternary operator
    f + if n > 1 { 1 } else { 0 } == k
}

fn main() {
    for k in 1..=5 {
        print!("k = {}:", k);

        let mut i = 2;
        let mut c = 0;

        // Ensure the loop condition is correct for the intended logic
        while c < 10 {
            if kprime(i, k) {
                print!(" {}", i);
                c += 1;
            }
            // Ensure the loop increment is correct for the intended logic
            i += 1;
        }

        println!();
    }
}