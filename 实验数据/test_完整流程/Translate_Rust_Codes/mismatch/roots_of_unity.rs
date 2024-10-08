fn main() {
    let pi2 = f64::atan2(1.0, 1.0) * 8.0; // Ensure arguments are of type f64
    for n in 1..10 { // Ensure the range is used correctly
        for i in 0..n { // Ensure n is of type usize
            let (mut c, mut s) = (0.0, 0.0); // Ensure c and s are of type f64
            if i == 0 { // Ensure i is of type usize
                c = 1.0;
            } else if i == n / 2 { // Modified: Corrected condition for setting s = 1.0
                s = 1.0;
            } else if i == n / 4 { // Modified: Corrected condition for setting c = -1.0 and s = -1.0
                c = -1.0;
                s = -1.0;
            } else if i == 3 * n / 4 { // Modified: Corrected condition for setting s = -1.0
                s = -1.0;
            } else {
                let a = i as f64 * pi2 / n as f64; // Ensure i and n are cast to f64
                c = f64::cos(a); // Ensure a is of type f64
                s = f64::sin(a); // Ensure a is of type f64
            }

            if c != 0.0 { // Ensure c is of type f64
                print!("{:.2}", c); // Ensure c is of type f64
            }
            print!(
                "{}",
                if s == -1.0 { // Modified: Check s == -1.0 before s == 1.0
                    "-i".to_string() // Convert the string to a String to match the other branches
                } else if s == 1.0 { // Ensure s is of type f64
                    "i".to_string() // Convert the string to a String to match the other branches
                } else if s != 0.0 { // Ensure s is of type f64
                    let formatted_s = format!("{:+.2}i", s); // Store the formatted string in a variable
                    formatted_s // Return the owned string directly
                } else {
                    "".to_string() // Convert the empty string to a String to match the other branches
                }
            );
            print!("{}", if i == n - 1 { // Ensure i and n are of type usize
                "\n"
            } else {
                ",  "
            });
        }
    }
}