use std::cmp::Ordering;
use std::fmt::Write;

// Ensure `catcmp` takes two arguments of type `&i32`
fn catcmp(a: &i32, b: &i32) -> Ordering {
    let mut ab = String::new();
    let mut ba = String::new();
    // Ensure `a` and `b` are correctly formatted as strings
    write!(&mut ab, "{}{}", a, b).unwrap();
    write!(&mut ba, "{}{}", b, a).unwrap();
    ba.cmp(&ab)
}

fn maxcat(a: &mut [i32]) {
    a.sort_by(catcmp);
    // Ensure `num` is correctly dereferenced
    for num in a { // Modified: Corrected dereferencing in the for loop
        // Ensure `num` is correctly formatted as a string
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