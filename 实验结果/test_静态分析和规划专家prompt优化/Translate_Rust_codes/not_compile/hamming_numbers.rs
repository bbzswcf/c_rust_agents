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
        Queue {
            alloc: 0,
            n: 1,
            q: ptr::null_mut(),
        }
    }
    fn qpush(&mut self, h: Ham) {
        let mut i;
        let mut j;
        if self.alloc <= self.n {
            self.alloc = if self.alloc > 0 { self.alloc * 2 } else { 16 };
            let layout = Layout::array::<Ham>(self.alloc).unwrap();
            self.q = unsafe { alloc::realloc(self.q as *mut u8, layout, self.alloc * std::mem::size_of::<Ham>()) as *mut Ham };
        }
        i = self.n;
        self.n += 1;
        while (j = i / 2) > 0 && unsafe { *self.q.offset(j as isize) } > h {
            unsafe { *self.q.offset(i as isize) = *self.q.offset(j as isize) };
            i = j;
        }
        unsafe { *self.q.offset(i as isize) = h };
    }
    fn qpop(&mut self) -> Ham {
        let mut i;
        let mut j: usize;
        let mut r;
        let mut t;
        loop {
            r = unsafe { *self.q.offset(1) };
            if self.n <= 1 || r != unsafe { *self.q.offset(1) } {
                break;
            }
            t = unsafe { *self.q.offset((self.n - 1) as isize) };
            self.n -= 1;
            i = 1;
            let mut j = i * 2;
            while j < self.n {
                if j + 1 < self.n && unsafe { *self.q.offset(j as isize) } > unsafe { *self.q.offset((j + 1) as isize) } {
                    j += 1;
                }
                if t <= unsafe { *self.q.offset(j as isize) } {
                    break;
                }
                unsafe { *self.q.offset(i as isize) = *self.q.offset(j as isize) };
                i = j;
            }
            unsafe { *self.q.offset(i as isize) = t };
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
        queue.qpush(h * 2);
        queue.qpush(h * 3);
        queue.qpush(h * 5);
        if i <= 20 || i == 1691 {
            println!("{:6}: {}", i, h);
        }
        i += 1;
    }
}