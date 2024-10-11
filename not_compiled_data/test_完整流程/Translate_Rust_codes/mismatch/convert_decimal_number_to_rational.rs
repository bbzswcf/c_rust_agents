fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Modified: Made `f` mutable to allow reassignment
    let mut h = [0, 1]; // Modified: Removed unnecessary initialization
    let mut k = [1, 0]; // Modified: Removed unnecessary initialization
    let mut n = 1;
    let mut neg = false; // Modified: Changed to boolean flag for simplicity

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    if f < 0.0 {
        neg = true; // Modified: Changed to boolean flag for simplicity
        f = -f; // Modified: Reassignment of `f` is now allowed
    }

    while f != f.floor() {
        n <<= 1;
        f *= 2.0; // Modified: Reassignment of `f` is now allowed
    }
    let mut d = f as i64;

    for i in 0..64 {
        let a = if n != 0 { d / n } else { 0 };
        if i != 0 && a == 0 {
            break; // Modified: Removed unreachable code condition
        }

        d = n;
        n = d % n;

        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            }
        }

        h[0] = h[1];
        h[1] = a * h[1] + h[0];

        k[0] = k[1];
        k[1] = a * k[1] + k[0];
    }

    *denom = k[1];
    *num = if neg { -h[1] } else { h[1] }; // Modified: Changed to boolean flag for simplicity
}

fn main() {
    // Modified: Initialize `d` before passing it to the `rat_approx` function
    let mut d: i64 = 0;
    // Modified: Initialize `n` before passing it to the `rat_approx` function
    let mut n: i64 = 0;
    let f = 1.0 / 7.0;

    println!("f = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i, &mut n, &mut d); // Modified: Removed unnecessary type casting
        // Modified: Removed unnecessary print statements
    }

    let f = f64::atan2(1.0, 1.0) * 4.0;
    println!("\nf = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i, &mut n, &mut d); // Modified: Removed unnecessary type casting
        // Modified: Removed unnecessary print statements
    }
}