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
        // Modified: Separated assignment from comparison to avoid type mismatch
        j = i / 2;
        while j > 0 && self.q[j] > h {
            self.q[i] = self.q[j];
            i = j;
            j = i / 2;
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

            self.n -= 1;
            t = self.q[self.n];
            let mut i = 1;
            let mut j;
            // Modified: Separated assignment from comparison to avoid type mismatch
            j = i * 2;
            while j < self.n {
                if j + 1 < self.n && self.q[j] > self.q[j + 1] {
                    j += 1;
                }
                if t <= self.q[j] {
                    break;
                }
                self.q[i] = self.q[j];
                i = j;
                j = i * 2;
            }
            self.q[i] = t;
        }

        r
    }
}

fn main() {
    let mut queue = Queue::new();
    let mut i = 1;
    let mut h;

    queue.qpush(i as Ham);
    while i <= 1691 {
        h = queue.qpop();
        // Modified: Used checked_mul to prevent integer overflow
        queue.qpush(h.checked_mul(2).unwrap_or(std::u64::MAX));
        queue.qpush(h.checked_mul(3).unwrap_or(std::u64::MAX));
        queue.qpush(h.checked_mul(5).unwrap_or(std::u64::MAX));

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
        i += 1;
    }
}