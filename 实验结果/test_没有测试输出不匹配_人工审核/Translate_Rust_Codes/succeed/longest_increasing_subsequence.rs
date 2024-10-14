#[derive(Clone)] // Ensure Node implements Clone trait
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
        // Store necessary values from `n[i]` in local variables before entering the mutable borrow scope
        let n_i_val = n[i].val;
        let mut n_i_len = n[i].len;
        let mut n_i_next = n[i].next.clone();

        for p in n.iter_mut().skip(i + 1) {
            // Use `n_i_val` and `n_i_len` within the loop
            if p.val > n_i_val && p.len >= n_i_len {
                // Use a temporary variable to hold the new node before assigning it back to `n[i]`
                n_i_next = Some(Box::new(p.clone()));
                n_i_len = p.len + 1;
            }
        }

        // Update `n[i]` with the new values after the loop
        n[i].next = n_i_next;
        n[i].len = n_i_len;
    }

    // Use a temporary variable to hold the new reference and then assign it back to p
    let mut p = &n[0];
    for i in 1..len {
        if n[i].len > p.len {
            // Update p with a temporary variable
            let new_p = &n[i];
            p = new_p;
        }
    }

    let mut current = Some(p);
    while let Some(node) = current {
        print!(" {}", node.val);
        current = node.next.as_deref();
    }
    println!();
}

fn main() {
    let x = [3, 2, 6, 4, 5, 1];
    let y = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

    lis(&x);
    lis(&y);
}