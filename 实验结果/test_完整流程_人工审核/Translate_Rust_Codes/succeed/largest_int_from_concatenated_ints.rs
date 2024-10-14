fn catcmp(a: &i32, b: &i32) -> std::cmp::Ordering {
    // Dereference `a` and `b` to get the integer values before formatting them as strings
    let ab = format!("{}{}", *a, *b);
    let ba = format!("{}{}", *b, *a);
    ba.cmp(&ab)
}

fn maxcat(a: &mut Vec<i32>) {
    // Ensure that the `sort_by` method is called with a comparator function that matches the expected signature
    a.sort_by(catcmp);
    // Iterate over the elements directly
    for num in a {
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