// Removed: Unused import
// use std::ptr;

type Byte = u8;

static mut GRID: Option<Vec<Byte>> = None;

static mut W: i32 = 0;
static mut H: i32 = 0;
static mut LEN: usize = 0;
static mut CNT: u64 = 0;

// Modified: Declared NEXT as a mutable static item
static mut NEXT: [i32; 4] = [-1, -1, 1, 1];
static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

unsafe fn walk(y: i32, x: i32) {
    let mut i: i32;
    // Modified: Removed unnecessary mutability
    let t: usize;

    if y == 0 || y == H || x == 0 || x == W {
        CNT += 2;
        return;
    }

    t = (y * (W + 1) + x) as usize;
    if let Some(ref mut grid) = GRID {
        grid[t] += 1;
        grid[LEN - t] += 1;

        for i in 0..4 {
            if grid[(t as i32 + NEXT[i as usize]) as usize] == 0 {
                walk(y + DIR[i as usize][0], x + DIR[i as usize][1]);
            }
        }

        grid[t] -= 1;
        grid[LEN - t] -= 1;
    }
}

unsafe fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut t: i32;
    // Modified: Removed unnecessary mutability
    let cx: i32;
    let cy: i32;
    let mut x: i32;

    H = hh;
    W = ww;

    if H & 1 != 0 {
        t = W;
        W = H;
        H = t;
    }
    if H & 1 != 0 {
        return 0;
    }
    if W == 1 {
        return 1;
    }
    if W == 2 {
        return H as u64;
    }
    if H == 2 {
        return W as u64;
    }

    cy = H / 2;
    cx = W / 2;

    LEN = ((H + 1) * (W + 1)) as usize;
    GRID = Some(vec![0; LEN]);

    // Modified: Declared NEXT as a mutable static item
    NEXT[0] = -1;
    NEXT[1] = -W - 1;
    NEXT[2] = 1;
    NEXT[3] = W + 1;

    if recur {
        CNT = 0;
    }

    if let Some(ref mut grid) = GRID {
        for x in cx + 1..W {
            // Modified: Changed the type of `t` to `usize`
            let t = (cy * (W + 1) + x) as usize;
            grid[t] = 1;
            grid[LEN - t] = 1;
            walk(cy - 1, x);
        }
    }

    CNT += 1;

    if H == W {
        CNT *= 2;
    } else if W & 1 == 0 && recur {
        solve(W, H, false);
    }

    CNT
}

fn main() {
    unsafe {
        let mut y: i32;
        let mut x: i32;
        for y in 1..=9 {
            for x in 1..=y {
                if x & 1 == 0 || y & 1 == 0 {
                    println!("{} x {}: {}", y, x, solve(y, x, true));
                }
            }
        }
    }
}