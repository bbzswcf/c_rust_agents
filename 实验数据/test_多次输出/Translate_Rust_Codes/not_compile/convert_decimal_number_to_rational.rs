use std::f64;

fn rat_approx(f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    let mut h = [0, 1, 0];
    let mut k = [1, 0, 0];
    let mut n = 1;
    let mut neg = false;

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    if f < 0.0 {
        neg = true;
        f = -f;
    }

    while f != f.floor() {
        n <<= 1;
        f *= 2.0;
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
    *num = if neg { -h[1] } else { h[1] };
}

fn main() {
    let mut d;
    let mut n;
    let mut f;

    println!("f = {:.14}", f = 1.0 / 7.0);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }

    println!("\nf = {:.14}", f = f64::consts::PI);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}