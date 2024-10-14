use std::alloc::{alloc, realloc, Layout};
use std::ptr::write_bytes;
use std::sync::Mutex;

type Byte = u8;

// Modified: Removed Mutex from GRID, W, H, LEN, CNT, and NEXT
static GRID: Vec<Byte> = Vec::new();
static W: i32 = 0;
static H: i32 = 0;
static LEN: usize = 0;
static CNT: u64 = 0;

// Modified: Removed Mutex from NEXT
static NEXT: [i32; 4] = [0; 4];

static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

fn walk(y: i32, x: i32, grid: &mut Vec<Byte>, w: i32, h: i32, len: usize, cnt: &mut u64) {
    let mut i: i32;
    let mut t: usize;

    if y == h || x == w {
        *cnt += 2;
        return;
    }

    t = (y * (w + 1) + x) as usize;
    grid[t] += 1; // Modified: grid is now mutable
    grid[len - t] += 1; // Modified: grid is now mutable

    for i in 0..4 {
        let next_index = (t as i32 + NEXT[i as usize]) as usize;
        if next_index < len && grid[next_index] == 0 {
            walk(y + DIR[i as usize][0], x + DIR[i as usize][1], grid, w, h, len, cnt);
        }
    }

    grid[t] -= 1; // Modified: grid is now mutable
    grid[len - t] -= 1; // Modified: grid is now mutable
}

fn solve(hh: i32, ww: i32, recur: i32) -> u64 {
    let mut t: i32;
    let mut cx: i32;
    let mut cy: i32;
    let mut x: i32;

    let mut h = hh;
    let mut w = ww;
    let mut len = (hh as usize + 1).checked_mul(ww as usize + 1).expect("Integer overflow");
    let mut cnt = 0;
    let mut grid = vec![0; len]; // Modified: Resized the Vec instead of reallocating a raw pointer

    if (hh & 1) != 0 {
        t = w;
        w = h;
        h = t;
    }
    if (hh & 1) != 0 {
        return 0;
    }
    if w == 1 {
        return 1;
    }
    if w == 2 {
        return h as u64;
    }
    if h == 2 {
        return w as u64;
    }

    cy = h / 2;
    cx = w / 2;

    len -= 1;

    if recur != 0 {
        cnt = 0;
    }
    for x in cx + 1..w {
        t = cy * (w + 1) + x;
        grid[t as usize] = 1; // Modified: grid is now mutable
        grid[len - t as usize] = 1; // Modified: grid is now mutable
        walk(cy - 1, x, &mut grid, w, h, len, &mut cnt);
    }
    cnt += 1;

    if h == w {
        cnt *= 2;
    } else if (w & 1) == 0 && recur != 0 {
        cnt += solve(w, h, 0);
    }

    cnt
}

fn main() {
    let mut y: i32;
    let mut x: i32;
    for y in 1..=9 {
        for x in 1..=y {
            if (x & 1) == 0 || (y & 1) == 0 {
                println!("{} x {}: {}", y, x, solve(y, x, 1));
            }
        }
    }
}