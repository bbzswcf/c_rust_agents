fn even_sel(x: i32) -> bool { x & 1 == 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }
fn grep(in_vec: &mut [i32], outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut out: Vec<i32> = if inplace {
        Vec::new()
    } else {
        Vec::new()
    };
    let mut j = 0;
    for i in 0..in_vec.len() {
        if sel(in_vec[i]) {
            if inplace {
                in_vec[j] = in_vec[i];
            } else {
                out.push(in_vec[i]);
            }
            j += 1;
        }
    }
    if inplace {
        in_vec.to_vec().truncate(j);
    } else if j < in_vec.len() {
        out.truncate(j);
    }
    *outlen = j;
    if inplace {
        in_vec.to_vec()
    } else {
        out
    }
}
fn main() {
    let mut in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut len = 0;
    let even = grep(&mut in_vec.clone(), &mut len, even_sel, false);
    print!("Filtered even:");
    for i in 0..len {
        print!(" {}", even[i]);
    }
    println!();
    let in_place_filtered = grep(&mut in_vec, &mut len, tri_sel, true);
    print!("In-place filtered not multiple of 3:");
    for i in 0..len {
        print!(" {}", in_vec[i]);
    }
    println!();
}