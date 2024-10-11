fn main() {
    let pi2 = f64::atan2(1.0, 1.0) * 8.0;
    for n in 1..10 {
        for i in 0..n {
            let (mut c, mut s) = (1.0, 0.0); // Initialize c to 1.0 and s to 0.0
            if i == 1 {
                s = 1.0; // Set s to 1.0 and c to 0.0 when i is 1
                c = 0.0;
            } else if i == 2 {
                c = -1.0; // Set c to -1.0 and s to 0.0 when i is 2
                s = 0.0;
            } else if i == 3 {
                s = -1.0; // Set s to -1.0 and c to 0.0 when i is 3
                c = 0.0;
            } else if i > 3 {
                let a = i as f64 * pi2 / n as f64; // Correct calculation of a
                c = f64::cos(a); // Calculate c using cosine
                s = f64::sin(a); // Calculate s using sine
            }

            if c != 0.0 {
                print!("{:+.2}", c); // Ensure the sign is included
            }
            print!(
                "{}",
                if s == 1.0 {
                    "i".to_string() // Correctly handle s == 1.0
                } else if s == -1.0 {
                    "-i".to_string() // Correctly handle s == -1.0
                } else if s != 0.0 {
                    format!("{:+.2}i", s) // Correctly format s with sign and precision
                } else {
                    "".to_string()
                }
            );
            print!("{}", if i == n - 1 { "\n" } else { ",  " }); // Correct condition for newline
        }
    }
}