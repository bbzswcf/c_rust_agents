fn even_sel(x: i32) -> bool { x & 1 == 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &mut Vec<i32>, sel: fn(i32) -> bool, inplace: bool) {
    if inplace {
        in_vec.retain(|&x| sel(x));
    } else {
        let mut out: Vec<i32> = Vec::with_capacity(in_vec.len());
        for &item in in_vec.iter() {
            if sel(item) {
                out.push(item);
            }
        }
        *in_vec = out;
    }
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let mut even = in_vec.clone();
    grep(&mut even, even_sel, false);
    println!("Filtered even:");
    for i in 0..even.len() {
        print!(" {}", even[i]);
    }
    println!();

    grep(&mut in_vec, tri_sel, true);
    println!("In-place filtered not multiple of 3:");
    for i in 0..in_vec.len() {
        print!(" {}", in_vec[i]);
    }
    println!();
}