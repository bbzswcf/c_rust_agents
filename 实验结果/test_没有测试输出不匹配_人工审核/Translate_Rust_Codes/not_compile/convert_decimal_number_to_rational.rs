fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    // Modified: Made `f` mutable to allow reassignment within the function
    let mut h = [0, 1, 0];
    let mut k = [1, 0, 0];
    let mut n = 1;
    let mut neg = 0;

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    if f < 0.0 {
        neg = 1;
        f = -f;
    }

    // Modified: Added a maximum iteration count to prevent potential infinite loop
    let mut max_iterations = 1000;
    while f != f.floor() && max_iterations > 0 {
        n <<= 1;
        f *= 2.0;
        max_iterations -= 1;
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

        let x = a;
        if k[1] * a + k[0] >= md {
            let x = (md - k[0]) / k[1];
            if x * 2 >= a || k[1] >= md {
                break;
            } else {
                break;
            }
        }

        h[2] = x * h[1] + h[0];
        h[0] = h[1];
        h[1] = h[2];

        k[2] = x * k[1] + k[0];
        k[0] = k[1];
        k[1] = k[2];
    }

    *denom = k[1];
    *num = if neg != 0 { -h[1] } else { h[1] };
}

fn main() {
    let mut n = 0;
    let mut d = 0;
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