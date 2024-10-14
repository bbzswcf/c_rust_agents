use std::mem;
use std::ptr;
use std::slice;

// Modified: Derived `Clone` trait for the `Edit` struct
#[derive(Debug, Clone)]
struct Edit {
    c1: u8,
    c2: u8,
    n: i32,
    next: *mut Edit,
}

fn leven(a: &str, b: &str) {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let la = a.len();
    let lb = b.len();

    let mut tbl: Vec<*mut Edit> = vec![ptr::null_mut(); la + 1];
    let mut data: Vec<Edit> = vec![Edit { c1: 0, c2: 0, n: 0, next: ptr::null_mut() }; (la + 1) * (lb + 1)];

    // Modified: Used `split_at_mut` to obtain non-overlapping mutable sub-slices of `data`
    for i in 1..=la {
        let (_, rest) = data.split_at_mut((i - 1) * (lb + 1) + lb + 1);
        tbl[i] = &mut rest[0] as *mut Edit;
    }

    for i in (0..=la).rev() {
        let aa = &a[i..];
        for j in (0..=lb).rev() {
            let bb = &b[j..];
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            // Modified: Split `data` once and reuse the resulting slices
            let (_, right) = data.split_at_mut(i * (lb + 1) + j);
            let e = &mut right[0];
            let (_, right) = data.split_at_mut((i + 1) * (lb + 1) + j + 1);
            let repl = &mut right[0];
            let (_, right) = data.split_at_mut((i + 1) * (lb + 1) + j);
            let dela = &mut right[0];
            let (_, right) = data.split_at_mut(i * (lb + 1) + j + 1);
            let delb = &mut right[0];

            // Modified: Ensure `e.c1` and `e.c2` are assigned only once per iteration
            e.c1 = *aa.first().unwrap_or(&0);
            e.c2 = *bb.first().unwrap_or(&0);

            if aa.is_empty() {
                e.next = delb as *mut Edit;
                e.n = unsafe { (*e.next).n } + 1;
                continue;
            }
            if bb.is_empty() {
                e.next = dela as *mut Edit;
                e.n = unsafe { (*e.next).n } + 1;
                continue;
            }

            e.next = repl as *mut Edit;
            if e.c1 == e.c2 {
                e.n = unsafe { (*e.next).n };
                continue;
            }

            if unsafe { (*e.next).n } > unsafe { (*delb).n } {
                e.next = delb as *mut Edit;
                e.c1 = 0; // Modified: Ensure `e.c1` is set to 0 if `delb` is chosen
            }
            if unsafe { (*e.next).n } > unsafe { (*dela).n } {
                e.next = dela as *mut Edit;
                // Removed: Unnecessary reassignment of `e.c1` and `e.c2`
            }
            e.n = unsafe { (*e.next).n } + 1;
        }
    }

    let p = &data[0];
    println!("{:?} -> {:?}: {} edits", a, b, p.n);

    let mut p = p as *const Edit;
    // Modified: Simplified loop condition to ensure `p` is not null
    while !p.is_null() {
        if unsafe { (*p).c1 } == unsafe { (*p).c2 } {
            print!("{}", unsafe { (*p).c1 } as char);
        } else {
            print!("(");
            if unsafe { (*p).c1 } != 0 {
                print!("{}", unsafe { (*p).c1 } as char);
            }
            print!(",");
            if unsafe { (*p).c2 } != 0 {
                print!("{}", unsafe { (*p).c2 } as char);
            }
            print!(")");
        }
        p = unsafe { (*p).next };
    }
    println!();
}

fn main() {
    leven("raisethysword", "rosettacode");
}