fn even_sel(x: i32) -> bool { x % 2 == 0 } // No change needed
fn tri_sel(x: i32) -> bool { x % 3 != 0 } // No change needed

fn grep(in_vec: &mut Vec<i32>, outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    // Modified: Initialize `out` as an empty vector when `inplace` is false
    let mut out: Vec<i32> = if inplace {
        in_vec.clone()
    } else {
        Vec::new()
    };

    let mut j = 0;
    // Modified: Use index-based loop to avoid immutable borrow during mutable borrow
    for i in 0..in_vec.len() {
        let item = in_vec[i];
        if sel(item) {
            if inplace {
                in_vec[j] = item; // Corrected: Modify the original `in_vec` directly
            } else {
                out.push(item);
            }
            j += 1;
        }
    }

    // Modified: Correct condition to handle in-place filtering
    if inplace {
        in_vec.truncate(j); // Corrected: Truncate the original `in_vec`
    } else {
        out.truncate(j);
    }

    *outlen = j; // No change needed
    out
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut even_len = 0; // No change needed
    let mut in_place_filtered_len = 0; // No change needed

    let even = grep(&mut in_vec.clone(), &mut even_len, even_sel, false);
    print!("Filtered even:");
    // Modified: Ensure the correct vector is being printed
    for item in even {
        print!(" {}", item);
    }
    println!();

    let in_place_filtered = grep(&mut in_vec, &mut in_place_filtered_len, tri_sel, true);
    print!("In-place filtered not multiple of 3:");
    // Modified: Ensure the correct vector is being printed
    for item in &in_vec {
        print!(" {}", item);
    }
    println!();
}