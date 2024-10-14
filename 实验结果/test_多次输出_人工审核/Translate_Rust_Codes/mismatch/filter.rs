fn even_sel(x: i32) -> bool { x & 1 == 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &mut Vec<i32>, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut out_vec: Vec<i32>;

    if inplace {
        out_vec = in_vec.drain(..).filter(|&x| sel(x)).collect();
    } else {
        out_vec = in_vec.iter().filter(|&&x| sel(x)).cloned().collect();
    }

    out_vec
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even_vec = grep(&mut in_vec.clone(), even_sel, false);
    println!("Filtered even: {:?}", even_vec);

    grep(&mut in_vec, tri_sel, true);
    println!("In-place filtered not multiple of 3: {:?}", in_vec);
}