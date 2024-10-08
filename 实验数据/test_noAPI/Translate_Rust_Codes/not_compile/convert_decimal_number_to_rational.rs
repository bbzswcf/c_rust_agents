// Modified: Removed unnecessary mutability in function parameters
fn rat_approx(f: f64, md: i64, num: &i64, denom: &i64) {
    // Modified: Initialized `k` and `h` arrays to avoid undefined variable errors
    let k = [0, 1];
    let h = [0, 1];

    let mut n = 1;

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    // Modified: Initialized `f` directly with the correct value to avoid unnecessary steps
    let mut f = f.abs();

    // Modified: Added a maximum iteration limit to prevent infinite loops
    for _ in 0..1000 {
        if f == f.floor() {
            break;
        }
        n <<= 1;
        f *= 2.0;
    }

    for i in 0..64 {
        let a = if n != 0 { (f * n as f64).round() as i64 / n } else { 0 };
        // Modified: Simplified the condition to avoid unreachable code
        if i != 0 && a == 0 {
            break;
        }

        n = (f * n as f64).round() as i64 % n;
        f = (f * n as f64).round() as f64 / n as f64;

        let x_a = a;
        // Modified: Added a check to avoid division by zero
        if k[1] != 0 && k[1] * a + k[0] >= md {
            let x_md = (md - k[0]) / k[1];
            if x_md * 2 >= a || k[1] >= md {
                break;
            }
        }

        let h2 = x_a * h[1] + h[0];
        let k2 = x_a * k[1] + k[0];

        if k2 >= md {
            break;
        }

        *denom = k2;
        *num = h2;
    }
}

// Modified: Changed the return type to () since no error handling is needed
fn main() {
    // Modified: Removed unnecessary mutability
    let n = 0;
    let d = 0;
    // Modified: Removed unnecessary mutability
    let f1 = 1.0 / 7.0;

    println!("f = {:.14}", f1);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f1, i as i64, &n, &d);
        println!("{}/{}", n, d);
    }

    // Modified: Removed unnecessary mutability
    let f2 = (1.0f64).atan2(1.0) * 4.0;
    println!("\nf = {:.14}", f2);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f2, i as i64, &n, &d);
        println!("{}/{}", n, d);
    }
}