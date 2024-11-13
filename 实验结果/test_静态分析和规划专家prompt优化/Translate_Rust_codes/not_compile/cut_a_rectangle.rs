type Byte = u8;
static mut GRID: Option<Vec<Byte>> = None;
static mut W: i32 = 0;
static mut H: i32 = 0;
static mut LEN: usize = 0;
static mut CNT: u64 = 0;
static NEXT: [i32; 4] = [-1, -unsafe { W } - 1, 1, unsafe { W } + 1];
static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

unsafe fn walk(y: i32, x: i32) {
    let mut i: i32;
    let mut t: usize;
    if y == 0 || y == unsafe { H } || x == 0 || x == unsafe { W } {
        unsafe { CNT = CNT.wrapping_add(2); }
        return;
    }
    t = (y * (unsafe { W } + 1) + x) as usize;
    if let Some(ref mut grid) = unsafe { GRID.as_mut() } {
        grid[t] += 1;
        grid[LEN - t] += 1;
        for i in 0..4 {
            if grid[t + NEXT[i as usize] as usize] == 0 {
                walk(y + DIR[i as usize][0], x + DIR[i as usize][1]);
            }
        }
        grid[t] -= 1;
        grid[LEN - t] -= 1;
    }
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let t: i32;
    let mut cx: i32;
    let mut cy: i32;
    let mut x: i32;
    unsafe { H = hh; }
    unsafe { W = ww; }
    if unsafe { H & 1 } != 0 {
        t = unsafe { W };
        unsafe { W = H; }
        unsafe { H = t; }
    }
    if unsafe { H & 1 } != 0 {
        return 0;
    }
    if unsafe { W } == 1 {
        return 1;
    }
    if unsafe { W } == 2 {
        return H as u64;
    }
    if unsafe { H } == 2 {
        return W as u64;
    }
    cy = unsafe { H / 2 };
    cx = unsafe { W / 2 };
    unsafe { LEN = ((H + 1) * (W + 1)) as usize; }
    unsafe { GRID = Some(vec![0; LEN]); }
    if recur {
        unsafe { CNT = 0; }
    }
    for x in cx + 1..unsafe { W } {
        t = cy * (unsafe { W } + 1) + x;
        if let Some(ref mut grid) = unsafe { GRID.as_mut() } {
            unsafe { grid[t as usize] = 1; }
            unsafe { grid[LEN - t as usize] = 1; }
            walk(cy - 1, x);
        }
    }
    unsafe { CNT = CNT.wrapping_add(1); }
    if unsafe { H } == unsafe { W } {
        unsafe { CNT = CNT.wrapping_mul(2); }
    } else if unsafe { W & 1 } == 0 && recur {
        unsafe { CNT = CNT.wrapping_add(solve(W, H, false)); }
    }
    unsafe { return CNT; }
}

fn main() {
    let mut x: i32;
    for y in 1..=9 {
        for x in 1..=y {
            if x & 1 == 0 || y & 1 == 0 {
                println!("{} x {}: {}", y, x, solve(y, x, true));
            }
        }
    }
}