use std::alloc;
use std::process;
use std::ptr;
use std::mem;

fn cholesky(a: &[f64], n: usize) -> Vec<f64> {
    let layout = alloc::Layout::from_size_align(n * n * mem::size_of::<f64>(), mem::align_of::<f64>()).unwrap();
    let raw_ptr = unsafe { alloc::alloc(layout) };
    if raw_ptr.is_null() {
        process::exit(1);
    }
    let mut l = unsafe { Vec::from_raw_parts(raw_ptr as *mut f64, n * n, n * n) };

    for i in 0..n {
        for j in 0..=i {
            let mut s = 0.0;
            for k in 0..j {
                s += l[i * n + k] * l[j * n + k];
            }
            l[i * n + j] = if i == j {
                (a[i * n + i] - s).sqrt()
            } else {
                (1.0 / l[j * n + j]) * (a[i * n + j] - s)
            };
        }
    }

    l
}

fn show_matrix(a: &[f64], n: usize) {
    for i in 0..n {
        for j in 0..n {
            print!("{:.5} ", a[i * n + j]);
        }
        println!();
    }
}

fn main() {
    let n = 3;
    let m1 = vec![25.0, 15.0, -5.0,
                  15.0, 18.0,  0.0,
                  -5.0,  0.0, 11.0];
    let c1 = cholesky(&m1, n);
    show_matrix(&c1, n);
    println!();

    let n = 4;
    let m2 = vec![18.0, 22.0,  54.0,  42.0,
                  22.0, 70.0,  86.0,  62.0,
                  54.0, 86.0, 174.0, 134.0,
                  42.0, 62.0, 134.0, 106.0];
    let c2 = cholesky(&m2, n);
    show_matrix(&c2, n);
}