use std::alloc::{alloc, realloc, Layout};
use std::ptr;
use std::mem;

struct Node {
    priority: i32,
    data: String,
}

// Modified: Implement the Clone trait for Node to allow cloning
impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            priority: self.priority,
            data: self.data.clone(),
        }
    }
}

struct Heap {
    nodes: *mut Node,
    len: usize,
    size: usize,
}

impl Heap {
    fn new() -> Self {
        Heap {
            nodes: ptr::null_mut(),
            len: 0,
            size: 0,
        }
    }

    fn push(&mut self, priority: i32, data: String) {
        if self.len + 1 >= self.size {
            self.size = if self.size > 0 { self.size * 2 } else { 4 };
            let layout = Layout::array::<Node>(self.size).unwrap();
            // Modified: Correctly calculate old_layout based on current size
            let old_layout = Layout::array::<Node>(self.size / 2).unwrap();
            let old_nodes = self.nodes;
            self.nodes = unsafe { realloc(self.nodes as *mut u8, old_layout, layout.size()) as *mut Node };
            // Handle potential memory leak if realloc returns null
            if self.nodes.is_null() {
                // Free the old memory block manually
                unsafe { std::alloc::dealloc(old_nodes as *mut u8, old_layout); }
                panic!("Failed to reallocate memory");
            }
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        while i > 1 && unsafe { (*self.nodes.offset(j as isize)).priority } > priority {
            // Modified: Ensure memory is initialized before cloning
            unsafe {
                ptr::write(self.nodes.offset(i as isize), Node { priority: 0, data: String::new(), });
                *self.nodes.offset(i as isize) = (*self.nodes.offset(j as isize)).clone();
            }
            i = j;
            j = j / 2;
        }
        unsafe {
            (*self.nodes.offset(i as isize)).priority = priority;
            (*self.nodes.offset(i as isize)).data = data;
        }
        self.len += 1;
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        // Modified: Ensure self.nodes is not null before dereferencing
        if self.nodes.is_null() {
            return None;
        }
        let data = unsafe { (*self.nodes.offset(1)).data.clone() };

        // Modified: Clone the Node instead of moving it
        unsafe { *self.nodes.offset(1) = (*self.nodes.offset(self.len as isize)).clone() };

        self.len -= 1;

        let mut i = 1;
        // Modified: Correct loop condition to ensure all elements are iterated
        while i * 2 <= self.len {
            let mut k = i * 2;
            if k + 1 <= self.len && unsafe { (*self.nodes.offset(k as isize + 1)).priority } < unsafe { (*self.nodes.offset(k as isize)).priority } {
                k += 1;
            }
            if unsafe { (*self.nodes.offset(i as isize)).priority } <= unsafe { (*self.nodes.offset(k as isize)).priority } {
                break;
            }
            // Modified: Clone the Node instead of moving it
            unsafe { *self.nodes.offset(i as isize) = (*self.nodes.offset(k as isize)).clone() };
            i = k;
        }
        Some(data)
    }
}

// Modified: Implement the Drop trait to handle memory deallocation
impl Drop for Heap {
    fn drop(&mut self) {
        if !self.nodes.is_null() {
            let layout = Layout::array::<Node>(self.size).unwrap();
            // Modified: Correctly use dealloc with layout
            unsafe {
                std::alloc::dealloc(self.nodes as *mut u8, layout);
            }
        }
    }
}

fn main() {
    let mut h = Heap::new();
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