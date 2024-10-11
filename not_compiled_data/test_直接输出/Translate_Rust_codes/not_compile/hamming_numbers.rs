use std::mem;

type Ham = u64;

struct Queue {
    alloc: usize,
    n: usize,
    q: Vec<Ham>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            alloc: 0,
            n: 1,
            q: Vec::new(),
        }
    }

    fn qpush(&mut self, h: Ham) {
        if self.alloc <= self.n {
            self.alloc = if self.alloc > 0 { self.alloc * 2 } else { 16 };
            self.q.resize(self.alloc, 0);
        }

        let mut i = self.n;
        self.n += 1;
        let mut j;
        while (j = i / 2) > 0 && self.q[j] > h {
            self.q[i] = self.q[j];
            i = j;
        }
        self.q[i] = h;
    }

    fn qpop(&mut self) -> Ham {
        let mut r;
        let mut t;
        loop {
            r = self.q[1];
            if self.n <= 1 || r != self.q[1] {
                break;
            }

            t = self.q[self.n - 1];
            self.n -= 1;
            let mut i = 1;
            let mut j;
            while (j = i * 2) < self.n {
                if j + 1 < self.n && self.q[j] > self.q[j + 1] {
                    j += 1;
                }
                if t <= self.q[j] {
                    break;
                }
                self.q[i] = self.q[j];
                i = j;
            }
            self.q[i] = t;
        }

        r
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.qpush(1);

    for i in 1..=1691 {
        let h = queue.qpop();
        queue.qpush(h * 2);
        queue.qpush(h * 3);
        queue.qpush(h * 5);

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
    }
}