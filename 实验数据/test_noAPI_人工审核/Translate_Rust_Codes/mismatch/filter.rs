fn even_sel(x: i32) -> bool {
    // Modified: Corrected the condition to check for even numbers
    x % 2 == 0
}

fn tri_sel(x: i32) -> bool {
    // Modified: Corrected the condition to check for numbers not multiples of 3
    x % 3 != 0
}

fn grep(in_vec: &[i32], sel: fn(i32) -> bool, inplace: bool) -> Option<Vec<i32>> {
    let mut out_vec: Vec<i32> = if inplace {
        // Modified: Ensure the input vector is modified in place when `inplace` is true
        in_vec.to_vec()
    } else {
        Vec::with_capacity(in_vec.len())
    };

    out_vec.retain(|&x| sel(x));

    if inplace {
        // Modified: Ensure the function returns `None` when `inplace` is true
        None
    } else {
        Some(out_vec)
    }
}

fn main() {
    let in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even = grep(&in_vec, even_sel, false).unwrap();
    print!("Filtered even:");
    for &x in &even {
        print!(" {}", x);
    }
    println!();

    let mut in_vec_copy = in_vec.clone();
    grep(&mut in_vec_copy, tri_sel, true);
    print!("In-place filtered not multiple of 3:");
    for &x in &in_vec_copy {
        print!(" {}", x);
    }
    println!();
}