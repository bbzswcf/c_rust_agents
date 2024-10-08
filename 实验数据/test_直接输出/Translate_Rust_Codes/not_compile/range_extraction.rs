use std::io::Write;

fn rprint(s: &mut String, x: &[i32]) -> usize {
    let mut a = s.len();
    let mut i = 0;
    let mut j;

    while i < x.len() {
        j = i;
        while j < x.len() - 1 && x[j + 1] == x[j] + 1 {
            j += 1;
        }

        if i + 1 < j {
            write!(s, "{},{}-{}", if a > 0 { "," } else { "" }, x[i], x[j]).unwrap();
            a += format!("{},{}-{}", if a > 0 { "," } else { "" }, x[i], x[j]).len();
        } else {
            while i <= j {
                write!(s, "{},{}", if a > 0 { "," } else { "" }, x[i]).unwrap();
                a += format!("{},{}", if a > 0 { "," } else { "" }, x[i]).len();
                i += 1;
            }
        }
    }

    a
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let x = [
        0, 1, 2, 4, 6, 7, 8, 11, 12, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 28, 29,
        30, 31, 32, 33, 35, 36, 37, 38, 39,
    ];

    let mut s = String::with_capacity(rprint(&mut String::new(), &x) + 1);
    rprint(&mut s, &x);
    println!("{}", s);

    Ok(())
}