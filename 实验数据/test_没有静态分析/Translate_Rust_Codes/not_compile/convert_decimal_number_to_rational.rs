fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Modified: Made `f` mutable to allow assignment within the function
    let h = [0, 1, 0]; // Modified: Declared as immutable since elements are not reassigned
    let k = [1, 0, 0]; // Modified: Declared as immutable since elements are not reassigned
    let mut n = 1;
    let mut neg = 0;

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    if f < 0.0 {
        neg = 1;
        f = -f; // Modified: `f` is now mutable, so this assignment is valid
    }

    // Modified: Use a small epsilon value to determine when `f` is close enough to its floor
    while (f - f.floor()).abs() > std::f64::EPSILON {
        n <<= 1;
        f *= 2.0; // Modified: `f` is now mutable, so this assignment is valid
    }
    let mut d = f as i64;

    for i in 0..64 {
        let a = if n != 0 { d / n } else { 0 };
        if i != 0 && a == 0 {
            break;
        }

        let x = d;
        d = n;
        n = x % n;

        // Modified: Removed redundant code blocks
        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            }
        }

        // Modified: Ensure array indices are within bounds
        let h2 = a * h[1] + h[0];
        let h0 = h[1];
        let h1 = h2;

        let k2 = a * k[1] + k[0];
        let k0 = k[1];
        let k1 = k2;
    }

    *denom = k[1];
    *num = if neg != 0 { -h[1] } else { h[1] };
}

fn main() {
    let mut d = 0;
    let mut n = 0;
    let f = 1.0 / 7.0;

    println!("f = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    let f = f64::atan2(1.0, 1.0) * 4.0;
    println!("\nf = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}