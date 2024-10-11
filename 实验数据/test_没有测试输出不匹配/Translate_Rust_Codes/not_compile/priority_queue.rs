// Added: Implement the Clone trait for the Node struct
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
            // Modified: Use resize to allocate space instead of reserve
            self.nodes.resize(self.size, Node { priority: 0, data: String::new() });
        }
        self.len += 1;
        let mut i = self.len;
        let mut j = i / 2;
        while i > 1 && self.nodes[j].priority > priority {
            // Removed: Redundant bounds check
            self.nodes.swap(i, j);
            i = j;
            j = j / 2;
        }
        // Modified: Ensure the dummy element at index 0 is not overwritten
        self.nodes[i] = Node { priority, data };
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        // Modified: Ensure that the index `1` is within bounds before accessing `self.nodes[1]`
        if self.len > 0 {
            let data = self.nodes[1].data.clone();
            // Modified: Ensure self.len is within bounds
            if self.len >= 1 {
                self.nodes.swap(1, self.len);
            }
            self.len -= 1;

            let mut i = 1;
            // Modified: Correct loop condition to ensure it iterates within valid range
            while i <= self.len {
                // Modified: Initialize `k` with `i` instead of `self.len + 1`
                let mut k = i;
                // Removed: Variable does not need to be mutable
                let j = 2 * i;
                if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                    k = j;
                }
                if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                    k = j + 1;
                }
                // Removed: Redundant bounds check
                if i != k {
                    self.nodes.swap(i, k);
                }
                i = k;
            }
            Some(data)
        } else {
            None
        }
    }
}

fn main() {
    // Modified: Use Box to manage heap-allocated data safely
    let mut h = Box::new(Heap {
        nodes: vec![Node { priority: 0, data: String::new() }], // Added: Dummy element at index 0
        len: 0, // Modified: Initialize len to 0 to correctly reflect the number of actual elements
        size: 4, // Modified: Initialize size to 4 to allocate enough space for additional elements
    });

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