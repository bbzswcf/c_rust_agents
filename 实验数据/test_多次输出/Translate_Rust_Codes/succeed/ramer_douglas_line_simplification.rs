#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn perpendicular_distance(p: Point, p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let d = (dx * dx + dy * dy).sqrt();
    (p.x * dy - p.y * dx + p2.x * p1.y - p2.y * p1.x).abs() / d
}

fn douglas_peucker(points: &[Point], epsilon: f64, dest: &mut [Point]) -> usize {
    assert!(points.len() >= 2);
    assert!(epsilon >= 0.0);

    let mut max_dist = 0.0;
    let mut index = 0;

    for (i, &point) in points.iter().enumerate().take(points.len() - 1).skip(1) {
        let dist = perpendicular_distance(point, points[0], points[points.len() - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }

    if max_dist > epsilon {
        let n1 = douglas_peucker(&points[..index + 1], epsilon, dest);
        let dest_len = dest.len();
        if dest_len >= n1 - 1 {
            let n2 = douglas_peucker(&points[index..], epsilon, &mut dest[n1 - 1..]);
            return n1 + n2 - 1;
        } else {
            return n1;
        }
    }

    if dest.len() >= 2 {
        dest[0] = points[0];
        dest[1] = points[points.len() - 1];
    }

    return 2;
}

fn print_points(points: &[Point]) {
    for (i, &point) in points.iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("({}, {})", point.x, point.y);
    }
    println!();
}

fn main() {
    let points = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 0.1 },
        Point { x: 2.0, y: -0.1 },
        Point { x: 3.0, y: 5.0 },
        Point { x: 4.0, y: 6.0 },
        Point { x: 5.0, y: 7.0 },
        Point { x: 6.0, y: 8.1 },
        Point { x: 7.0, y: 9.0 },
        Point { x: 8.0, y: 9.0 },
        Point { x: 9.0, y: 9.0 },
    ];

    let mut out = [Point { x: 0.0, y: 0.0 }; 10];
    let n = douglas_peucker(&points, 1.0, &mut out);
    print_points(&out[..n]);
}