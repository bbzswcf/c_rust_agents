fn even_sel(x: i32) -> bool { x % 2 == 0 } // Corrected the logic to check for even numbers
fn tri_sel(x: i32) -> bool { x % 3 != 0 } // Returns true if x is not a multiple of 3

fn grep(in_vec: &mut Vec<i32>, outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) {
    let mut out: Vec<i32> = Vec::with_capacity(in_vec.len());

    let mut j = 0;
    for &item in in_vec.iter() {
        if sel(item) {
            out.push(item); // Use push instead of indexing to avoid out-of-bounds errors
            j += 1;
        }
    }

    if inplace {
        *in_vec = out; // Ensure the original vector `in_vec` is updated with the filtered elements
    }

    *outlen = j; // Ensure len is correctly updated after filtering
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // Declared `in_vec` as mutable to allow mutable borrowing
    let mut len = in_vec.len(); // Initialize `len` with the original vector length

    // Modified: Ensure the `grep` function correctly filters even numbers in place
    grep(&mut in_vec, &mut len, even_sel, true);
    print!("Filtered even:");
    for i in 0..len {
        print!(" {}", in_vec[i]); // Use `in_vec` directly after the `grep` call for even numbers
    }
    println!();

    // Modified: Ensure the length `len` is correctly updated after the first `grep` call
    // len = in_vec.len(); // Removed: Use the `len` variable updated by the `grep` function call

    // Modified: Use `inplace: true` for the second call to `grep` to modify the original vector
    grep(&mut in_vec, &mut len, tri_sel, true);
    // Modified: Ensure the length `len` is correctly updated after the second `grep` call
    // len = in_vec.len(); // Removed: Use the `len` variable updated by the `grep` function call

    print!("In-place filtered not multiple of 3:");
    for i in 0..len {
        print!(" {}", in_vec[i]);
    }
    println!();
}