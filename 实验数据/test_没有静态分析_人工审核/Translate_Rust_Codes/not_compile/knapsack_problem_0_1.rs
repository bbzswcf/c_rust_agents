struct Item {
    name: &'static str,
    weight: i32,
    value: i32,
}

const ITEMS: [Item; 22] = [
    Item { name: "map", weight: 9, value: 150 },
    Item { name: "compass", weight: 13, value: 35 },
    Item { name: "water", weight: 153, value: 200 },
    Item { name: "sandwich", weight: 50, value: 160 },
    Item { name: "glucose", weight: 15, value: 60 },
    Item { name: "tin", weight: 68, value: 45 },
    Item { name: "banana", weight: 27, value: 60 },
    Item { name: "apple", weight: 39, value: 40 },
    Item { name: "cheese", weight: 23, value: 30 },
    Item { name: "beer", weight: 52, value: 10 },
    Item { name: "suntan cream", weight: 11, value: 70 },
    Item { name: "camera", weight: 32, value: 30 },
    Item { name: "T-shirt", weight: 24, value: 15 },
    Item { name: "trousers", weight: 48, value: 10 },
    Item { name: "umbrella", weight: 73, value: 40 },
    Item { name: "waterproof trousers", weight: 42, value: 70 },
    Item { name: "waterproof overclothes", weight: 43, value: 75 },
    Item { name: "note-case", weight: 22, value: 80 },
    Item { name: "sunglasses", weight: 7, value: 20 },
    Item { name: "towel", weight: 18, value: 12 },
    Item { name: "socks", weight: 4, value: 50 },
    Item { name: "book", weight: 30, value: 10 },
];

fn knapsack(items: &[Item], n: usize, w: i32) -> Vec<bool> {
    let mut mm = vec![0; (n + 1) * (w as usize + 1)];
    // Modified: Initialize `m` without using the `vec!` macro
    let mut m: Vec<&mut [i32]> = Vec::with_capacity(n + 1);
    m.push(&mut mm[..]);
    for i in 1..=n {
        // Modified: Use `split_at_mut` to ensure non-overlapping mutable borrows
        let (left, right) = mm.split_at_mut((i + 1) * (w as usize + 1));
        m.push(&mut right[..(w as usize + 1)]);
        for j in 0..=w as usize {
            if items[i - 1].weight > j as i32 {
                m[i][j] = m[i - 1][j];
            } else {
                let a = m[i - 1][j];
                let b = m[i - 1][j - items[i - 1].weight as usize] + items[i - 1].value;
                m[i][j] = if a > b { a } else { b };
            }
        }
    }
    let mut s = vec![false; n];
    let mut j = w as usize;
    for i in (1..=n).rev() {
        // Modified: Ensure the indices used to access `m` are within bounds
        if i > 0 && j < items[i - 1].weight as usize {
            continue;
        }
        if m[i][j] > m[i - 1][j] {
            s[i - 1] = true;
            j -= items[i - 1].weight as usize;
        }
    }
    s
}

fn main() {
    let n = ITEMS.len();
    let s = knapsack(&ITEMS, n, 400);
    let mut tw = 0;
    let mut tv = 0;
    for i in 0..n {
        if s[i] {
            println!("{:<22} {:5} {:5}", ITEMS[i].name, ITEMS[i].weight, ITEMS[i].value);
            tw += ITEMS[i].weight;
            tv += ITEMS[i].value;
        }
    }
    println!("{:<22} {:5} {:5}", "totals:", tw, tv);
}