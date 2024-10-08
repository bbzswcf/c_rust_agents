use std::f64::consts::PI;

fn main() {
    // Modified: Corrected the calculation of pi2 to directly use 2 * PI
    let pi2 = 2.0 * PI;

    for n in 1..10 {
        for i in 0..n {
            let (mut c, mut s) = (0.0, 0.0);

            if i == 0 {
                c = 1.0;
            } else if i == n / 4 { // Modified: Corrected the condition for s = 1.0
                s = 1.0;
            } else if i == n / 2 { // Modified: Corrected the condition for c = -1.0
                c = -1.0;
            } else if i == 3 * n / 4 { // Modified: Corrected the condition for s = -1.0
                s = -1.0;
            } else {
                let a = (i as f64) * pi2 / (n as f64); // Modified: Ensured the calculation of a is correct
                c = a.cos();
                s = a.sin();
            }

            if c != 0.0 {
                print!("{:.2}", c);
            }

            match s {
                1.0 => print!("+1.00i"), // Modified: Ensured consistent formatting of complex numbers
                -1.0 => print!("-1.00i"),
                _ if s != 0.0 => print!("{:+.2}i", s),
                _ => (),
            }

            if i == n - 1 { // Modified: Ensured the condition correctly identifies the last element in the row
                println!();
            } else {
                print!(",  ");
            }
        }
    }
}