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
    fn new(size: usize) -> Self {
        Heap {
            nodes: Vec::with_capacity(size),
            len: 0,
            size,
        }
    }

    fn push(&mut self, priority: i32, data: String) {
        if self.len + 1 >= self.size {
            self.size = if self.size > 0 { self.size * 2 } else { 4 };
            self.nodes.resize(self.size, Node { priority: 0, data: String::new() });
        }
        let mut i = self.len + 1;
        let mut j = i / 2;
        // Modified: Ensure that `i` and `j` are within bounds before accessing `self.nodes`
        while i > 1 && j > 0 && self.nodes[j - 1].priority > priority {
            self.nodes[i - 1] = self.nodes[j - 1].clone();
            i = j;
            j = j / 2;
        }
        self.nodes[i - 1] = Node { priority, data };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<String> {
        if self.len == 0 {
            return None;
        }
        let data = self.nodes[0].data.clone();
        self.nodes[0] = self.nodes[self.len - 1].clone();
        self.len -= 1;

        let mut i = 1;
        // Modified: Ensure that the loop condition correctly handles the heap structure
        while i <= self.len / 2 {
            let mut k = i;
            let j = 2 * i;
            if j <= self.len && self.nodes[j - 1].priority < self.nodes[k - 1].priority {
                k = j;
            }
            if j + 1 <= self.len && self.nodes[j].priority < self.nodes[k - 1].priority {
                k = j + 1;
            }
            if k == i {
                break;
            }
            self.nodes.swap(i - 1, k - 1);
            i = k;
        }
        Some(data)
    }
}

fn main() {
    let mut h = Heap::new(4);

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