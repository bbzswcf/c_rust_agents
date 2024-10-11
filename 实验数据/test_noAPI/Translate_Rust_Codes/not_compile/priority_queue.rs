#[derive(Clone)]
struct Node {
    priority: i32,
    data: String,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            priority: 0,
            data: String::new(),
        }
    }
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
            self.nodes.resize_with(self.size, Default::default); // Ensure new elements are initialized with default value
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        while i > 1 && j < self.len && self.nodes[j].priority > priority { // Ensure j is within bounds
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
        let data = std::mem::replace(&mut self.nodes[1].data, String::new());

        // Ensure self.len is a valid index before accessing self.nodes[self.len]
        if self.len > 0 {
            self.nodes[1] = self.nodes[self.len].clone();
        }

        self.len -= 1;

        let mut i = 1;
        while i <= self.len { // Ensure i is within bounds
            let mut k = i;
            let j = 2 * i;
            // Ensure j is within bounds before accessing self.nodes[j]
            if j <= self.len && self.nodes[j].priority < self.nodes[k].priority {
                k = j;
            }
            // Ensure j + 1 is within bounds before accessing self.nodes[j + 1]
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
    let mut h = Heap {
        nodes: Vec::with_capacity(4),
        len: 0,
        size: 4,
    };
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