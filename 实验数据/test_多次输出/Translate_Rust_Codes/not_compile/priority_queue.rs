use std::ptr;

#[derive(Debug)]
struct Node {
    priority: i32,
    data: String,
}

#[derive(Debug)]
struct Heap {
    nodes: Vec<Node>,
    len: usize,
    size: usize,
}

impl Heap {
    fn new() -> Self {
        Heap {
            nodes: Vec::new(),
            len: 0,
            size: 0,
        }
    }

    fn push(&mut self, priority: i32, data: String) {
        if self.len + 1 >= self.size {
            self.size = if self.size == 0 { 4 } else { self.size * 2 };
            self.nodes.resize_with(self.size, Default::default);
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