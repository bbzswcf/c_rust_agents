use std::mem;

type Byte = u8;

fn walk(grid: &mut Vec<Byte>, w: i32, h: i32, len: usize, y: i32, x: i32, cnt: &mut u64) {
    let next = [0, -1, -w - 1, 1, w + 1];
    let dir = [(0, -1), (-1, 0), (0, 1), (1, 0)];

    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }

    let t = (y * (w + 1) + x) as usize;
    grid[t] += 1;
    grid[len - t] += 1;

    for i in 0..4 {
        if grid[t + next[i] as usize] == 0 {
            walk(grid, w, h, len, y + dir[i].0, x + dir[i].1, cnt);
        }
    }

    grid[t] -= 1;
    grid[len - t] -= 1;
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut h = hh;
    let mut w = ww;
    let mut t;
    let mut cx;
    let mut cy;
    let mut x;
    let mut cnt = 0;

    if h & 1 != 0 {
        t = w;
        w = h;
        h = t;
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

    let len = (h + 1) * (w + 1) as usize;
    let mut grid = vec![0; len];

    let next = [-1, -w - 1, 1, w + 1];

    if recur {
        cnt = 0;
    }

    for x in cx + 1..w {
        t = (cy * (w + 1) + x) as usize;
        grid[t] = 1;
        grid[len - t - 1] = 1;
        walk(&mut grid, w, h, len - 1, cy - 1, x, &mut cnt);
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