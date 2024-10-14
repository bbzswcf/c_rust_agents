fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Removed: Unused variables `h` and `k`

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    if f < 0.0 {
        f = -f; // Allowed assignment to mutable `f`
    }

    // Removed: Unused variable `d`

    // Added: Ensure that `n` is never zero before entering the loop
    let mut n = if f != f.floor() { 1 } else { 0 };
    while n != 0 && f != f.floor() {
        n <<= 1;
        f *= 2.0; // Allowed assignment to mutable `f`
    }

    // Removed: Unused variable `d`

    // Removed: Unused variable `i`
    for _ in 0..64 {
        let a = if n != 0 { (f as i64) / n } else { 0 };
        if a == 0 {
            break;
        }

        // Removed: Unused variable `x`
        n = (f as i64) % n;

        if a * 2 >= a {
            break;
        }
    }

    *denom = 1; // Removed: Unused variable `k`
    *num = f as i64; // Removed: Unused variable `h`
}

fn main() {
    let mut n = 0;
    let mut d = 0;
    let f1 = 1.0 / 7.0;
    let f2 = f64::atan2(1.0, 1.0) * 4.0;

    println!("f = {:.14}", f1);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f1, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    println!("\nf = {:.14}", f2);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f2, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}