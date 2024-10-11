fn rprint(s: &mut String, x: &[i32]) -> usize {
    let mut i = 0;
    let mut j = 0; // Modified: Initialize `j` to avoid potential issues with uninitialized variables

    while i < x.len() {
        j = i;
        while j + 1 < x.len() && x[j + 1] == x[j] + 1 { // Modified: Ensure safe access to `x[j + 1]`
            j += 1;
        }

        if i + 1 < j {
            if s.len() > 0 {
                s.push_str(&format!(",{}-{}", x[i], x[j])); // Modified: Use `push_str` and `format!` instead of `write!`
            } else {
                s.push_str(&format!("{}-{}", x[i], x[j])); // Modified: Use `push_str` and `format!` instead of `write!`
            }
        } else {
            while i <= j { // Modified: Ensure `i` is incremented correctly within the loop
                if s.len() > 0 {
                    s.push_str(&format!(",{}", x[i])); // Modified: Use `push_str` and `format!` instead of `write!`
                } else {
                    s.push_str(&format!("{}", x[i])); // Modified: Use `push_str` and `format!` instead of `write!`
                }
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

    let mut s = String::new();
    rprint(&mut s, &x);
    println!("{}", s);
}