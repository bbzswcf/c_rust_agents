use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;

struct Node {
    priority: i32,
    data: String,
}

struct Heap {
    nodes: Vec<Node>,
    len: usize,
    size: usize,
}

impl Heap {
    fn push(&mut self, priority: i32, data: String) {
        if self.len + 1 >= self.size {
            self.size = if self.size > 0 { self.size * 2 } else { 4 };
            self.nodes.resize(self.size, Node { priority: 0, data: String::new() });
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        while i > 1 && self.nodes[j].priority > priority {
            self.nodes[i] = self.nodes[j].clone();
            i = j;
            j = j / 2;
        }
        self.nodes[i] = Node { priority, data };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let data = self.nodes[1].data.clone();
        self.nodes[1] = self.nodes[self.len].clone();
        self.len -= 1;

        let mut i = 1;
        while i < self.len + 1 {
            let mut k = self.len + 1;
            let mut j = 2 * i;
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                k = j + 1;
            }
            self.nodes[i] = self.nodes[k].clone();
            i = k;
        }
        Some(data)
    }
}

fn main() {
    let layout = Layout::new::<Heap>();
    let ptr = unsafe { alloc(layout) };
    let mut h: &mut Heap = unsafe { &mut *(ptr as *mut Heap) };
    h.nodes = Vec::with_capacity(4);
    h.len = 0;
    h.size = 4;

    h.push(3, String::from("Clear drains"));
    h.push(4, String::from("Feed cat"));
    h.push(5, String::from("Make tea"));
    h.push(1, String::from("Solve RC tasks"));
    h.push(2, String::from("Tax return"));

    for _ in 0..5 {
        if let Some(data) = h.pop() {
            println!("{}", data);
        }
    }

    unsafe {
        ptr::drop_in_place(h);
        dealloc(ptr, layout);
    }
}