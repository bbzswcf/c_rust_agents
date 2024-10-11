use std::mem;

type Byte = u8;

static DIR: [(i32, i32); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

fn walk(grid: &mut Vec<Byte>, w: i32, h: i32, y: i32, x: i32, cnt: &mut u64) {
    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }

    let t = (y * (w + 1) + x) as usize;
    grid[t] += 1;
    grid[grid.len() - 1 - t] += 1;

    for i in 0..4 {
        let next = match i {
            0 => -1,
            1 => -(w + 1),
            2 => 1,
            3 => w + 1,
            _ => unreachable!(),
        };

        if grid[(t as i32 + next) as usize] == 0 {
            walk(grid, w, h, y + DIR[i].0, x + DIR[i].1, cnt);
        }
    }

    grid[t] -= 1;
    grid[grid.len() - 1 - t] -= 1;
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut h = hh;
    let mut w = ww;

    if h & 1 != 0 {
        mem::swap(&mut h, &mut w);
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

    let cy = h / 2;
    let cx = w / 2;

    let len = (h + 1) * (w + 1) as usize;
    let mut grid = vec![0; len];

    let mut next = [-1, -(w + 1), 1, w + 1];

    let mut cnt = if recur { 0 } else { 1 };
    for x in cx + 1..w {
        let t = (cy * (w + 1) + x) as usize;
        grid[t] = 1;
        grid[len - 1 - t] = 1;
        walk(&mut grid, w, h, cy - 1, x, &mut cnt);
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