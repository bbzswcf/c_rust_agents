struct Node {
    val: i32,
    len: i32,
    next: Option<Box<Node>>,
}

fn lis(v: &[i32]) {
    let len = v.len();
    let mut n: Vec<Node> = Vec::with_capacity(len);

    // Initialize the Node vector with each node having a subsequence length of 1
    for i in 0..len {
        n.push(Node {
            val: v[i],
            len: 1,
            next: None,
        });
    }

    // Update the subsequence lengths and next pointers correctly
    for i in (0..len).rev() {
        let node = &mut n[i];

        for j in (i + 1)..len {
            let p = &n[j];
            if p.val > node.val && p.len + 1 > node.len {
                // Update the next pointer to point to the correct subsequent node
                node.next = Some(Box::new(Node {
                    val: p.val,
                    len: p.len,
                    // Modified: Manually clone the Node inside the Box and wrap it back in a Box
                    next: p.next.as_ref().map(|boxed_node| Box::new(*boxed_node.clone())),
                }));
                // Update the length to reflect the correct subsequence length
                node.len = p.len + 1;
            }
        }
    }

    // Find the node with the maximum subsequence length
    let mut max_len_node = &n[0];
    for i in 0..len {
        if n[i].len > max_len_node.len {
            max_len_node = &n[i];
        }
    }

    // Traverse the subsequence in the correct order
    let mut current = Some(max_len_node);
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