use std::ptr;

fn elem(a: &[i32], e: i32) -> bool {
    for &item in a {
        if item == e {
            return true;
        }
    }
    false
}

fn nub(a: &mut Vec<i32>) -> usize {
    let mut m = 0;
    for i in 0..a.len() {
        if !elem(&a[..m], a[i]) {
            a[m] = a[i];
            m += 1;
        }
    }
    m
}

fn nub_new(a: &[i32]) -> (Vec<i32>, usize) {
    let mut c = Vec::with_capacity(a.len());
    unsafe {
        c.set_len(a.len());
        ptr::copy_nonoverlapping(a.as_ptr(), c.as_mut_ptr(), a.len());
    }
    let m = nub(&mut c);
    let mut b = Vec::with_capacity(m);
    unsafe {
        b.set_len(m);
        ptr::copy_nonoverlapping(c.as_ptr(), b.as_mut_ptr(), m);
    }
    (b, m)
}

fn main() {
    let a = vec![1, 2, 1, 4, 5, 2, 15, 1, 3, 4];
    let (b, n) = nub_new(&a);

    for i in 0..n {
        print!("{} ", b[i]);
    }
    println!();
}