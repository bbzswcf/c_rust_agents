fn even_sel(x: i32) -> bool { x & 1 == 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &mut Vec<i32>, outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    if inplace {
        in_vec.retain(|arg0: &i32| sel(*arg0));
        *outlen = in_vec.len();
        in_vec.to_vec()
    } else {
        let mut out: Vec<i32> = Vec::new();
        for &item in in_vec.iter() {
            if sel(item) {
                out.push(item);
            }
        }
        *outlen = out.len();
        out
    }
}

fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut len = 0;
    let even = grep(&mut in_vec, &mut len, even_sel, false);
    print!("Filtered even:");
    for i in 0..len {
        print!(" {}", even[i]);
    }
    println!();
    
    grep(&mut in_vec, &mut len, tri_sel, true);
    in_vec.truncate(len); // Fix: Truncate the vector to match the number of retained elements
    print!("In-place filtered not multiple of 3:");
    for i in 0..in_vec.len() {
        print!(" {}", in_vec[i]);
    }
    println!();
}