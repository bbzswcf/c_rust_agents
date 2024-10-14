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

    match k.cmp(&st) {
        std::cmp::Ordering::Equal => v[st],
        std::cmp::Ordering::Less => qselect(&mut v[..st], k),
        std::cmp::Ordering::Greater => qselect(&mut v[st..], k - st),
    }
}

fn main() {
    let x = [9, 8, 7, 6, 5, 0, 1, 2, 3, 4];
    let mut y = [0; 10];

    for i in 0..10 {
        y.copy_from_slice(&x); // qselect modifies array
        println!("{}: {}", i, qselect(&mut y, i));
    }
}