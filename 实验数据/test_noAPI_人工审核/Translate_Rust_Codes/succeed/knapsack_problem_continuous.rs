use std::cmp::Ordering;

#[derive(Clone)]
struct Item {
    w: f64,
    v: f64,
    name: String,
}

// Modified: Corrected the comparison logic to return Ordering::Greater when ua > ub and Ordering::Less when ua < ub
fn item_cmp(a: &Item, b: &Item) -> Ordering {
    let ua = a.v / a.w;
    let ub = b.v / b.w;
    if ua > ub {
        Ordering::Greater
    } else if ua < ub {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn main() {
    let items = vec![
        Item { w: 3.8, v: 36.0, name: "beef".to_string() },
        Item { w: 5.4, v: 43.0, name: "pork".to_string() },
        Item { w: 3.6, v: 90.0, name: "ham".to_string() },
        Item { w: 2.4, v: 45.0, name: "greaves".to_string() },
        Item { w: 4.0, v: 30.0, name: "flitch".to_string() },
        Item { w: 2.5, v: 56.0, name: "brawn".to_string() },
        Item { w: 3.7, v: 67.0, name: "welt".to_string() },
        Item { w: 3.0, v: 95.0, name: "salami".to_string() },
        Item { w: 5.9, v: 98.0, name: "sausage".to_string() },
    ];

    let mut sorted_items = items.clone();
    sorted_items.sort_by(item_cmp);

    let mut space = 15.0;
    for item in sorted_items.iter().rev() {
        if space <= 0.0 {
            break;
        }
        // Modified: Ensure the subtraction only occurs if the item is fully taken
        if space >= item.w {
            println!("take all {}", item.name);
            space -= item.w;
        } else {
            // Modified: Formatted the output to limit decimal places
            println!("take {:.2}kg of {:.1} kg of {}", space, item.w, item.name);
            space = 0.0; // No need to subtract as the item is not fully taken
        }
    }
}