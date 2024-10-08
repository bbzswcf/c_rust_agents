fn rat_approx(f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    let h = [0, 1, 0]; // Modified: Declared as immutable
    let k = [1, 0, 0]; // Modified: Declared as immutable
    let n = 1; // Modified: Declared as immutable
    let neg = 0; // Modified: Declared as immutable

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    let mut f = f; // Modified: Declared as mutable
    if f < 0.0 {
        let neg = 1; // Modified: Declared as immutable
        f = -f; // Modified: Made f mutable
    }

    let mut n = n; // Modified: Declared as mutable
    while f != f.floor() {
        n <<= 1;
        f *= 2.0; // Modified: Made f mutable
    }
    let mut d = f as i64; // Modified: Declared as mutable

    for i in 0..64 {
        let a = if n != 0 { d / n } else { 0 };
        if i != 0 && a == 0 {
            break;
        }

        let x = d;
        d = n;
        n = x % n;

        let x = a;
        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            }
        }

        let new_h = [x * h[1] + h[0], h[1], h[2]]; // Modified: Avoided variable shadowing
        let new_k = [x * k[1] + k[0], k[1], k[2]]; // Modified: Avoided variable shadowing

        // Update h and k with new_h and new_k
        let h = new_h;
        let k = new_k;
    }
    *denom = k[1];
    *num = if neg != 0 { -h[1] } else { h[1] };
}

fn main() {
    // Modified: Initialize f before using it
    let mut f = 1.0 / 7.0;
    println!("f = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        // Modified: Initialize n and d before using them
        let mut n = 0;
        let mut d = 0;
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    // Modified: Initialize f before using it
    f = f64::atan2(1.0, 1.0) * 4.0;
    println!("\nf = {:.14}", f);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        // Modified: Initialize n and d before using them
        let mut n = 0;
        let mut d = 0;
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}