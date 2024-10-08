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
        for p in n.iter_mut().skip(i + 1) {
            if p.val > n[i].val && p.len >= n[i].len {
                // 使用 std::mem::replace 更新 next
                n[i].next = std::mem::replace(&mut p.next, None);
                n[i].len = p.len + 1;
            }
        }
    }

    // 找到最长的递增子序列
    let mut p = Box::new(&mut n[0]);
    for i in 1..len {
        if n[i].len > p.len {
            p = Box::new(&mut n[i]);
        }
    }

    // 打印递增子序列
    let mut current = Some(p);
    while let Some(node) = current {
        print!(" {}", node.val);
        // 修改: 使用 as_mut 和 map 来正确处理 Option<Box<Node>>
        current = node.next.as_mut().map(|next| &mut **next);
    }
    println!();
}

fn main() {
    let x = [3, 2, 6, 4, 5, 1];
    // 测试 x 和 y
    let y = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

    lis(&x);
    lis(&y);
}