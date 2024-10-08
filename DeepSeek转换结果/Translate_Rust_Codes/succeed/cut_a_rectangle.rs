fn walk(y: i32, x: i32, grid: &mut Vec<u8>, w: i32, h: i32, len: i32, cnt: &mut u64, next: &[i32; 4], dir: &[[i32; 2]; 4]) {
    if y == 0 || y == h || x == 0 || x == w {
        *cnt += 2;
        return;
    }

    let t = y * (w + 1) + x;
    grid[t as usize] += 1;
    grid[(len - t) as usize] += 1;

    for i in 0..4 {
        let next_pos = t + next[i];
        if next_pos >= 0 && next_pos < len && grid[next_pos as usize] == 0 {
            walk(y + dir[i][0], x + dir[i][1], grid, w, h, len, cnt, next, dir);
        }
    }

    grid[t as usize] -= 1;
    grid[(len - t) as usize] -= 1;
}

fn solve(mut hh: i32, mut ww: i32, recur: bool) -> u64 {
    let mut grid: Vec<u8> = Vec::new();
    let mut w: i32;
    let mut h: i32;
    let mut len: i32;
    let mut cnt: u64 = 0;
    let mut next: [i32; 4] = [0, -1, 1, 1];
    let dir: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

    h = hh;
    w = ww;

    if h % 2 != 0 {
        let t = w;
        w = h;
        h = t;
    }

    if h % 2 != 0 { return 0; }
    if w == 1 { return 1; }
    if w == 2 { return h as u64; }
    if h == 2 { return w as u64; }

    let cy = h / 2;
    let cx = w / 2;

    len = (h + 1) * (w + 1);
    grid.resize(len as usize, 0);
    len -= 1;

    next[0] = -1;
    next[1] = -w - 1;
    next[2] = 1;
    next[3] = w + 1;

    for x in cx + 1..w {
        let t = cy * (w + 1) + x;
        grid[t as usize] = 1;
        grid[(len - t) as usize] = 1;
        walk(cy - 1, x, &mut grid, w, h, len, &mut cnt, &next, &dir);
    }
    cnt += 1;

    if h == w {
        cnt *= 2;
    } else if w % 2 == 0 && recur {
        cnt += solve(w, h, false);
    }

    cnt
}

fn main() {
    for y in 1..=9 {
        for x in 1..=y {
            if x % 2 == 0 || y % 2 == 0 {
                println!("{} x {}: {}", y, x, solve(y, x, true));
            }
        }
    }
}