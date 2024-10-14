fn binomial(mut m: u64, n: u64) -> u64 {
    // Modified: Made `m` mutable to allow decrement operation
    let mut r = 1;
    let mut d = m - n; // Declare d as mutable
    if d > n {
        // Modified: Shadow `n` and `d` with new values to avoid reassignment of immutable variables
        let temp = n;
        let n = d;
        let d = temp;
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
    // Ensured all operations within the division are performed on u64 types
    binomial(2 * n as u64, n as u64) / (1 + n as u64)
}

fn catalan2(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }

    let mut r = 0;
    for i in 0..n {
        // Modified: Ensure all operations are performed on i32 types
        r += catalan2(i) * catalan2((n - 1 - i) as i32);
    }
    r
}

fn catalan3(n: i32) -> u64 {
    if n == 0 {
        return 1;
    }
    // Ensured all operations within the multiplication and division are performed on u64 types
    (2 * (2 * n as u64 - 1) * catalan3(n - 1)) / (1 + n as u64)
}

fn main() {
    println!("\tdirect\tsumming\tfrac");
    for i in 0..16 {
        println!("{}\t{}\t{}\t{}", i, catalan1(i), catalan2(i), catalan3(i));
    }
}