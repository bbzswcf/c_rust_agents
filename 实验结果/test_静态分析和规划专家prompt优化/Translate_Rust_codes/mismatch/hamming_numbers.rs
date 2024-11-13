use std::alloc::{realloc, Layout};
use std::ptr;

type Ham = u64;
static mut ALLOC: usize = 0;
static mut N: usize = 1;
static mut Q: *mut Ham = ptr::null_mut();

unsafe fn qpush(h: Ham) {
    let mut i: usize;
    let mut j: usize;
    if ALLOC <= N {
        ALLOC = if ALLOC > 0 { ALLOC * 2 } else { 16 };
        let layout = Layout::array::<Ham>(ALLOC).unwrap();
        Q = realloc(Q as *mut u8, layout, ALLOC * std::mem::size_of::<Ham>()) as *mut Ham;
    }
    i = N;
    N += 1;
    while { j = i / 2; j > 0 && *Q.offset((j - 1) as isize) > h } {
        *Q.offset(i as isize) = *Q.offset((j - 1) as isize);
        i = j - 1;
    }
    *Q.offset(i as isize) = h;
}

unsafe fn qpop() -> Ham {
    let mut i: usize;
    let mut j: usize;
    let mut r: Ham;
    let mut t: Ham;
    loop {
        r = *Q.offset(0);
        if N <= 1 || r != *Q.offset(0) {
            break;
        }
        t = *Q.offset((N - 1) as isize);
        N -= 1;
        i = 0;
        while { j = i * 2 + 1; j < N } {
            if j + 1 < N && *Q.offset(j as isize) > *Q.offset((j + 1) as isize) {
                j += 1;
            }
            if t <= *Q.offset(j as isize) {
                break;
            }
            *Q.offset(i as isize) = *Q.offset(j as isize);
            i = j;
        }
        *Q.offset(i as isize) = t;
    }
    r
}

fn main() {
    unsafe {
        let mut i: usize = 1;
        let mut h: Ham;
        qpush(1); // Initialize with 1
        while i <= 1691 {
            h = qpop();
            if let Some(val) = h.checked_mul(2) { qpush(val); }
            if let Some(val) = h.checked_mul(3) { qpush(val); }
            if let Some(val) = h.checked_mul(5) { qpush(val); }
            if i <= 20 || i == 1691 {
                println!("{:6}: {}", i, h);
            }
            i += 1;
        }
    }
}