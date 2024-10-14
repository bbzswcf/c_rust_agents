use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

// Modified: Changed the `arry` parameter to mutable to allow borrowing as mutable inside the function
fn hailstone(mut n: i32, mut arry: Option<&mut [i32]>) -> i32 {
    let mut hs = 1;
    let mut index = 0;

    while n != 1 {
        hs += 1;
        if let Some(ref mut arry) = arry {
            if index < arry.len() {
                arry[index] = n;
                index += 1;
            }
        }
        n = if n & 1 != 0 { 3 * n + 1 } else { n / 2 };
    }
    if let Some(ref mut arry) = arry {
        if index < arry.len() {
            arry[index] = n;
        }
    }
    hs
}

fn main() {
    let mut hmax = 0;
    let mut jatmax = 0;

    for j in 1..100000 {
        let n = hailstone(j, None);
        if hmax < n {
            hmax = n;
            jatmax = j;
        }
    }

    let n = hailstone(27, None);
    let layout = Layout::array::<i32>(n as usize).unwrap();
    let arry = unsafe { alloc(layout) as *mut i32 };

    let n = hailstone(27, Some(unsafe { std::slice::from_raw_parts_mut(arry, n as usize) }));

    println!(
        "[ {}, {}, {}, {}, ...., {}, {}, {}, {}] len={}",
        unsafe { *arry.offset(0) },
        unsafe { *arry.offset(1) },
        unsafe { *arry.offset(2) },
        unsafe { *arry.offset(3) },
        unsafe { *arry.offset((n - 4) as isize) },
        unsafe { *arry.offset((n - 3) as isize) },
        unsafe { *arry.offset((n - 2) as isize) },
        unsafe { *arry.offset((n - 1) as isize) },
        n
    );
    println!("Max {} at j= {}", hmax, jatmax);

    unsafe { dealloc(arry as *mut u8, layout) };
}