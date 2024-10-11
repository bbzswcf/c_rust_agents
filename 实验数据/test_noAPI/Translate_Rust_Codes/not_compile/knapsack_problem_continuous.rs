use std::cmp::Ordering;

struct Item {
    w: f64,
    v: f64,
    name: &'static str,
}

const ITEMS: [Item; 9] = [
    Item { w: 3.8, v: 36.0, name: "beef" },
    Item { w: 5.4, v: 43.0, name: "pork" },
    Item { w: 3.6, v: 90.0, name: "ham" },
    Item { w: 2.4, v: 45.0, name: "greaves" },
    Item { w: 4.0, v: 30.0, name: "flitch" },
    Item { w: 2.5, v: 56.0, name: "brawn" },
    Item { w: 3.7, v: 67.0, name: "welt" },
    Item { w: 3.0, v: 95.0, name: "salami" },
    Item { w: 5.9, v: 98.0, name: "sausage" },
];

fn main() {
    let mut items = ITEMS.to_vec();

    // Modified: Use a closure directly to satisfy the `Fn` trait requirement
    items.sort_by(|a, b| {
        let ua = a.v / a.w;
        let ub = b.v / b.w;
        const EPSILON: f64 = 1e-9; // Added: Small epsilon value for floating-point comparison
        if (ua - ub).abs() < EPSILON {
            Ordering::Equal
        } else if ua < ub {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let mut space = 15.0;
    for item in items.iter().rev() {
        if space <= 0.0 {
            break;
        }
        if space >= item.w {
            println!("take all {}", item.name);
        } else {
            println!("take {}kg of {} kg of {}", space, item.w, item.name);
        }
        // Modified: Ensure `space` does not underflow
        if space >= item.w {
            space -= item.w;
        } else {
            space = 0.0;
        }
    }
}