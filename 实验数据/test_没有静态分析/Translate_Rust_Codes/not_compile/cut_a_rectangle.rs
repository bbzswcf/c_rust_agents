type Byte = u8;

static mut GRID: Vec<Byte> = Vec::new();

static mut W: i32 = 0;
static mut H: i32 = 0;
static mut LEN: usize = 0;
static mut CNT: u64 = 0;

// Wrapped the initialization of NEXT in an unsafe block
static NEXT: [i32; 4] = unsafe { [-1, -W - 1, 1, W + 1] };
static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

unsafe fn walk(y: i32, x: i32) {
    // Removed unused variables i and t

    if y == 0 || y == H || x == 0 || x == W {
        // Modified: Ensure that CNT does not overflow
        CNT = match CNT.checked_add(2) {
            Some(val) => val,
            None => panic!("CNT overflow"),
        };
        return;
    }

    let t = (y * (W + 1) + x) as usize;
    // Modified: Ensure that the calculation of t does not result in an out-of-bounds index
    if t >= LEN {
        panic!("Index out of bounds");
    }
    GRID[t] += 1;
    GRID[LEN - t] += 1;

    for i in 0..4 {
        if GRID[t + NEXT[i] as usize] == 0 {
            // Modified: Ensure that the addition operations do not overflow
            let new_y = match y.checked_add(DIR[i][0]) {
                Some(val) => val,
                None => panic!("y overflow"),
            };
            let new_x = match x.checked_add(DIR[i][1]) {
                Some(val) => val,
                None => panic!("x overflow"),
            };
            walk(new_y, new_x);
        }
    }

    GRID[t] -= 1;
    GRID[LEN - t] -= 1;
}

unsafe fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut t: i32;
    let mut cx: i32;
    let mut cy: i32;
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

    // Modified: Ensure that the multiplication does not overflow
    LEN = match ((H + 1) as usize).checked_mul((W + 1) as usize) {
        Some(val) => val,
        None => panic!("LEN overflow"),
    };
    // Modified: Ensure that the resizing operation does not cause an overflow
    if LEN > usize::MAX / 2 {
        panic!("LEN too large for resizing");
    }
    GRID.resize(LEN, 0);
    LEN -= 1;

    if recur {
        CNT = 0;
    }
    for x in cx + 1..W {
        t = cy * (W + 1) + x;
        GRID[t as usize] = 1;
        GRID[LEN - t as usize] = 1;
        walk(cy - 1, x);
    }
    // Modified: Ensure that the addition operation does not overflow
    CNT = match CNT.checked_add(1) {
        Some(val) => val,
        None => panic!("CNT overflow"),
    };

    if H == W {
        // Modified: Ensure that the multiplication operation does not overflow
        CNT = match CNT.checked_mul(2) {
            Some(val) => val,
            None => panic!("CNT overflow"),
        };
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