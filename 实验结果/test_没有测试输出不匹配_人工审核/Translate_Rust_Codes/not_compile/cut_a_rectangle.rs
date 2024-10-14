use std::mem;

type Byte = u8;

fn walk(grid: &mut Vec<Byte>, w: i32, h: i32, len: usize, y: i32, x: i32, cnt: &mut u64) {
    let next = [0, -1, -w - 1, 1, w + 1];
    let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }

    // Modified: Ensure all operands are of the same type before performing the arithmetic operation
    let t = (y as usize) * ((w + 1) as usize) + (x as usize);
    grid[t] += 1;
    grid[len - t] += 1;

    for i in 0..4 {
        // Modified: Enclose the cast operation in parentheses to ensure it is interpreted as a comparison
        if t + (next[i] as usize) < len && grid[t + (next[i] as usize)] == 0 {
            walk(grid, w, h, len, y + dir[i].0, x + dir[i].1, cnt);
        }
    }

    grid[t] -= 1;
    grid[len - t] -= 1;
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut h = hh;
    let mut w = ww;
    // Modified: Explicitly define the type of `t` to avoid type inference issues later in the code
    let mut t: usize;
    let mut cx;
    let mut cy;
    // Modified: Removed unused variable `x`
    let mut cnt = 0;

    if h & 1 != 0 {
        // Modified: Convert `w` to `usize` before assigning it to `t`
        t = w as usize;
        w = h;
        // Modified: Convert `t` to `i32` before assigning it to `h`
        h = t as i32;
    }
    if h & 1 != 0 {
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

    // Modified: Ensure both operands are of the same type before performing the multiplication
    let len = (h as usize + 1) * (w as usize + 1);
    let mut grid = vec![0; len];

    let next = [-1, -w - 1, 1, w + 1];

    if recur {
        cnt = 0;
    }

    for x in cx + 1..w {
        // Modified: Ensure all operands are of the same type before performing the arithmetic operation
        t = (cy as usize) * ((w + 1) as usize) + (x as usize);
        grid[t] = 1;
        // Modified: Ensure the index `len - t - 1` is within the bounds of the `grid` array
        if len > t + 1 {
            grid[len - t - 1] = 1;
        }
        // Modified: Ensure `len - 1` is of type `usize` when passing it to the `walk` function
        walk(&mut grid, w, h, (len - 1) as usize, cy - 1, x, &mut cnt);
    }

    cnt += 1;

    if h == w {
        cnt *= 2;
    } else if w & 1 == 0 && recur {
        cnt += solve(w, h, false);
    }

    cnt
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