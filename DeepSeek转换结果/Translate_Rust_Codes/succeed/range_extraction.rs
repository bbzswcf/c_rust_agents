fn rprint(s: &mut String, x: &[i32]) -> usize {
    let mut i = 0;
    let mut j;
    let mut a = s.len();

    while i < x.len() {
        j = i;
        while j < x.len() - 1 && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if i + 1 < j {
            if a > 0 {
                s.push_str(&format!(",{}", x[i]));
            } else {
                s.push_str(&format!("{}", x[i]));
            }
            s.push_str(&format!("-{}", x[j]));
        } else {
            while i <= j {
                if a > 0 {
                    s.push_str(&format!(",{}", x[i]));
                } else {
                    s.push_str(&format!("{}", x[i]));
                }
                i += 1;
            }
        }
        i = j + 1;
        a += 1;
    }

    s.len()
}

fn main() {
    let x = vec![
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let mut s = String::new();
    rprint(&mut s, &x);
    println!("{}", s);
}