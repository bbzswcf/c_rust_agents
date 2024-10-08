fn rat_approx(f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Initialize arrays with only the necessary elements
    let mut h = [0, 1, 0];
    let mut k = [1, 0, 0];

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    // Renamed shadowed variable `f` to `abs_f` to avoid confusion
    let abs_f = if f < 0.0 {
        *num = -1;
        -f
    } else {
        *num = 1;
        f
    };

    let mut n = 1;

    // Use a more robust condition to avoid infinite loops
    let mut counter = 0;
    while (abs_f - abs_f.floor()).abs() > 1e-10 && counter < 1000 {
        // Ensure that the bitwise shift operation does not cause an overflow
        if n <= i64::MAX >> 1 {
            n <<= 1;
        } else {
            break;
        }
        counter += 1;
    }

    // Modified: Declare `d` as mutable to allow reassignment
    let mut d = abs_f as i64;

    for i in 0..64 {
        // Ensure `n` is not zero before performing the division
        let a = if n != 0 { d / n } else { 0 };
        if i != 0 && a == 0 {
            break;
        }

        // Renamed shadowed variable `x` to avoid confusion
        let d_temp = d;
        let n_temp = n;
        n = d_temp % n;
        d = n_temp;

        let a_temp = a;
        if k[1] * a + k[0] >= md {
            // Removed: Unused variable `x`
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            }
        }

        // Update individual elements of the array to maintain the correct state
        h[0] = h[1];
        h[1] = h[2];
        h[2] = a_temp * h[1] + h[0];

        k[0] = k[1];
        k[1] = k[2];
        k[2] = a_temp * k[1] + k[0];
    }
    *denom = k[1];
    *num *= h[1];
}

fn main() {
    // Removed: Unused variable `f`
    // let f = 1.0 / 7.0;

    println!("f = {:.14}", 1.0 / 7.0);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        let mut n = 0;
        let mut d = 0;
        rat_approx(1.0 / 7.0, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    // Removed: Unused variable `pi`
    // let pi = f64::atan2(1.0, 1.0) * 4.0;
    println!("\nf = {:.14}", f64::atan2(1.0, 1.0) * 4.0);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        let mut n = 0;
        let mut d = 0;
        rat_approx(f64::atan2(1.0, 1.0) * 4.0, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}