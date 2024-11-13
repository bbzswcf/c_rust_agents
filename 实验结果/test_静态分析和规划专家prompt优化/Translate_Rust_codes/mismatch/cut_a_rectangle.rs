use std::mem;
type Byte = u8;

fn walk(grid: &mut Vec<Byte>, w: usize, h: usize, len: usize, y: usize, x: usize, cnt: &mut u64) {
    let mut next = [0isize; 4];
    next[0] = -1;
    next[1] = -(w as isize) - 1;
    next[2] = 1;
    next[3] = (w + 1) as isize;
    let dir = [[0, -1], [-1, 0], [0, 1], [1, 0]];
    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }
    let t = y * (w + 1) + x;
    grid[t] += 1;
    grid[len - t] += 1;
    for i in 0..4 {
        let new_t = (t as isize + next[i]) as usize;
        if new_t < len && grid[new_t] == 0 {
            walk(grid, w, h, len, (y as isize + dir[i][0]) as usize, (x as isize + dir[i][1]) as usize, cnt);
        }
    }
    grid[t] -= 1;
    grid[len - t] -= 1;
}

fn solve(hh: usize, ww: usize, recur: bool) -> u64 {
    let mut h = hh;
    let mut w = ww;
    let mut t;
    let mut cx;
    let mut cy;
    let mut x: usize;
    let mut cnt = 0;
    if h & 1 != 0 {
        t = w;
        w = h;
        h = t;
    }
    if h & 1 != 0 {
        let mut x: usize;
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
    let len = (h + 1) * (w + 1);
    let mut grid = vec![0; len];
    if recur {
        cnt = 0;
    }
    for x in cx + 1..w {
        t = cy * (w + 1) + x;
        grid[t] = 1;
        grid[len - t] = 1;
        walk(&mut grid, w, h, len, cy - 1, x, &mut cnt);
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