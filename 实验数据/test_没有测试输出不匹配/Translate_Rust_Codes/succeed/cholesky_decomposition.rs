use std::alloc::{alloc, Layout};
use std::process::exit;
use std::ptr;
use std::mem;

fn cholesky(a: &[f64], n: usize) -> *mut f64 {
    let layout = Layout::array::<f64>(n * n).unwrap();
    let l = unsafe { alloc(layout) as *mut f64 };
    if l.is_null() {
        exit(1);
    }

    unsafe {
        ptr::write_bytes(l, 0, n * n);
    }

    for i in 0..n {
        for j in 0..(i + 1) {
            let mut s = 0.0;
            for k in 0..j {
                s += unsafe { *l.add(i * n + k) } * unsafe { *l.add(j * n + k) };
            }
            unsafe {
                *l.add(i * n + j) = if i == j {
                    (a[i * n + i] - s).sqrt()
                } else {
                    (1.0 / *l.add(j * n + j)) * (a[i * n + j] - s)
                };
            }
        }
    }

    l
}

fn show_matrix(a: *const f64, n: usize) {
    for i in 0..n {
        for j in 0..n {
            print!("{:.5} ", unsafe { *a.add(i * n + j) });
        }
        println!();
    }
}

fn main() {
    let n = 3;
    let m1 = [25.0, 15.0, -5.0,
              15.0, 18.0,  0.0,
              -5.0,  0.0, 11.0];
    let c1 = cholesky(&m1, n);
    show_matrix(c1, n);
    println!();

    let n = 4;
    let m2 = [18.0, 22.0,  54.0,  42.0,
              22.0, 70.0,  86.0,  62.0,
              54.0, 86.0, 174.0, 134.0,
              42.0, 62.0, 134.0, 106.0];
    let c2 = cholesky(&m2, n);
    show_matrix(c2, n);
}