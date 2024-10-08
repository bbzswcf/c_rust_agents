const N: i32 = 32;
const K: i32 = 3;
const MAX: i32 = N * K;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn rot(n: i32, p: &mut Point, rx: i32, ry: i32) {
    let mut t: i32;
    if ry == 0 {
        if rx == 1 {
            p.x = n - 1 - p.x;
            p.y = n - 1 - p.y;
        }
        t = p.x;
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
    let mut d: i32;
    let mut x: i32;
    let mut y: i32;
    let mut cx: i32;
    let mut cy: i32;
    let mut px: i32;
    let mut py: i32;
    let mut pts = vec![vec![' '; MAX as usize]; MAX as usize];
    let mut curr = Point { x: 0, y: 0 };
    let mut prev = Point { x: 0, y: 0 };

    for x in 0..MAX {
        for y in 0..MAX {
            pts[x as usize][y as usize] = ' ';
        }
    }

    pts[0][0] = '.';

    for d in 1..(N * N) {
        d2pt(N, d, &mut curr);
        cx = curr.x * K;
        cy = curr.y * K;
        px = prev.x * K;
        py = prev.y * K;
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
            print!("{}", pts[y as usize][x as usize]);
        }
        println!();
    }
}