use std::alloc::{alloc_zeroed, Layout};
use std::process::exit;
use std::ptr;
use std::mem;

fn cholesky(a: &[f64], n: usize) -> *mut f64 {
    unsafe {
        let layout = Layout::array::<f64>(n * n).unwrap();
        let l = alloc_zeroed(layout) as *mut f64;
        if l.is_null() {
            exit(1);
        }

        for i in 0..n {
            for j in 0..=i {
                let mut s = 0.0;
                for k in 0..j {
                    s += *l.add(i * n + k) * *l.add(j * n + k);
                }
                *l.add(i * n + j) = if i == j {
                    (a[i * n + i] - s).sqrt()
                } else {
                    (1.0 / *l.add(j * n + j)) * (a[i * n + j] - s)
                };
            }
        }

        l
    }
}

fn show_matrix(a: *const f64, n: usize) {
    unsafe {
        for i in 0..n {
            for j in 0..n {
                print!("{:.5} ", *a.add(i * n + j));
            }
            println!();
        }
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
    unsafe {
        let layout = Layout::array::<f64>(n * n).unwrap();
        ptr::drop_in_place(c1);
        std::alloc::dealloc(c1 as *mut u8, layout);
    }

    let n = 4;
    let m2 = [18.0, 22.0,  54.0,  42.0,
              22.0, 70.0,  86.0,  62.0,
              54.0, 86.0, 174.0, 134.0,
              42.0, 62.0, 134.0, 106.0];
    let c2 = cholesky(&m2, n);
    show_matrix(c2, n);
    unsafe {
        let layout = Layout::array::<f64>(n * n).unwrap();
        ptr::drop_in_place(c2);
        std::alloc::dealloc(c2 as *mut u8, layout);
    }
}