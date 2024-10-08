#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

// Returns the distance from point p to the line between p1 and p2
fn perpendicular_distance(p: Point, p1: Point, p2: Point) -> f64 {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let d = (dx * dx + dy * dy).sqrt();
    (p.x * dy - p.y * dx + p2.x * p1.y - p2.y * p1.x).abs() / d
}

// Simplify an array of points using the RamerDouglasPeucker algorithm.
// Returns the number of output points.
fn douglas_peucker(points: &[Point], epsilon: f64, dest: &mut [Point], destlen: usize) -> usize {
    assert!(points.len() >= 2);
    assert!(epsilon >= 0.0);
    let mut max_dist = 0.0;
    let mut index = 0;
    for i in 1..points.len() - 1 {
        let dist = perpendicular_distance(points[i], points[0], points[points.len() - 1]);
        if dist > max_dist {
            max_dist = dist;
            index = i;
        }
    }
    if max_dist > epsilon {
        let n1 = douglas_peucker(&points[..index + 1], epsilon, dest, destlen);
        if destlen >= n1 - 1 {
            let destlen = destlen - n1 + 1;
            let dest = &mut dest[n1 - 1..];
            let n2 = douglas_peucker(&points[index..], epsilon, dest, destlen);
            return n1 + n2 - 1;
        } else {
            return n1;
        }
    }
    if destlen >= 2 {
        dest[0] = points[0];
        dest[1] = points[points.len() - 1];
    }
    2
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
    let len = points.len();
    let mut out = vec![Point { x: 0.0, y: 0.0 }; len];
    let n = douglas_peucker(&points, 1.0, &mut out, len);
    print_points(&out[..n]);
}