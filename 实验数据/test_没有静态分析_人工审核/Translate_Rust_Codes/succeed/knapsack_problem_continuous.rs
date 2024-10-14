#[derive(Clone)] // Added: Implement the Clone trait for the Item struct
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

fn item_cmp(a: &Item, b: &Item) -> std::cmp::Ordering {
    let ua = a.v / a.w;
    let ub = b.v / b.w;
    if ua < ub {
        std::cmp::Ordering::Less
    } else {
        std::cmp::Ordering::Greater
    }
}

fn main() {
    let mut items = ITEMS.to_vec(); // Fixed: Now works because Item implements Clone
    let mut space = 15.0; // Modified: Declared `space` as mutable to allow reassignment

    items.sort_by(item_cmp);

    for item in items.iter().rev() {
        if space <= 0.0 {
            break;
        }
        if space >= item.w {
            println!("take all {}", item.name);
        } else {
            // Modified: Format the weight value to one decimal place to match the expected output
            println!("take {:.1}kg of {} kg of {}", space, item.w, item.name);
        }
        space -= item.w; // Modified: Now allowed because `space` is mutable
    }
}