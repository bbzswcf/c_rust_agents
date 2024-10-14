use std::sync::{Mutex, RwLock};
use lazy_static::lazy_static;

type Byte = u8;

lazy_static! {
    static ref GRID: Mutex<Vec<Byte>> = Mutex::new(Vec::new());
    static ref W: RwLock<i32> = RwLock::new(0);
    static ref H: RwLock<i32> = RwLock::new(0);
    static ref LEN: RwLock<i32> = RwLock::new(0);
    static ref CNT: Mutex<u64> = Mutex::new(0);
    static ref NEXT: [i32; 4] = [-1, -1, 1, 1]; // Modified: Initialize NEXT with hardcoded values
}

static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

fn walk(y: i32, x: i32) {
    let mut grid = GRID.lock().expect("Failed to lock GRID");
    let w = *W.read().expect("Failed to read W");
    let h = *H.read().expect("Failed to read H");
    let len = *LEN.read().expect("Failed to read LEN");
    let mut cnt = CNT.lock().expect("Failed to lock CNT");

    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }

    let t = y * (w + 1) + x;
    grid[t as usize] += 1;
    grid[(len - t) as usize] += 1;

    for i in 0..4 {
        if grid[(t + NEXT[i]) as usize] == 0 {
            walk(y + DIR[i][0], x + DIR[i][1]);
        }
    }

    grid[t as usize] -= 1;
    grid[(len - t) as usize] -= 1;
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut t: i32;
    let mut cx: i32;
    let mut cy: i32;

    *W.write().expect("Failed to write W") = ww;
    *H.write().expect("Failed to write H") = hh;

    if hh & 1 != 0 {
        t = ww;
        *W.write().expect("Failed to write W") = hh;
        *H.write().expect("Failed to write H") = t;
    }
    if *H.read().expect("Failed to read H") & 1 != 0 {
        return 0;
    }
    if *W.read().expect("Failed to read W") == 1 {
        return 1;
    }
    if *W.read().expect("Failed to read W") == 2 {
        return *H.read().expect("Failed to read H") as u64;
    }
    if *H.read().expect("Failed to read H") == 2 {
        return *W.read().expect("Failed to read W") as u64;
    }

    cy = *H.read().expect("Failed to read H") / 2;
    cx = *W.read().expect("Failed to read W") / 2;

    *LEN.write().expect("Failed to write LEN") = (*H.read().expect("Failed to read H") + 1) * (*W.read().expect("Failed to read W") + 1);
    let mut grid = GRID.lock().expect("Failed to lock GRID");
    grid.resize(*LEN.read().expect("Failed to read LEN") as usize, 0);
    *LEN.write().expect("Failed to write LEN") -= 1;

    if recur {
        *CNT.lock().expect("Failed to lock CNT") = 0;
    }
    for x in cx + 1..*W.read().expect("Failed to read W") {
        t = cy * (*W.read().expect("Failed to read W") + 1) + x;
        grid[t as usize] = 1;
        grid[(*LEN.read().expect("Failed to read LEN") - t) as usize] = 1;
        walk(cy - 1, x);
    }
    *CNT.lock().expect("Failed to lock CNT") += 1;

    if *H.read().expect("Failed to read H") == *W.read().expect("Failed to read W") {
        *CNT.lock().expect("Failed to lock CNT") *= 2;
    } else if *W.read().expect("Failed to read W") & 1 == 0 && recur {
        solve(*W.read().expect("Failed to read W"), *H.read().expect("Failed to read H"), false);
    }

    *CNT.lock().expect("Failed to lock CNT")
}

fn main() {
    for y in 1..=9 {
        for x in 1..=y {
            if x & 1 == 0 || y & 1 == 0 {
                println!("{} x {}: {}", y, x, solve(y, x, true));
            }
        }
    }
}