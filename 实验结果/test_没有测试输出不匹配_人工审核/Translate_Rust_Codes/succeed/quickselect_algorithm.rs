use std::ptr;

fn qselect(v: &mut [i32], k: usize) -> i32 {
    let len = v.len();
    let mut st = 0;

    for i in 0..len - 1 {
        if v[i] <= v[len - 1] {
            v.swap(i, st);
            st += 1;
        }
    }

    v.swap(len - 1, st);

    if k == st {
        v[st]
    } else if st > k {
        qselect(&mut v[..st], k)
    } else {
        qselect(&mut v[st..], k - st)
    }
}

fn main() {
    let x = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let n = x.len();
    let mut y = vec![0; n];

    for i in 0..10 {
        unsafe {
            ptr::copy_nonoverlapping(x.as_ptr(), y.as_mut_ptr(), n);
        }
        println!("{}: {}", i, qselect(&mut y, i));
    }
}