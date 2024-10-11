use std::alloc;

type Byte = u8;

static mut GRID: *mut Byte = std::ptr::null_mut();

static mut W: i32 = 0;
static mut H: i32 = 0;
static mut LEN: usize = 0;
static mut CNT: u64 = 0;

static NEXT: [i32; 4] = [0, -1, 0, 1];
static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

unsafe fn walk(y: i32, x: i32) {
    let mut i: i32 = 0; // Initialized i
    let mut t: usize = 0; // Initialized t

    if y == 0 || y == H || x == 0 || x == W {
        CNT += 2;
        return;
    }

    t = (y * (W + 1) + x) as usize;
    (*GRID.offset(t as isize)) += 1;
    (*GRID.offset((LEN - t) as isize)) += 1;

    for i in 0..4 {
        // Modified: Ensure that the type of `t` is consistent with the type expected by `GRID.offset`
        if (*GRID.offset((t as isize + NEXT[i as usize] as isize) as isize)) == 0 {
            walk(y + DIR[i as usize][0], x + DIR[i as usize][1]);
        }
    }

    (*GRID.offset(t as isize)) -= 1;
    (*GRID.offset((LEN - t) as isize)) -= 1;
}

unsafe fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut t: i32 = 0; // Initialized t
    let mut cx: i32 = 0; // Initialized cx
    let mut cy: i32 = 0; // Initialized cy
    let mut x: i32 = 0; // Initialized x

    H = hh;
    W = ww;

    if (H & 1) != 0 {
        t = W;
        W = H;
        H = t;
    }
    if (H & 1) != 0 {
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

    LEN = (H + 1) * (W + 1) as usize;
    GRID = realloc(GRID, LEN);
    memset(GRID, 0, LEN - 1);

    NEXT[0] = -1;
    NEXT[1] = -W - 1;
    NEXT[2] = 1;
    NEXT[3] = W + 1;

    if recur {
        CNT = 0;
    }
    for x in cx + 1..W {
        t = cy * (W + 1) + x;
        (*GRID.offset(t as isize)) = 1;
        (*GRID.offset((LEN - t as usize) as isize)) = 1;
        walk(cy - 1, x);
    }
    CNT += 1;

    if H == W {
        CNT *= 2;
    } else if (W & 1) == 0 && recur {
        solve(W, H, false);
    }

    CNT
}

unsafe fn realloc(ptr: *mut Byte, size: usize) -> *mut Byte {
    if ptr.is_null() {
        return alloc::alloc(alloc::Layout::array::<Byte>(size).unwrap());
    } else {
        return alloc::realloc(ptr, alloc::Layout::array::<Byte>(size).unwrap(), size);
    }
}

unsafe fn memset(ptr: *mut Byte, value: u8, size: usize) {
    for i in 0..size {
        *ptr.offset(i as isize) = value;
    }
}

fn main() {
    unsafe {
        let mut y: i32 = 0; // Initialized y
        let mut x: i32 = 0; // Initialized x
        for y in 1..=9 {
            for x in 1..=y {
                if (x & 1) == 0 || (y & 1) == 0 {
                    println!("{} x {}: {}", y, x, solve(y, x, true));
                }
            }
        }
    }
}