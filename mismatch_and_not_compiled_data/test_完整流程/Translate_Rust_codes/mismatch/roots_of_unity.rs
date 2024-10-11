fn main() {
    let pi2 = f64::atan2(1.0, 1.0) * 8.0;
    for n in 1..10 {
        for i in 0..n {
            let (mut c, mut s) = (1.0, 0.0); // Modified: Set initial values for `c` and `s` when `i == 0`
            if i != 0 {
                if i == n - 1 {
                    c = -1.0;
                    s = 0.0; // Modified: Ensure `s` is `0.0` when `i == n - 1`
                } else if i == n / 2 {
                    c = 0.0; // Modified: Ensure `c` is `0.0` when `i == n / 2`
                    s = -1.0;
                } else {
                    let a = i as f64 * pi2 / (n as f64); // Modified: Corrected calculation of `a`
                    c = f64::cos(a);
                    s = f64::sin(a);
                }
            }

            // Modified: Always print the real part with two decimal places
            print!("{:.2}", c);

            // Modified: Correctly format the imaginary part with a sign
            if s >= 0.0 {
                print!("+{:.2}i", s);
            } else {
                print!("{:.2}i", s);
            }

            // Modified: Ensure proper newline handling after each row
            if i == n - 1 {
                println!();
            } else {
                print!(",  ");
            }
        }
    }
}