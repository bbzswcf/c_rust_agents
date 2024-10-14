use std::mem;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Node {
    data: Point,
    next: Option<Box<Node>>,
}

fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - a.y) > (b.y - a.y) * (c.x - a.x)
}

fn comp(lhs: &Point, rhs: &Point) -> std::cmp::Ordering {
    if lhs.x < rhs.x {
        std::cmp::Ordering::Less
    } else if rhs.x < lhs.x {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

// Modified: Accept a mutable reference to `ptr`
fn push_back(ptr: &mut Option<Box<Node>>, data: Point) {
    if ptr.is_none() {
        *ptr = Some(Box::new(Node {
            data,
            next: None,
        }));
        return;
    }

    // Modified: Use `as_mut` to get a mutable reference to the contained value without moving it
    let mut current = ptr.as_mut().unwrap();
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }

    current.next = Some(Box::new(Node {
        data,
        next: None,
    }));
}

// Modified: Accept a mutable reference to `ptr`
fn pop_back(ptr: &mut Option<Box<Node>>) {
    if ptr.is_none() {
        return;
    }

    if ptr.as_ref().unwrap().next.is_none() {
        *ptr = None;
        return;
    }

    // Modified: Use `as_mut` to get a mutable reference to the contained value without moving it
    let mut current = ptr.as_mut().unwrap();
    while current.next.as_ref().unwrap().next.is_some() {
        current = current.next.as_mut().unwrap();
    }

    current.next = None;
}

fn print(ptr: Option<Box<Node>>) {
    print!("[");
    if let Some(node) = ptr {
        print!("({}, {})", node.data.x, node.data.y);
        let mut current = node.next;
        while let Some(node) = current {
            print!(", ({}, {})", node.data.x, node.data.y);
            current = node.next;
        }
    }
    println!("]");
}

fn convex_hull(points: &[Point]) -> Option<Box<Node>> {
    let mut h: Option<Box<Node>> = None;
    let mut h_len = 0;

    let mut sorted_points = points.to_vec();
    sorted_points.sort_by(comp);

    for &p in sorted_points.iter() {
        while h_len >= 2 {
            // Modified: Use `as_mut` to get a mutable reference to the contained value without moving it
            let mut hptr = h.as_mut().unwrap();
            while hptr.next.as_ref().unwrap().next.is_some() {
                hptr = hptr.next.as_mut().unwrap();
            }
            if ccw(&hptr.data, &hptr.next.as_ref().unwrap().data, &p) {
                break;
            }

            pop_back(&mut h);
            h_len -= 1;
        }

        push_back(&mut h, p);
        h_len += 1;
    }

    for &p in sorted_points.iter().rev() {
        while h_len >= 2 {
            // Modified: Use `as_mut` to get a mutable reference to the contained value without moving it
            let mut hptr = h.as_mut().unwrap();
            while hptr.next.as_ref().unwrap().next.is_some() {
                hptr = hptr.next.as_mut().unwrap();
            }
            if ccw(&hptr.data, &hptr.next.as_ref().unwrap().data, &p) {
                break;
            }

            pop_back(&mut h);
            h_len -= 1;
        }

        push_back(&mut h, p);
        h_len += 1;
    }

    pop_back(&mut h);
    h
}

fn main() {
    let points = [
        Point { x: 16, y:  3 }, Point { x: 12, y: 17 }, Point { x:  0, y:  6 }, Point { x: -4, y: -6 }, Point { x: 16, y:  6 },
        Point { x: 16, y: -7 }, Point { x: 16, y: -3 }, Point { x: 17, y: -4 }, Point { x:  5, y: 19 }, Point { x: 19, y: -8 },
        Point { x:  3, y: 16 }, Point { x: 12, y: 13 }, Point { x:  3, y: -4 }, Point { x: 17, y:  5 }, Point { x: -3, y: 15 },
        Point { x: -3, y: -9 }, Point { x:  0, y: 11 }, Point { x: -9, y: -3 }, Point { x: -4, y: -2 }, Point { x: 12, y: 10 },
    ];

    let hull = convex_hull(&points);
    println!("Convex Hull: ");
    print(hull);
}