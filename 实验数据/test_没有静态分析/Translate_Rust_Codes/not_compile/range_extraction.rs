fn rprint(s: &mut String, x: &[i32]) -> usize {
    // Modified: Directly modify `s` without reassigning to `a`
    let mut i = 0;
    let mut j;

    while i < x.len() {
        j = i;
        // Modified: Corrected loop condition to ensure `j + 1` does not exceed bounds
        while j < x.len() - 1 && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if i + 1 < j {
            // Modified: Efficiently build the formatted string without `format!`
            if !s.is_empty() {
                s.push(',');
            }
            s.push_str(&x[i].to_string());
        } else {
            while i <= j {
                // Modified: Efficiently build the formatted string without `format!`
                if !s.is_empty() {
                    s.push(',');
                }
                s.push_str(&x[i].to_string());
                i += 1;
            }
        }
    }

    s.len()
}

fn main() {
    let x = [
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    // Modified: Corrected capacity calculation based on the length of the input array and separators
    let mut s = String::with_capacity(x.len() * 3); // Assuming each number takes up to 3 characters (e.g., "100")
    rprint(&mut s, &x);
    println!("{}", s);
}