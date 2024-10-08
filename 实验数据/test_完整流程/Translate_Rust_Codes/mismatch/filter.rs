fn even_sel(x: &i32) -> bool { *x & 1 == 0 } // Modified: Changed function signature to accept a reference to i32
fn tri_sel(x: &i32) -> bool { *x % 3 != 0 } // Modified: Changed function signature to accept a reference to i32

fn grep(in_vec: &mut Vec<i32>, sel: fn(&i32) -> bool, inplace: bool) -> (Vec<i32>, usize) {
    // Modified: Return a tuple containing the filtered vector and its length
    if inplace {
        in_vec.retain(sel);
        (in_vec.clone(), in_vec.len()) // Return the new length of the vector
    } else {
        let mut out: Vec<i32> = Vec::with_capacity(in_vec.len());
        for &x in in_vec.iter() {
            if sel(&x) { // Modified: Dereference the argument before passing it to sel
                out.push(x);
            }
        }
        (out.clone(), out.len()) // Return the new length of the vector
    }
}

fn main() {
    let in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut len = 0;

    let mut even = in_vec.clone();
    let (filtered_even, new_len) = grep(&mut even, even_sel, false); // Modified: Update len with the new length
    len = new_len;
    print!("Filtered even:");
    for i in 0..len {
        print!(" {}", filtered_even[i]);
    }
    println!();

    let mut in_place_vec = in_vec.clone();
    let (filtered_tri, new_len) = grep(&mut in_place_vec, tri_sel, true); // Modified: Update len with the new length
    len = new_len;
    print!("In-place filtered not multiple of 3:");
    for i in 0..len {
        print!(" {}", filtered_tri[i]);
    }
    println!();
}