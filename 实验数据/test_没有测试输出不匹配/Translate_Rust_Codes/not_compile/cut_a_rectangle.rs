use lazy_static::lazy_static;

type Byte = u8;

lazy_static! {
    static ref GRID: std::sync::Mutex<Option<Vec<Byte>>> = std::sync::Mutex::new(None);
    static ref W: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
    static ref H: std::sync::Mutex<i32> = std::sync::Mutex::new(0);
    static ref LEN: std::sync::Mutex<usize> = std::sync::Mutex::new(0);
    static ref CNT: std::sync::Mutex<u64> = std::sync::Mutex::new(0);
}

static DIR: [[i32; 2]; 4] = [[0, -1], [-1, 0], [0, 1], [1, 0]];

fn walk(y: i32, x: i32, NEXT: [i32; 4]) {
    let mut i: usize;
    let t: usize;

    if y == 0 || y == *H.lock().unwrap() || x == 0 || x == *W.lock().unwrap() {
        *CNT.lock().unwrap() = CNT.lock().unwrap().wrapping_add(2);
        return;
    }

    t = (y * (*W.lock().unwrap() + 1) + x) as usize;
    if let Some(ref mut grid) = *GRID.lock().unwrap() {
        grid[t] += 1;
        grid[*LEN.lock().unwrap() - t] += 1;

        for i in 0..4 {
            let index = (t as i32 + NEXT[i]) as usize;
            if index < *LEN.lock().unwrap() && grid[index] == 0 {
                walk(y + DIR[i][0], x + DIR[i][1], NEXT);
            }
        }

        grid[t] -= 1;
        grid[*LEN.lock().unwrap() - t] -= 1;
    }
}

fn solve(hh: i32, ww: i32, recur: bool) -> u64 {
    let mut t: i32;
    let cx: i32;
    let cy: i32;
    let mut x: i32;

    *H.lock().unwrap() = hh;
    *W.lock().unwrap() = ww;

    if *H.lock().unwrap() & 1 != 0 {
        t = *W.lock().unwrap();
        *W.lock().unwrap() = *H.lock().unwrap();
        *H.lock().unwrap() = t;
    }
    if *H.lock().unwrap() & 1 != 0 {
        return 0;
    }
    if *W.lock().unwrap() == 1 {
        return 1;
    }
    if *W.lock().unwrap() == 2 {
        return *H.lock().unwrap() as u64;
    }
    if *H.lock().unwrap() == 2 {
        return *W.lock().unwrap() as u64;
    }

    cy = *H.lock().unwrap() / 2;
    cx = *W.lock().unwrap() / 2;

    *LEN.lock().unwrap() = (*H.lock().unwrap() + 1).checked_mul(*W.lock().unwrap() + 1).expect("Overflow in LEN calculation") as usize;
    *GRID.lock().unwrap() = Some(vec![0; *LEN.lock().unwrap()]);

    if recur {
        *CNT.lock().unwrap() = 0;
    }

    let mut NEXT: [i32; 4] = [-1, -*W.lock().unwrap() - 1, 1, *W.lock().unwrap() + 1];

    for x in cx + 1..*W.lock().unwrap() {
        t = cy * (*W.lock().unwrap() + 1) + x;
        if let Some(ref mut grid) = *GRID.lock().unwrap() {
            grid[t as usize] = 1;
            grid[*LEN.lock().unwrap() - t as usize] = 1;
        }
        walk(cy - 1, x, NEXT);
    }
    *CNT.lock().unwrap() += 1;

    if *H.lock().unwrap() == *W.lock().unwrap() {
        *CNT.lock().unwrap() *= 2;
    } else if *W.lock().unwrap() & 1 == 0 && recur {
        solve(*W.lock().unwrap(), *H.lock().unwrap(), false);
    }

    *CNT.lock().unwrap()
}

fn main() {
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