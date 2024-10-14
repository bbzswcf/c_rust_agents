#[derive(Clone)]
struct Node {
    val: i32,
    len: i32,
    next: Option<Box<Node>>,
}

fn lis(v: &[i32]) {
    let len = v.len();
    let mut nodes: Vec<Node> = (0..len)
        .map(|i| Node {
            val: v[i],
            len: 1,
            next: None,
        })
        .collect();

    for i in (0..len).rev() {
        for j in (i + 1)..len {
            if nodes[j].val > nodes[i].val && nodes[j].len >= nodes[i].len {
                nodes[i].next = Some(Box::new(nodes[j].clone()));
                nodes[i].len = nodes[j].len + 1;
            }
        }
    }

    let mut longest = &nodes[0];
    for i in 1..len {
        if nodes[i].len > longest.len {
            longest = &nodes[i];
        }
    }

    let mut current = longest;
    while let Some(next) = &current.next {
        print!(" {}", current.val);
        current = next;
    }
    println!(" {}", current.val);
}

fn main() {
    let x = [3, 2, 6, 4, 5, 1];
    let y = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

    lis(&x);
    lis(&y);
}