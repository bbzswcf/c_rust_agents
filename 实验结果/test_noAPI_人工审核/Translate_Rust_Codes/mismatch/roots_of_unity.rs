use std::f64::consts::PI;

fn main() {
    let pi2 = 2.0 * PI;

    for n in 1..10 {
        for i in 0..n {
            let mut c = 0.0;
            let mut s = 0.0;

            if i == 0 {
                c = 1.0;
                s = 0.0; // Modified: Corrected handling of s when i is 0
            } else if i == n / 4 {
                c = 0.0; // Modified: Corrected handling of c when i is n / 4
                s = 1.0;
            } else if i == n / 2 {
                c = -1.0;
                s = 0.0; // Modified: Corrected handling of s when i is n / 2
            } else if i == 3 * n / 4 {
                c = 0.0; // Modified: Corrected handling of c when i is 3 * n / 4
                s = -1.0;
            } else {
                // Modified: Corrected the calculation of a to correctly calculate the angle in radians
                let a = (pi2 * (i as f64)) / (n as f64);
                c = a.cos();
                s = a.sin();
            }

            // Modified: Corrected handling of the sign for c
            if c > 0.0 {
                print!("+{:.2}", c);
            } else if c < 0.0 {
                print!("{:.2}", c);
            }

            match s {
                1.0 => print!(" +i"), // Modified: Corrected handling of s in match statement
                -1.0 => print!(" -i"), // Modified: Corrected handling of s in match statement
                _ if s > 0.0 => print!(" +{:.2}i", s), // Modified: Corrected handling of positive s
                _ if s < 0.0 => print!(" {:.2}i", s), // Modified: Corrected handling of negative s
                _ => (),
            }

            // Modified: Corrected handling of the last element in each row
            if i == n - 1 {
                println!();
            } else {
                print!(",  ");
            }
        }
    }
}