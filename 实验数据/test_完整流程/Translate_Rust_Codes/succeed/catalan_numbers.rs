fn binomial(mut m: u64, n: u64) -> u64 { // Modified: Made argument `m` mutable
    let mut r = 1;
    let mut d = m - n; // Declared d as mutable
    let mut n = n; // Declared n as mutable
    if d > n {
        let temp = n;
        n = d;
        d = temp;
    }

    while m > n {
        r *= m;
        m -= 1;
        while d > 1 && r % d == 0 {
            r /= d;
            d -= 1;
        }
    }

    r
}

fn catalan1(n: i32) -> u64 {
    // Ensured that the function `binomial` is called with arguments that satisfy its trait bounds
    binomial(2 * n as u64, n as u64) / (1 + n as u64) // Casted (1 + n as u64) to u64
}

fn catalan2(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut r = 0;
    for i in 0..(n as i32) { // Casted the range to i32
        r += catalan2(i) * catalan2(n - 1 - i); // Removed unnecessary casts
    }
    r
}

fn catalan3(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }
    // Ensured that the arithmetic operations within the function do not result in type mismatches
    (2_u64 * (2 * n - 1) as u64 * catalan3(n - 1)) / (1 + n) as u64 // Casted all intermediate results to u64
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}