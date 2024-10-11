// Removed unused import
// use std::mem;

fn hailstone(mut n: i32, arry: Option<&mut Vec<i32>>) -> i32 {
    let mut hs = 1;

    while n != 1 {
        hs += 1;
        if let Some(ref mut arry) = arry {
            arry.push(n);
        }
        n = if n & 1 != 0 { 3 * n + 1 } else { n / 2 };
    }
    if let Some(ref mut arry) = arry {
        arry.push(n);
    }
    hs
}

fn main() {
    let mut hmax = 0;
    let mut jatmax = 0;

    // Removed declaration of `n` before the loop since it is not needed
    for j in 1..100000 {
        let n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }

    // Removed unused variable `n`
    let mut arry = Vec::with_capacity(27 as usize); // Ensure arry is mutable
    let n = hailstone(27, Some(&mut arry));

    // Ensure n is large enough to avoid out-of-bounds access
    if n >= 4 {
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
        println!("Array length is too short to access elements at index n-4");
    }
    println!("Max {} at j= {}", hmax, jatmax);
}