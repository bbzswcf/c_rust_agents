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
            // Modified: The `resize` method now works because `Node` implements `Clone`
            self.nodes.resize(self.size, Node { priority: 0, data: String::new() });
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        // Modified: Ensure that `j` is within valid bounds before accessing `self.nodes[j]`
        while i > 1 && j > 0 && j <= self.len && j < self.nodes.len() && self.nodes[j].priority > priority {
            // Modified: The `clone` method is now available because `Node` implements `Clone`
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
        // Modified: Ensure that the dummy node at index 0 is not accessed or modified
        self.nodes[1] = self.nodes[self.len].clone();
        self.nodes.swap(1, self.len);
        self.len -= 1;

        let mut i = 1;
        // Modified: Ensure `i` does not exceed `self.len` to prevent out-of-bounds access
        while i <= self.len {
            let mut k = self.len + 1;
            let mut j = 2 * i;
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                k = j + 1;
            }
            // Modified: Ensure `k` is within bounds before accessing `self.nodes[k]`
            if k < self.nodes.len() {
                // Modified: Use `split_at_mut` to avoid borrow checker error
                let (left, right) = self.nodes.split_at_mut(i);
                let temp = std::mem::replace(&mut right[0], left[k - i].clone());
                self.nodes[k] = temp;
                i = k;
            } else {
                break;
            }
        }
        Some(data)
    }
}

fn main() {
    let mut h = Heap {
        nodes: Vec::with_capacity(4),
        len: 0,
        size: 4,
    };
    // Modified: Removed the unnecessary dummy node at index 0
    // h.nodes.push(Node { priority: 0, data: String::new() }); // Dummy node at index 0
    // Modified: The length should be initialized to 0 since the dummy node at index 0 should not be included in the length calculation
    h.len = 0;

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
}