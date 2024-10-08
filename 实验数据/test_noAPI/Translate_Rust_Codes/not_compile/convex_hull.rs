#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Node {
    data: Point,
    next: Option<Box<Node>>,
}

// Modified: Correctly determine the counter-clockwise orientation using cross product
fn ccw(a: &Point, b: &Point, c: &Point) -> bool {
    (b.x - a.x) * (c.y - a.y) > (b.y - a.y) * (c.x - a.x)
}

// Modified: Correct sorting logic to handle secondary sorting by y-coordinate
fn comp(lhs: &Point, rhs: &Point) -> std::cmp::Ordering {
    if lhs.x < rhs.x {
        std::cmp::Ordering::Less
    } else if lhs.x > rhs.x {
        std::cmp::Ordering::Greater
    } else if lhs.y < rhs.y {
        std::cmp::Ordering::Less
    } else if lhs.y > rhs.y {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

fn free_node(mut ptr: Option<Box<Node>>) {
    while let Some(mut node) = ptr {
        ptr = node.next.take(); // Modified: Use Option::take to take ownership and drop the node
    }
}

// Modified: Correct handling of `current` to iterate through the linked list
fn push_back(mut ptr: Option<Box<Node>>, data: Point) -> Option<Box<Node>> {
    if ptr.is_none() {
        return Some(Box::new(Node {
            data,
            next: None,
        }));
    }

    let mut current = &mut ptr;
    while let Some(ref mut node) = current {
        if node.next.is_none() {
            node.next = Some(Box::new(Node {
                data,
                next: None,
            }));
            break;
        }
        current = &mut node.next.as_mut().unwrap(); // Modified: Update `current` to take ownership of the next node
    }

    ptr // Modified: Return the updated `ptr` after taking ownership of the `next` field
}

// Modified: Correct handling of `current` to iterate through the linked list
fn pop_back(mut ptr: Option<Box<Node>>) -> Option<Box<Node>> {
    if ptr.is_none() {
        return None;
    }

    if ptr.as_ref().unwrap().next.is_none() {
        return None;
    }

    let mut current = ptr.as_mut().unwrap();
    while let Some(node) = current.next.as_ref() {
        if node.next.is_none() {
            break;
        }
        current = current.next.as_mut().unwrap(); // Modified: Update `current` to take ownership of the next node
    }

    current.next.take(); // Modified: Use Option::take to take ownership of the next field
    ptr // Modified: Return the updated `ptr` after taking ownership of the `next` field
}

// Modified: Changed the `print` function to accept `Option<&Node>` instead of `Option<&Box<Node>>`
fn print(ptr: Option<&Node>) {
    print!("[");
    if let Some(node) = ptr {
        print!("({}, {})", node.data.x, node.data.y);
        let mut current = &node.next;
        while let Some(node) = current {
            print!(", ({}, {})", node.data.x, node.data.y);
            current = &node.next; // Modified: Directly use `node.next` reference without reassigning
        }
    }
    println!("]");
}

// Modified: Correct handling of `hptr` to iterate through the linked list
fn convex_hull(len: usize, p: &[Point]) -> Option<Box<Node>> {
    let mut h: Option<Box<Node>> = None;
    let mut h_len = 0;

    let mut points = p.to_vec();
    points.sort_by(comp);

    for i in 0..len {
        while h_len >= 2 {
            let mut hptr = h.as_ref();
            while let Some(node) = hptr {
                if let Some(next_node) = node.next.as_ref() {
                    if ccw(&node.data, &next_node.data, &points[i]) {
                        break;
                    }
                }
                hptr = node.next.as_ref(); // Modified: Use a reference to update `hptr`
            }
            h = pop_back(h);
            h_len -= 1;
        }

        h = push_back(h, points[i]);
        h_len += 1;
    }

    for i in (0..len).rev() {
        while h_len >= 2 {
            let mut hptr = h.as_ref();
            while let Some(node) = hptr {
                if let Some(next_node) = node.next.as_ref() {
                    if ccw(&node.data, &next_node.data, &points[i]) {
                        break;
                    }
                }
                hptr = node.next.as_ref(); // Modified: Use a reference to update `hptr`
            }
            h = pop_back(h);
            h_len -= 1;
        }

        h = push_back(h, points[i]);
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
        Point { x: -3, y: -9 }, Point { x:  0, y: 11 }, Point { x: -9, y: -3 }, Point { x: -4, y: -2 }, Point { x: 12, y: 10 }
    ];

    let hull = convex_hull(points.len(), &points);
    println!("Convex Hull: ");
    // Modified: Pass a reference to hull instead of cloning it
    print(hull.as_ref().map(|node| node.as_ref()));

    free_node(hull);
}