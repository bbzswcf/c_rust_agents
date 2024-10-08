const N: usize = 32;
const K: usize = 3;
const MAX: usize = N * K;

struct Point {
    x: i32,
    y: i32,
}

fn rot(n: i32, p: &mut Point, rx: i32, ry: i32) {
    if ry == 0 {
        if rx == 1 {
            p.x = n - 1 - p.x;
            p.y = n - 1 - p.y;
        }
        let t = p.x;
        p.x = p.y;
        p.y = t;
    }
}

fn d2pt(n: i32, d: i32, p: &mut Point) {
    let mut s = 1;
    let mut t = d;
    p.x = 0;
    p.y = 0;
    while s < n {
        let rx = 1 & (t / 2);
        let ry = 1 & (t ^ rx);
        rot(s, p, rx, ry);
        p.x += s * rx;
        p.y += s * ry;
        t /= 4;
        s *= 2;
    }
}

fn main() {
    let mut pts = vec![vec![' '; MAX]; MAX];
    let mut prev = Point { x: 0, y: 0 };
    pts[0][0] = '.';

    for d in 1..(N * N) {
        let mut curr = Point { x: 0, y: 0 };
        d2pt(N as i32, d as i32, &mut curr);
        let cx = curr.x * K as i32;
        let cy = curr.y * K as i32;
        let px = prev.x * K as i32;
        let py = prev.y * K as i32;
        pts[cx as usize][cy as usize] = '.';

        if cx == px {
            if py < cy {
                for y in (py + 1)..cy {
                    pts[cx as usize][y as usize] = '|';
                }
            } else {
                for y in (cy + 1)..py {
                    pts[cx as usize][y as usize] = '|';
                }
            }
        } else {
            if px < cx {
                for x in (px + 1)..cx {
                    pts[x as usize][cy as usize] = '_';
                }
            } else {
                for x in (cx + 1)..px {
                    pts[x as usize][cy as usize] = '_';
                }
            }
        }
        prev = curr;
    }

    for x in 0..MAX {
        for y in 0..MAX {
            print!("{}", pts[y][x]);
        }
        println!();
    }
}