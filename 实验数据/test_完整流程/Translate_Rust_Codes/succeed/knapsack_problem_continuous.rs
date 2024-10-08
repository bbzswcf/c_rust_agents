#[derive(Clone)] // Added: Implement the `Clone` trait for the `Item` struct
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

// Modified: Added a case for equality to satisfy the trait bound for `sort_by`
fn item_cmp(a: &Item, b: &Item) -> std::cmp::Ordering {
    let ua = a.v / a.w;
    let ub = b.v / b.w;
    if (ua - ub).abs() > EPSILON { // Modified: Added epsilon for floating-point comparison
        if ua < ub {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    } else {
        std::cmp::Ordering::Equal
    }
}

const EPSILON: f64 = 1e-9; // Added: Small epsilon value for floating-point comparison

fn main() {
    let mut items = ITEMS.to_vec(); // Modified: `Item` now implements `Clone`, satisfying the trait bound
    items.sort_by(item_cmp);

    let mut space = 15.0;
    for item in items.iter().rev() {
        if space <= 0.0 {
            break;
        }
        if space >= item.w {
            println!("take all {}", item.name);
            space -= item.w;
        } else {
            // Modified: Ensure consistent formatting for floating-point numbers
            println!("take {:.1}kg of {:.1} kg of {}", space, item.w, item.name);
            // Modified: Adjust the logic to correctly handle the remaining space when taking partial items
            space = 0.0;
        }
    }
}