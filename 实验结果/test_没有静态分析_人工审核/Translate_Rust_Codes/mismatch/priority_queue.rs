// Modified: Added Clone trait to Node struct
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
    fn new() -> Self {
        // Modified: Initialize nodes with an empty vector and set the initial size to 0
        let nodes = Vec::new();
        Heap {
            nodes,
            len: 0,
            size: 0,
        }
    }

    fn push(&mut self, priority: i32, data: String) {
        if self.len + 1 >= self.size {
            self.size = if self.size > 0 { self.size * 2 } else { 4 };
            // Modified: Resize the vector and then initialize the new nodes with the correct values
            self.nodes.resize(self.size, Node { priority, data: String::new() });
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        while i > 1 && self.nodes[j].priority > priority {
            // Modified: Added Clone trait to Node struct to allow moving values
            self.nodes[i] = self.nodes[j].clone();
            i = j;
            j = j / 2;
        }
        // Modified: Create a new Node instance with the provided priority and data
        self.nodes[i] = Node { priority, data };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let data = self.nodes[1].data.clone();
        // Modified: Move the last node to the root position
        self.nodes[1] = self.nodes[self.len].clone();
        self.len -= 1; // Modified: Decrement len after the node has been moved to the root position
        let mut i = 1;
        while i <= self.len / 2 { // Modified: Corrected loop condition to ensure the loop iterates correctly over the nodes
            let mut k = i; // Modified: Initialize k with i to correctly compare nodes
            let j = 2 * i;
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j + 1].priority < self.nodes[k].priority {
                k = j + 1;
            }
            // Modified: Added Clone trait to Node struct to allow moving values
            self.nodes[i] = self.nodes[k].clone();
            if i == k { // Modified: Break the loop if no swap is needed
                break;
            }
            i = k;
        }
        Some(data)
    }
}

fn main() {
    // Modified: Declared `h` as mutable to allow mutable borrowing in subsequent method calls
    let mut h = Heap::new();

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