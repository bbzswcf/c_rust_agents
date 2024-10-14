use std::mem;

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

fn knapsack(items: &[Item], n: usize, mut w: i32) -> Vec<i32> {
    let mut m = vec![vec![0; w as usize + 1]; n + 1];
    let mut s = vec![0; n];

    for i in 1..=n {
        for j in 0..=w {
            if items[i - 1].weight > j {
                m[i][j as usize] = m[i - 1][j as usize];
            } else {
                let a = m[i - 1][j as usize];
                let b = m[i - 1][(j as usize - items[i - 1].weight as usize)] + items[i - 1].value;
                m[i][j as usize] = if a > b { a } else { b };
            }
        }
    }

    for i in (0..n).rev() {
        if m[i + 1][w as usize] > m[i][w as usize] {
            s[i] = 1;
            let new_w = w - items[i].weight;
            if new_w < 0 {
                break;
            }
            w = new_w; // Modified: Declared `w` as mutable in the function parameter
        }
    }

    s
}

fn main() {
    let n = ITEMS.len();
    let s = knapsack(&ITEMS, n, 400);
    let mut tw = 0; // Modified: Declared `tw` as mutable
    let mut tv = 0; // Modified: Declared `tv` as mutable
    for i in 0..n {
        if s[i] != 0 {
            println!("{:<22} {:>5} {:>5}", ITEMS[i].name, ITEMS[i].weight, ITEMS[i].value);
            tw += ITEMS[i].weight; // Modified: Declared `tw` as mutable
            tv += ITEMS[i].value; // Modified: Declared `tv` as mutable
        }
    }
    println!("{:<22} {:>5} {:>5}", "totals:", tw, tv);
}