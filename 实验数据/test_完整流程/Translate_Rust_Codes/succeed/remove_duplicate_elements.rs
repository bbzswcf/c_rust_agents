use std::ptr;

fn elem(a: &[i32], e: i32) -> bool {
    for &item in a {
        if item == e {
            return true;
        }
    }
    false
}

fn nub(a: &mut [i32]) -> usize {
    let mut m = 0;
    for i in 0..a.len() {
        if !elem(&a[..m], a[i]) {
            a[m] = a[i];
            m += 1;
        }
    }
    m
}

fn nub_new(a: &[i32]) -> Vec<i32> {
    let mut c = Vec::with_capacity(a.len());
    // Initialize the buffer before setting its length
    c.extend(a.iter().cloned());
    let m = nub(&mut c);
    let mut b = Vec::with_capacity(m);
    // Initialize the buffer before setting its length
    b.extend(c.iter().take(m).cloned());
    b
}

fn main() {
    let a = vec![1, 2, 1, 4, 5, 2, 15, 1, 3, 4];
    let b = nub_new(&a);

    for &item in &b {
        print!("{} ", item);
    }
    // Removed empty string literal
    println!();
}