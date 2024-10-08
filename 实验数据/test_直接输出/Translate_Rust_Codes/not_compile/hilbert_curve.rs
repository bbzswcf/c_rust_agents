const N: usize = 32;
const K: usize = 3;
const MAX: usize = N * K;

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
    let mut x: usize;
    let mut y: usize;
    let mut cx: usize;
    let mut cy: usize;
    let mut px: usize;
    let mut py: usize;
    let mut pts = vec![vec![' '; MAX]; MAX];
    let mut curr = Point { x: 0, y: 0 };
    let mut prev = Point { x: 0, y: 0 };

    for x in 0..MAX {
        for y in 0..MAX {
            pts[x][y] = ' ';
        }
    }

    pts[0][0] = '.';

    for d in 1..(N * N) as i32 {
        d2pt(N as i32, d, &mut curr);
        cx = (curr.x * K as i32) as usize;
        cy = (curr.y * K as i32) as usize;
        px = (prev.x * K as i32) as usize;
        py = (prev.y * K as i32) as usize;
        pts[cx][cy] = '.';

        if cx == px {
            if py < cy {
                for y in py + 1..cy {
                    pts[cx][y] = '|';
                }
            } else {
                for y in cy + 1..py {
                    pts[cx][y] = '|';
                }
            }
        } else {
            if px < cx {
                for x in px + 1..cx {
                    pts[x][cy] = '_';
                }
            } else {
                for x in cx + 1..px {
                    pts[x][cy] = '_';
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