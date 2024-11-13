use std::alloc::{alloc, Layout};
use std::mem;
use std::ptr;
#[derive(Clone)]
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
        self.nodes[i].priority = priority;
        self.nodes[i].data = data;
        self.len += 1;
    }
    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let data = mem::replace(&mut self.nodes[1].data, String::new());
        self.nodes[1] = self.nodes[self.len];
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
    let h: &mut Heap = unsafe { &mut *(ptr as *mut Heap) };
    h.nodes = Vec::with_capacity(4);
    h.len = 0;
    h.size = 4;
    h.push(3, "Clear drains".to_string());
    h.push(4, "Feed cat".to_string());
    h.push(5, "Make tea".to_string());
    h.push(1, "Solve RC tasks".to_string());
    h.push(2, "Tax return".to_string());
    for _ in 0..5 {
        if let Some(data) = h.pop() {
            println!("{}", data);
        }
    }
    unsafe {
        ptr::drop_in_place(h);
        std::alloc::dealloc(ptr, layout);
    }
}