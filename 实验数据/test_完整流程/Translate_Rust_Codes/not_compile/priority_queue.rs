use std::alloc::{alloc_zeroed, dealloc, Layout};

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
        let mut i = self.len; // Correctly set the index for the new node
        let mut j = (i + 1) / 2; // Adjust parent index calculation
        while i > 0 && self.nodes[j].priority > priority { // Correct loop condition
            self.nodes.swap(i, j); // Use `swap` to maintain heap property
            i = j;
            j = (i + 1) / 2;
        }
        self.nodes[i] = Node { priority, data }; // Insert the new node at the correct position
        self.len += 1; // Increment `self.len` after inserting the new node
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let data = std::mem::take(&mut self.nodes[1].data);
        let (left, right) = self.nodes.split_at_mut(self.len); // Fix mutable borrow error
        std::mem::swap(&mut left[1], &mut right[0]); // Correctly swap the last element with the root
        self.len -= 1; // Decrement `self.len` after the swap operation
        let mut i = 1; // Start from the root of the heap
        while i <= self.len / 2 { // Correct loop condition to maintain heap property
            let mut k = i;
            let j = 2 * i;
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                k = j + 1;
            }
            if k != i {
                self.nodes.swap(i, k);
                i = k;
            } else {
                break;
            }
        }
        Some(data)
    }
}

fn main() {
    let layout = Layout::new::<Heap>();
    let ptr = unsafe { alloc_zeroed(layout) };
    let h: &mut Heap = unsafe { &mut *(ptr as *mut Heap) };
    h.nodes = vec![Node { priority: 0, data: String::new() }; 5]; // Initialize with an empty node at index 0
    h.size = 5;
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

    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}