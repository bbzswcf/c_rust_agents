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
    } else if lhs.x > rhs.x {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn free_node(mut ptr: Option<Box<Node>>) {
    while let Some(mut node) = ptr {
        ptr = node.next.take();
    }
}

fn push_back(ptr: Option<Box<Node>>, data: Point) -> Option<Box<Node>> {
    if let Some(mut node) = ptr {
        let mut current = &mut node;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(Node {
            data,
            next: None,
        }));
        Some(node)
    } else {
        Some(Box::new(Node {
            data,
            next: None,
        }))
    }
}

fn pop_back(mut ptr: Option<Box<Node>>) -> Option<Box<Node>> {
    if ptr.is_none() {
        return None;
    }
    if ptr.as_ref().unwrap().next.is_none() {
        return None;
    }

    let mut current = &mut ptr;
    while current.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
        current = &mut current.as_mut().unwrap().next;
    }

    current.as_mut().unwrap().next = None;
    ptr
}

fn print(ptr: &Option<Box<Node>>) {
    print!("[");
    if let Some(node) = ptr {
        print!("({}, {})", node.data.x, node.data.y);
        let mut current = &node.next;
        while let Some(ref next) = current {
            print!(", ({}, {})", next.data.x, next.data.y);
            current = &next.next;
        }
    }
    print!("]");
}

fn convex_hull(points: &[Point]) -> Option<Box<Node>> {
    let mut h: Option<Box<Node>> = None;
    let mut h_len = 0;

    let mut sorted_points = points.to_vec();
    sorted_points.sort_by(comp);

    for &p in &sorted_points {
        while h_len >= 2 {
            let mut hptr = &mut h;
            while hptr.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
                hptr = &mut hptr.as_mut().unwrap().next;
            }
            if ccw(&hptr.as_ref().unwrap().data, &hptr.as_ref().unwrap().next.as_ref().unwrap().data, &p) {
                break;
            }

            h = pop_back(h);
            h_len -= 1;
        }

        h = push_back(h, p);
        h_len += 1;
    }

    for &p in sorted_points.iter().rev() {
        while h_len >= 2 {
            let mut hptr = &mut h;
            while hptr.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
                hptr = &mut hptr.as_mut().unwrap().next;
            }
            if ccw(&hptr.as_ref().unwrap().data, &hptr.as_ref().unwrap().next.as_ref().unwrap().data, &p) {
                break;
            }

            h = pop_back(h);
            h_len -= 1;
        }

        h = push_back(h, p);
        h_len += 1;
    }

    h = pop_back(h);
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
    print(&hull);
    println!();

    free_node(hull);
}