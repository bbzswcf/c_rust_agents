fn even_sel(x: i32) -> bool { x & 1 == 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &[i32], sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut out_vec = if inplace {
        in_vec.to_vec()
    } else {
        Vec::with_capacity(in_vec.len())
    };

    out_vec.retain(|&x| sel(x));

    out_vec
}

fn main() {
    let in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even: Vec<i32> = grep(&in_vec, even_sel, false);
    println!("Filtered even: {:?}", even);

    let mut in_vec = in_vec;
    in_vec = grep(&in_vec, tri_sel, true);
    println!("In-place filtered not multiple of 3: {:?}", in_vec);
}