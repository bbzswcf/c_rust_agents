#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone)] // Added: Derive Clone trait for Node
struct Node {
    data: Point,
    next: Option<Box<Node>>,
}

// Modified: Changed return type to i32 to handle collinear points
fn ccw(a: &Point, b: &Point, c: &Point) -> i32 {
    let val = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if val == 0 {
        0
    } else if val > 0 {
        1
    } else {
        -1
    }
}

fn comp(lhs: &Point, rhs: &Point) -> std::cmp::Ordering {
    if lhs.x < rhs.x || (lhs.x == rhs.x && lhs.y < rhs.y) {
        std::cmp::Ordering::Less
    } else if lhs.x > rhs.x || (lhs.x == rhs.x && lhs.y > rhs.y) {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn push_back(mut ptr: Option<Box<Node>>, data: Point) -> Option<Box<Node>> {
    if ptr.is_none() {
        return Some(Box::new(Node {
            data,
            next: None,
        }));
    }

    let mut current = ptr.as_mut().unwrap();
    while current.next.is_some() {
        current = current.next.as_mut().unwrap();
    }

    current.next = Some(Box::new(Node {
        data,
        next: None,
    }));

    ptr // Modified: Return the original head of the list
}

fn pop_back(mut ptr: Option<Box<Node>>) -> Option<Box<Node>> {
    if ptr.is_none() {
        return None;
    }

    if ptr.as_ref().unwrap().next.is_none() {
        return None;
    }

    let mut current = ptr.as_mut().unwrap();
    while current.next.as_ref().unwrap().next.is_some() {
        current = current.next.as_mut().unwrap();
    }

    current.next = None;
    ptr
}

// Modified: Correctly format the output to match the expected format
fn print(ptr: &Option<Box<Node>>) {
    print!("[");
    if let Some(node) = ptr {
        print!("({}, {})", node.data.x, node.data.y);
        let mut current = &node.next;
        while let Some(node) = current {
            print!(", ({}, {})", node.data.x, node.data.y);
            current = &node.next;
        }
    }
    println!("]");
}

// Modified: Correctly handle the removal of collinear points and the construction of the upper hull
fn convex_hull(len: usize, p: &[Point]) -> Option<Box<Node>> {
    let mut points = p.to_vec();
    points.sort_by(comp);

    let mut hull: Vec<Point> = Vec::new();

    // Build the lower hull
    for point in points.iter() {
        while hull.len() >= 2 && ccw(&hull[hull.len() - 2], &hull[hull.len() - 1], point) <= 0 {
            hull.pop();
        }
        hull.push(*point);
    }

    // Build the upper hull
    let lower_hull_size = hull.len();
    for point in points.iter().rev().skip(1) {
        while hull.len() > lower_hull_size && ccw(&hull[hull.len() - 2], &hull[hull.len() - 1], point) <= 0 {
            hull.pop();
        }
        hull.push(*point);
    }

    // Remove the last point which is the same as the first point
    hull.pop();

    // Reverse the upper hull part to maintain the correct order
    hull[lower_hull_size..].reverse();

    // Convert the hull vector to a linked list
    let mut hull_list: Option<Box<Node>> = None;
    for point in hull.iter().rev() {
        hull_list = Some(Box::new(Node {
            data: *point,
            next: hull_list,
        }));
    }

    hull_list
}

fn main() {
    let points = [
        Point { x: 16, y: 3 }, Point { x: 12, y: 17 }, Point { x: 0, y: 6 }, Point { x: -4, y: -6 },
        Point { x: 16, y: 6 }, Point { x: 16, y: -7 }, Point { x: 16, y: -3 }, Point { x: 17, y: -4 },
        Point { x: 5, y: 19 }, Point { x: 19, y: -8 }, Point { x: 3, y: 16 }, Point { x: 12, y: 13 },
        Point { x: 3, y: -4 }, Point { x: 17, y: 5 }, Point { x: -3, y: 15 }, Point { x: -3, y: -9 },
        Point { x: 0, y: 11 }, Point { x: -9, y: -3 }, Point { x: -4, y: -2 }, Point { x: 12, y: 10 },
    ];

    let hull = convex_hull(points.len(), &points);
    println!("Convex Hull: ");
    print(&hull); // Modified: Pass a reference to the list to avoid cloning
    println!();
}