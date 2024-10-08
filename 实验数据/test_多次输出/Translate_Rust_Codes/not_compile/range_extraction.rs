fn rprint(s: Option<&mut String>, x: &[i32]) -> usize {
    let mut a = s.map_or(0, |s| s.len());
    let mut i = 0;
    let mut j;

    while i < x.len() {
        j = i;
        while j < x.len() - 1 && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if let Some(ref mut s) = s {
            if i + 1 < j {
                a += write!(s, "{}{}-{}", if a > 0 { "," } else { "" }, x[i], x[j]).unwrap().len();
            } else {
                while i <= j {
                    a += write!(s, "{}{}", if a > 0 { "," } else { "" }, x[i]).unwrap().len();
                    i += 1;
                }
            }
        } else {
            if i + 1 < j {
                a += format!("{}{}-{}", if a > 0 { "," } else { "" }, x[i], x[j]).len();
            } else {
                while i <= j {
                    a += format!("{}{}", if a > 0 { "," } else { "" }, x[i]).len();
                    i += 1;
                }
            }
        }

        i = j + 1;
    }

    a
}

fn main() {
    let x = [
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let len = rprint(None, &x);
    let mut s = String::with_capacity(len + 1);
    rprint(Some(&mut s), &x);
    println!("{}", s);
}