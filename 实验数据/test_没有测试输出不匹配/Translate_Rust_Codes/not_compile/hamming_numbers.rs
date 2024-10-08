use std::alloc::{self, Layout};
use std::ptr;

type Ham = u64;

struct Queue {
    alloc: usize,
    n: usize,
    q: *mut Ham,
}

impl Queue {
    fn new() -> Self {
        let initial_alloc = 16;
        let layout = Layout::array::<Ham>(initial_alloc).unwrap();
        let q = unsafe { alloc::alloc(layout) as *mut Ham };
        Queue {
            alloc: initial_alloc,
            n: 1,
            q,
        }
    }

    fn qpush(&mut self, h: Ham) {
        if self.alloc <= self.n {
            // Ensure that the multiplication does not overflow
            let new_alloc = self.alloc.checked_mul(2).unwrap_or(usize::MAX);
            if new_alloc == usize::MAX {
                panic!("Multiplication overflow during reallocation");
            }
            let layout = Layout::array::<Ham>(new_alloc).unwrap();
            self.q = unsafe { alloc::realloc(self.q as *mut u8, layout, new_alloc * std::mem::size_of::<Ham>()) as *mut Ham };
            self.alloc = new_alloc;
        }

        let mut i: usize = self.n;
        self.n += 1;
        let mut j;
        j = i / 2;
        while j > 0 && unsafe { *self.q.offset(j as isize) } > h {
            unsafe { *self.q.offset(i as isize) = *self.q.offset(j as isize) };
            i = j;
            j = i / 2;
        }
        unsafe { *self.q.offset(i as isize) = h };
    }

    fn qpop(&mut self) -> Ham {
        let mut r;
        let mut t;
        loop {
            r = unsafe { *self.q.offset(1) };
            if self.n <= 1 {
                break;
            }

            self.n = self.n.checked_sub(1).unwrap_or(0);
            t = unsafe { *self.q.offset(self.n as isize) };
            let mut i: usize = 1;
            let mut j;
            // Ensure that the multiplication does not overflow
            j = i.checked_mul(2).unwrap_or(usize::MAX);
            while j < self.n {
                // Ensure that the addition does not overflow
                if j.checked_add(1).unwrap_or(usize::MAX) < self.n && unsafe { *self.q.offset(j as isize) } > unsafe { *self.q.offset((j.checked_add(1).unwrap_or(usize::MAX)) as isize) } {
                    j = j.checked_add(1).unwrap_or(usize::MAX);
                }
                if t <= unsafe { *self.q.offset(j as isize) } {
                    break;
                }
                unsafe { *self.q.offset(i as isize) = *self.q.offset(j as isize) };
                i = j;
                // Ensure that the multiplication does not overflow
                j = i.checked_mul(2).unwrap_or(usize::MAX);
            }
            unsafe { *self.q.offset(i as isize) = t };
        }

        r
    }
}

fn main() {
    let mut queue = Queue::new();
    queue.qpush(1);

    for i in 1..=1691 {
        let h = queue.qpop();
        // Ensure that the multiplication does not overflow
        if let Some(result) = h.checked_mul(2) {
            queue.qpush(result);
        } else {
            panic!("Multiplication overflow during element pushing");
        }
        if let Some(result) = h.checked_mul(3) {
            queue.qpush(result);
        } else {
            panic!("Multiplication overflow during element pushing");
        }
        if let Some(result) = h.checked_mul(5) {
            queue.qpush(result);
        } else {
            panic!("Multiplication overflow during element pushing");
        }

        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
    }
}