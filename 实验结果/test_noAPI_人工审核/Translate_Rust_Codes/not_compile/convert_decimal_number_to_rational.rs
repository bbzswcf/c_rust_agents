fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Modified: Ensure the array `h` has enough elements to access index `2`
    let mut h = [0_i64, 1_i64, 0_i64];
    // Modified: Ensure the array `k` has enough elements to access index `2`
    let mut k = [1_i64, 0_i64, 0_i64];
    let mut n = 1;

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    let neg = if f < 0.0 {
        // Modified: Directly assign `f` to its absolute value to avoid unnecessary redefinition
        f = f.abs();
        1
    } else {
        0
    };

    // No change needed: Epsilon value check is appropriate
    while (f - f.floor()).abs() > std::f64::EPSILON {
        // Modified: Ensure that the loop has a condition to break out of it if `n` exceeds a reasonable limit
        if n > i64::MAX / 2 {
            break;
        }
        n <<= 1;
        f *= 2.0; // No change needed: `f` is mutable
    }
    let mut d = f as i64;

    for i in 0..64 {
        let a = if n != 0 { d / n } else { 0 };
        if i != 0 && a == 0 {
            break;
        }

        let temp_d = d; // Modified: Avoid variable shadowing by using different variable names
        d = n;
        n = temp_d % n;

        let temp_a = a; // Modified: Avoid variable shadowing by using different variable names
        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            }
        }

        // Modified: Use a more robust method to handle potential overflow
        h[2] = h[1].checked_mul(temp_a).and_then(|v| v.checked_add(h[0])).unwrap_or_else(|| {
            // Handle overflow
            i64::MAX
        });
        h[0] = h[1];
        h[1] = h[2];

        k[2] = k[1].checked_mul(temp_a).and_then(|v| v.checked_add(k[0])).unwrap_or_else(|| {
            // Handle overflow
            i64::MAX
        });
        k[0] = k[1];
        k[1] = k[2];
    }

    *denom = k[1];
    *num = if neg != 0 { -h[1] } else { h[1] };
}

// Modified: Changed return type to `()` as no error handling is needed
fn main() {
    let f1 = 1.0 / 7.0;
    let f2 = f64::atan2(1.0, 1.0) * 4.0;

    println!("f = {:.14}", f1);
    // Modified: Define `n` and `d` once before the loop and reuse them
    let (mut n, mut d) = (0, 0);
    for i in (1..=20000000).step_by(16) {
        rat_approx(f1, i, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    println!("\nf = {:.14}", f2);
    // Modified: Define `n` and `d` once before the loop and reuse them
    let (mut n, mut d) = (0, 0);
    for i in (1..=20000000).step_by(16) {
        rat_approx(f2, i, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}