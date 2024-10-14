use std::mem;

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

fn nub_new(a: &[i32]) -> (Vec<i32>, usize) {
    let mut c = a.to_vec();
    let m = nub(&mut c);
    let mut b = Vec::with_capacity(m);
    b.extend_from_slice(&c[..m]);
    (b, m)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = vec![1, 2, 1, 4, 5, 2, 15, 1, 3, 4];
    let (b, n) = nub_new(&a);

    for &item in &b[..n] {
        print!("{} ", item);
    }
    println!();

    Ok(())
}