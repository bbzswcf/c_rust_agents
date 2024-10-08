use std::alloc::{Layout, alloc, dealloc};
use std::ptr::write_bytes;

type Byte = u8;

// Modified: Use safe alternatives to static mut variables
struct GridState {
    grid: Vec<Byte>,
    w: i32,
    h: i32,
    len: i32,
    cnt: u64,
    next: [i32; 4],
}

impl GridState {
    fn new() -> Self {
        Self {
            grid: Vec::new(),
            w: 0,
            h: 0,
            len: 0,
            cnt: 0,
            next: [0, -1, 0, 1],
        }
    }

    fn walk(&mut self, y: i32, x: i32) {
        let mut i: i32;
        let mut t: i32;

        // Modified: Correct boundary conditions
        if y <= 0 || y >= self.h || x <= 0 || x >= self.w {
            self.cnt += 2;
            return;
        }

        t = y * (self.w + 1) + x;
        // Modified: Ensure GRID is not null before dereferencing
        if t >= self.len {
            panic!("Index out of bounds");
        }
        self.grid[t as usize] += 1;
        self.grid[(self.len - t) as usize] += 1;

        for i in 0..4 {
            // Modified: Ensure array access is within bounds
            if i < self.next.len() as i32 && (t + self.next[i as usize]) < self.len && self.grid[(t + self.next[i as usize]) as usize] == 0 {
                self.walk(y + DIR[i as usize][0], x + DIR[i as usize][1]);
            }
        }

        // Modified: Ensure GRID is not null before dereferencing
        if t >= self.len {
            panic!("Index out of bounds");
        }
        self.grid[t as usize] -= 1;
        self.grid[(self.len - t) as usize] -= 1;
    }

    fn solve(&mut self, hh: i32, ww: i32, recur: i32) -> u64 {
        let mut t: i32;
        let mut cx: i32;
        let mut cy: i32;
        let mut x: i32;

        self.h = hh;
        self.w = ww;

        // Modified: Correct logic for determining when to return early
        if (self.h & 1) != 0 {
            t = self.w;
            self.w = self.h;
            self.h = t;
        }
        if (self.h & 1) != 0 {
            return 0;
        }
        if self.w == 1 {
            return 1;
        }
        if self.w == 2 {
            return self.h as u64;
        }
        if self.h == 2 {
            return self.w as u64;
        }

        cy = self.h / 2;
        cx = self.w / 2;

        // Modified: Ensure no integer overflow
        self.len = (self.h + 1).checked_mul(self.w + 1).expect("Integer overflow");
        self.grid.resize(self.len as usize, 0);

        self.next[0] = -1;
        self.next[1] = -self.w - 1;
        self.next[2] = 1;
        self.next[3] = self.w + 1;

        if recur != 0 {
            self.cnt = 0;
        }
        for x in cx + 1..self.w {
            t = cy * (self.w + 1) + x;
            // Modified: Ensure GRID is not null before dereferencing
            if t >= self.len {
                panic!("Index out of bounds");
            }
            self.grid[t as usize] = 1;
            self.grid[(self.len - t) as usize] = 1;
            self.walk(cy - 1, x);
        }
        self.cnt += 1;

        if self.h == self.w {
            self.cnt *= 2;
        } else if (self.w & 1) == 0 && recur != 0 {
            self.solve(self.w, self.h, 0);
        }

        self.cnt
    }
}

const DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

fn main() {
    let mut state = GridState::new();
    let mut y: i32;
    let mut x: i32;
    for y in 1..=9 {
        for x in 1..=y {
            if (x & 1) == 0 || (y & 1) == 0 {
                println!("{} x {}: {}", y, x, state.solve(y, x, 1));
            }
        }
    }
}