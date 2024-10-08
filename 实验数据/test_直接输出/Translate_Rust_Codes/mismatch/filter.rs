fn even_sel(x: i32) -> bool { !(x & 1) != 0 }
fn tri_sel(x: i32) -> bool { x % 3 != 0 }

fn grep(in_vec: &[i32], outlen: &mut usize, sel: fn(i32) -> bool, inplace: bool) -> Vec<i32> {
    let mut out: Vec<i32> = if inplace {
        Vec::with_capacity(in_vec.len())
    } else {
        Vec::with_capacity(in_vec.len())
    };

    for &item in in_vec {
        if sel(item) {
            out.push(item);
        }
    }

    *outlen = out.len();
    out
}

fn main() {
    let in_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut len = 0;

    let even = grep(&in_vec, &mut len, even_sel, false);
    println!("Filtered even:");
    for i in 0..len {
        print!(" {}", even[i]);
    }
    println!();

    let in_place_filtered = grep(&mut in_vec.clone(), &mut len, tri_sel, true);
    println!("In-place filtered not multiple of 3:");
    for i in 0..len {
        print!(" {}", in_place_filtered[i]);
    }
    println!();
}