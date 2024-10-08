fn main() {
    let pi2 = f64::atan2(1.0, 1.0) * 8.0;
    for n in 1..10 {
        for i in 0..n {
            let (mut c, mut s) = (0.0, 0.0);
            if i == 0 {
                c = 1.0;
            } else if n == 4 * i {
                s = 1.0;
            } else if n == 2 * i {
                c = -1.0;
            } else if 3 * n == 4 * i {
                s = -1.0;
            } else {
                let a = i as f64 * pi2 / n as f64;
                c = f64::cos(a);
                s = f64::sin(a);
            }

            if c != 0.0 {
                print!("{:.2}", c);
            }
            match s {
                1.0 => print!("i"),
                -1.0 => print!("-i"),
                _ if s != 0.0 => print!("{:+.2}i", s),
                _ => (),
            }
            if i == n - 1 {
                println!();
            } else {
                print!(",  ");
            }
        }
    }
}