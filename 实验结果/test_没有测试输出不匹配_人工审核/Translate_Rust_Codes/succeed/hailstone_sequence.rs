// Removed: Unused import
// use std::mem;

// Modified: Ensure `arry` is mutable in the function signature
fn hailstone(mut n: i32, mut arry: Option<&mut Vec<i32>>) -> i32 {
    let mut hs = 1; // Ensure `hs` is mutable if you intend to modify it later in the code

    while n != 1 {
        hs += 1;
        if let Some(ref mut arry) = arry { // Ensure `arry` is mutable if you intend to modify it
            arry.push(n);
        }
        n = if n & 1 == 1 { 3 * n + 1 } else { n / 2 };
    }
    if let Some(ref mut arry) = arry { // Ensure `arry` is mutable if you intend to modify it
        arry.push(n);
    }
    hs
}

fn main() {
    let mut hmax = 0; // Ensure `hmax` is mutable if you intend to modify it
    let mut jatmax = 0; // Ensure `jatmax` is mutable if you intend to modify it
    let mut n;

    for j in 1..100000 {
        n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }

    n = hailstone(27, None);
    let mut arry = Vec::with_capacity(n as usize); // Ensure `arry` is mutable if you intend to modify it
    n = hailstone(27, Some(&mut arry)); // Ensure `arry` is mutable if you intend to modify it

    // Ensure the index access is within bounds
    if n >= 4 && n <= arry.len() as i32 {
        println!(
            "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
            arry[0],
            arry[1],
            arry[2],
            arry[3],
            arry[n as usize - 4],
            arry[n as usize - 3],
            arry[n as usize - 2],
            arry[n as usize - 1],
            n
        );
    } else {
        println!("Array is too small to access elements at index n-4 to n-1");
    }
    println!("Max {} at j= {}", hmax, jatmax); // Ensure `hmax` and `jatmax` are mutable if you intend to modify them
}