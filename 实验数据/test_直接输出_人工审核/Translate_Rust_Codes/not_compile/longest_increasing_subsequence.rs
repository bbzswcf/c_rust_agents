use std::alloc::{alloc, Layout};
use std::mem;

struct Node {
    val: i32,
    len: i32,
    next: Option<Box<Node>>,
}

fn lis(v: &[i32]) {
    let len = v.len();
    let mut nodes: Vec<Node> = Vec::with_capacity(len);

    unsafe {
        let layout = Layout::array::<Node>(len).unwrap();
        let ptr = alloc(layout) as *mut Node;
        nodes.set_len(len);
        for i in 0..len {
            (*ptr.add(i)).val = v[i];
            (*ptr.add(i)).len = 1;
            (*ptr.add(i)).next = None;
        }

        for i in (0..len).rev() {
            let mut p = ptr.add(i);
            for j in i + 1..len {
                let q = ptr.add(j);
                if (*q).val > (*p).val && (*q).len >= (*p).len {
                    (*p).next = Some(Box::new(*q));
                    (*p).len = (*q).len + 1;
                }
            }
        }

        let mut p = ptr;
        for i in 0..len {
            if (*ptr.add(i)).len > (*p).len {
                p = ptr.add(i);
            }
        }

        while let Some(node) = &(*p).next {
            print!(" {}", (*p).val);
            p = node.as_ref();
        }
        print!(" {}", (*p).val);
        println!();
    }
}

fn main() {
    let x = [3, 2, 6, 4, 5, 1];
    let y = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];

    lis(&x);
    lis(&y);
}