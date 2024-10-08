const N: usize = 32;
const K: usize = 3;
const MAX: usize = N * K;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn rot(n: usize, p: &mut Point, rx: usize, ry: usize) {
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

fn d2pt(n: usize, d: usize, p: &mut Point) {
    let mut s = 1;
    let mut t = d;
    p.x = 0;
    p.y = 0;
    while s < n {
        let rx = (t / 2) & 1;
        let ry = (t ^ rx) & 1;
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
        d2pt(N, d, &mut curr);
        let cx = curr.x * K;
        let cy = curr.y * K;
        let px = prev.x * K;
        let py = prev.y * K;
        pts[cx][cy] = '.';

        if cx == px {
            if py < cy {
                for y in (py + 1)..cy {
                    pts[cx][y] = '|';
                }
            } else {
                for y in (cy + 1)..py {
                    pts[cx][y] = '|';
                }
            }
        } else {
            if px < cx {
                for x in (px + 1)..cx {
                    pts[x][cy] = '_';
                }
            } else {
                for x in (cx + 1)..px {
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