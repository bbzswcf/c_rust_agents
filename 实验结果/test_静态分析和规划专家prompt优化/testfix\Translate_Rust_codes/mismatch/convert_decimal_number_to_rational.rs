fn rat_approx(mut f: f64, md: i64, num: &mut i64, denom: &mut i64) {
    let mut h = [0, 1];
    let mut k = [1, 0];
    let mut n = 1;
    let neg = f < 0.0;

    if neg {
        f = -f;
    }

    if md <= 1 {
        *denom = 1;
        *num = f as i64;
        return;
    }

    while f - f.floor() > 1e-12 {
        n <<= 1;
        f *= 2.0;
    }

    let mut d = f as i64;

    for _ in 0..64 {
        let a = if n != 0 { d / n } else { 0 };
        if a == 0 {
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
        h[0] = h[1];
        h[1] = x * h[1] + h[0];
        k[0] = k[1];
        k[1] = x * k[1] + k[0];
    }

    *denom = k[1];
    *num = if neg { -h[1] } else { h[1] };
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
    println!("\nf = {:.14}", f64::atan2(1.0, 1.0) * 4.0);
    for i in (1..=20000000).step_by(16) {
        print!("denom <= {}: ", i);
        rat_approx(f64::atan2(1.0, 1.0) * 4.0, i as i64, &mut n, &mut d);
        println!("{}/{}", n, d);
    }
}