use std::cmp::Ordering;

fn catcmp(a: &i32, b: &i32) -> Ordering {
    let ab = format!("{}{}", a, b);
    let ba = format!("{}{}", b, a);
    ba.cmp(&ab)
}

fn maxcat(a: &mut [i32]) {
    a.sort_by(catcmp);
    for &num in a {
        print!("{}", num);
    }
    println!();
}

fn main() {
    let mut x = vec![1, 34, 3, 98, 9, 76, 45, 4];
    let mut y = vec![54, 546, 548, 60];

    maxcat(&mut x);
    maxcat(&mut y);
}