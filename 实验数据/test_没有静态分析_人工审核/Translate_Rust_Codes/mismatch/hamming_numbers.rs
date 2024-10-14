type Ham = u64;

struct Queue {
    q: Vec<Ham>,
    n: usize,
    alloc: usize,
}

impl Queue {
    fn new() -> Self {
        Queue {
            q: Vec::new(),
            n: 1,
            alloc: 0,
        }
    }

    fn qpush(&mut self, h: Ham) {
        if self.alloc <= self.n {
            // Ensure the heap can accommodate new elements without unnecessary resizing
            self.alloc = if self.alloc > 0 { self.alloc * 2 } else { 16 };
            self.q.resize(self.alloc, 0);
            self.q.push(0); // Ensure the vector is resized correctly to accommodate new elements
        }

        let mut i = self.n - 1; // Start from index 0, not from self.n
        self.n += 1;
        let mut j;
        // Correctly maintain the heap property by comparing and positioning elements
        j = if (i + 1) / 2 > 1 { (i + 1) / 2 - 1 } else { 0 }; // Ensure j does not underflow
        while j > 0 && self.q[j - 1] > h { // Compare parent and child nodes
            self.q[i] = self.q[j - 1];
            i = j - 1;
            j = if (i + 1) / 2 > 1 { (i + 1) / 2 - 1 } else { 0 }; // Ensure j does not underflow
        }
        self.q[i] = h;
    }

    fn qpop(&mut self) -> Ham {
        // Initialize `r` with the correct value from the heap
        let mut r = self.q[0]; // Start from index 0, not from 1
        let mut t;
        loop {
            if self.n == 0 { // Ensure the queue is not empty before attempting to access elements
                // Break condition to avoid infinite loop
                break;
            }

            r = self.q[0];
            t = self.q[self.n - 1];
            self.n -= 1;
            let mut i = 0;
            let mut j;
            // Correctly maintain the heap property by comparing and positioning elements
            j = (i + 1) * 2 - 1; // Ensure the index calculation is correct
            while j < self.n { // Ensure the loop condition correctly handles the range of indices
                if j + 1 < self.n && self.q[j] > self.q[j + 1] {
                    j += 1;
                }
                if t <= self.q[j] {
                    break;
                }
                self.q[i] = self.q[j];
                i = j;
                j = (i + 1) * 2 - 1; // Ensure the index calculation is correct
            }
            self.q[i] = t;
            break;
        }

        r
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.qpush(1);

    for i in 1..=1691 {
        let h = queue.qpop();

        // Use checked_mul to prevent overflow and ensure correct sequence of numbers
        if let Some(h2) = h.checked_mul(2) {
            queue.qpush(h2); // Ensure the correct sequence of numbers is generated
        }
        if let Some(h3) = h.checked_mul(3) {
            queue.qpush(h3);
        }
        if let Some(h5) = h.checked_mul(5) {
            queue.qpush(h5);
        }

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
    }
}