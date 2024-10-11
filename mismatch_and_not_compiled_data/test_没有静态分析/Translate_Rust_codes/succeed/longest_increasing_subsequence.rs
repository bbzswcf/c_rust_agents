// Added: Implement the `Clone` trait for the `Node` struct
#[derive(Clone)]
struct Node {
    val: i32,
    len: i32,
    next: Option<Box<Node>>,
}

fn lis(v: &[i32]) {
    let len = v.len();
    let mut n: Vec<Node> = Vec::with_capacity(len);
    for i in 0..len {
        n.push(Node { val: v[i], len: 1, next: None });
    }

    for i in (0..len).rev() {
        // Modified: Removed `mut` keyword to make `p` immutable
        let p = &mut n[i];
        for j in i + 1..len {
            // Modified: Use `split_at_mut` to safely access and modify different parts of the vector
            let (left, right) = n.split_at_mut(j);
            let p = &mut left[i];
            let n_j = &right[0];
            if n_j.val > p.val && n_j.len >= p.len {
                // Modified: Use the `clone` method after implementing the `Clone` trait
                p.next = Some(Box::new(n_j.clone()));
                p.len = n_j.len + 1;
            }
        }
    }

    let mut p = &n[0];
    for i in 1..len {
        if n[i].len > p.len {
            p = &n[i];
        }
    }

    while let Some(node) = &p.next {
        print!(" {}", p.val);
        p = node;
    }
    print!(" {}\n", p.val);
}

fn main() {
    let x = [3, 2, 6, 4, 5, 1];
    let y = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

    lis(&x);
    lis(&y);
}