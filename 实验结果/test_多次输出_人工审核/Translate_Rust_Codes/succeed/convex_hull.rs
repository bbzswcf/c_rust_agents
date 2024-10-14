#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    data: Point,
    next: Option<Box<Node>>,
}

fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - a.y) > (b.y - a.y) * (c.x - a.x)
}

fn comp(lhs: &Point, rhs: &Point) -> std::cmp::Ordering {
    lhs.x.cmp(&rhs.x)
}

fn free_node(mut ptr: Option<Box<Node>>) {
    while let Some(node) = ptr {
        ptr = node.next;
    }
}

fn push_back(ptr: Option<Box<Node>>, data: Point) -> Option<Box<Node>> {
    if let Some(mut node) = ptr {
        let mut current = &mut node;
        while let Some(ref mut next) = current.next {
            current = next;
        }
        current.next = Some(Box::new(Node { data, next: None }));
        Some(node)
    } else {
        Some(Box::new(Node { data, next: None }))
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

fn convex_hull(mut points: Vec<Point>) -> Option<Box<Node>> {
    let mut hull: Option<Box<Node>> = None;
    let mut hull_len = 0;

    points.sort_by(comp);

    for point in points.iter() {
        while hull_len >= 2 {
            let mut hptr = hull.as_ref().unwrap();
            while hptr.next.as_ref().unwrap().next.is_some() {
                hptr = hptr.next.as_ref().unwrap();
            }
            if ccw(&hptr.data, &hptr.next.as_ref().unwrap().data, point) {
                break;
            }

            hull = pop_back(hull);
            hull_len -= 1;
        }

        hull = push_back(hull, *point);
        hull_len += 1;
    }

    for point in points.iter().rev() {
        while hull_len >= 2 {
            let mut hptr = hull.as_ref().unwrap();
            while hptr.next.as_ref().unwrap().next.is_some() {
                hptr = hptr.next.as_ref().unwrap();
            }
            if ccw(&hptr.data, &hptr.next.as_ref().unwrap().data, point) {
                break;
            }

            hull = pop_back(hull);
            hull_len -= 1;
        }

        hull = push_back(hull, *point);
        hull_len += 1;
    }

    hull = pop_back(hull);
    hull
}

fn main() {
    let points = vec![
        Point { x: 16, y: 3 }, Point { x: 12, y: 17 }, Point { x: 0, y: 6 }, Point { x: -4, y: -6 }, Point { x: 16, y: 6 },
        Point { x: 16, y: -7 }, Point { x: 16, y: -3 }, Point { x: 17, y: -4 }, Point { x: 5, y: 19 }, Point { x: 19, y: -8 },
        Point { x: 3, y: 16 }, Point { x: 12, y: 13 }, Point { x: 3, y: -4 }, Point { x: 17, y: 5 }, Point { x: -3, y: 15 },
        Point { x: -3, y: -9 }, Point { x: 0, y: 11 }, Point { x: -9, y: -3 }, Point { x: -4, y: -2 }, Point { x: 12, y: 10 },
    ];

    let hull = convex_hull(points);
    println!("Convex Hull: ");
    print(&hull);
    println!();

    free_node(hull);
}