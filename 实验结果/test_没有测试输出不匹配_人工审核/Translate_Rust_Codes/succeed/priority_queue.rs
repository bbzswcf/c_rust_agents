use std::alloc::{alloc, Layout};
use std::ptr::copy_nonoverlapping;
use std::mem::{size_of, zeroed};

// Modified: Annotate the `Node` struct with `#[derive(Clone)]` to automatically derive the `Clone` trait
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
            // Modified: The `resize` method requires the elements of the vector to implement the `Clone` trait
            self.nodes.resize(self.size, Node { priority: 0, data: String::new() });
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        while i > 1 && self.nodes[j].priority > priority {
            // Modified: Ensure that the `Node` struct implements the `Clone` trait
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
        // Modified: Ensure that the `Node` struct implements the `Clone` trait
        self.nodes[1] = self.nodes[self.len].clone();
        self.len -= 1;
        let mut i = 1;
        while i < self.len + 1 {
            let mut k = self.len + 1;
            let j = 2 * i;
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                k = j + 1;
            }
            // Modified: Ensure that the `Node` struct implements the `Clone` trait
            self.nodes[i] = self.nodes[k].clone();
            i = k;
        }
        Some(data)
    }
}

fn main() {
    let mut h = Heap {
        nodes: Vec::with_capacity(1),
        len: 0,
        size: 1,
    };
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
}