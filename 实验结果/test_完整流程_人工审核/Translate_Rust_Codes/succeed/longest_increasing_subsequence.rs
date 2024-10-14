#[derive(Clone)] // Added: Implement the `Clone` trait for the `Node` struct
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
        for j in i+1..len {
            if n[j].val > n[i].val && n[j].len >= n[i].len {
                n[i].next = Some(Box::new(n[j].clone())); // Fixed: Use `clone()` to avoid borrow checker error
                n[i].len = n[j].len + 1;
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