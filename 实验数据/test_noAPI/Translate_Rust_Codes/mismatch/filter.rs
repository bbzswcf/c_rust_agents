fn is_even(x: i32) -> bool { (x & 1) == 0 }

fn not_multiple_of_three(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &mut Vec<i32>, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut j = 0;

    // Iterate over indices to avoid immutable borrow during mutable borrow
    for i in 0..in_vec.len() {
        let item = in_vec[i];
        if sel(item) {
            if inplace {
                in_vec[j] = item; // Correctly place the item in the in-place vector
            } else {
                in_vec[j] = item; // Place the item in the in-place vector for consistency
            }
            j += 1;
        }
    }

    if inplace {
        // Truncate `in_vec` directly when `inplace` is `true`
        in_vec.truncate(j);
        Vec::new() // Return an empty vector to indicate in-place modification
    } else {
        in_vec.truncate(j); // Truncate `in_vec` for the non-inplace case
        in_vec.to_vec() // Return a clone of the modified vector
    }
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Avoid cloning `in_vec` for the first call to `grep`
    let even = grep(&mut in_vec, is_even, false);
    print!("Filtered even:");
    for &item in even.iter() {
        print!(" {}", item);
    }
    println!();

    grep(&mut in_vec, not_multiple_of_three, true);
    print!("In-place filtered not multiple of 3:");
    for &item in in_vec.iter() {
        print!(" {}", item);
    }
    println!();

    // Removed: Unnecessary variable initialization
    // let mut len = in_vec.len();
}