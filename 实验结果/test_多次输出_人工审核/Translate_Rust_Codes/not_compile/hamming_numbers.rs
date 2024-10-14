use std::mem;

type Ham = u64;

struct MinHeap {
    data: Vec<Ham>,
    size: usize,
    capacity: usize,
}

impl MinHeap {
    fn new() -> Self {
        Self {
            data: Vec::with_capacity(16),
            size: 0,
            capacity: 16,
        }
    }

    fn push(&mut self, h: Ham) {
        if self.size >= self.capacity {
            self.capacity = if self.capacity == 0 { 16 } else { self.capacity * 2 };
            self.data.resize(self.capacity, 0);
        }

        let mut i = self.size;
        self.size += 1;
        while i > 0 {
            let j = (i - 1) / 2;
            if self.data[j] <= h {
                break;
            }
            self.data[i] = self.data[j];
            i = j;
        }
        self.data[i] = h;
    }

    fn pop(&mut self) -> Ham {
        let mut r = self.data[0];
        let mut i = 0;

        while self.size > 1 && r == self.data[0] {
            let mut t = self.data[self.size - 1];
            self.size -= 1;

            while (i * 2 + 1) < self.size {
                let mut j = i * 2 + 1;
                if j + 1 < self.size && self.data[j] > self.data[j + 1] {
                    j += 1;
                }
                if t <= self.data[j] {
                    break;
                }
                self.data[i] = self.data[j];
                i = j;
            }
            self.data[i] = t;
            r = self.data[0];
        }

        r
    }
}

fn main() {
    let mut heap = MinHeap::new();
    heap.push(1);

    for i in 1..=1691 {
        let h = heap.pop();
        heap.push(h * 2);
        heap.push(h * 3);
        heap.push(h * 5);

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
    }
}