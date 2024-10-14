#[derive(Clone)]
struct Item {
    name: String,
    weight: i32,
    value: i32,
}

fn knapsack(items: &[Item], max_weight: i32) -> Vec<bool> {
    let n = items.len();
    let mut mm = vec![0; (n + 1) * (max_weight as usize + 1)];
    let mut m = vec![&mut mm[..]; n + 1];
    m[0] = &mut mm[..max_weight as usize + 1];

    for i in 1..=n {
        m[i] = &mut mm[(i * (max_weight as usize + 1))..((i + 1) * (max_weight as usize + 1))];
        for j in 0..=max_weight {
            if items[i - 1].weight > j {
                m[i][j as usize] = m[i - 1][j as usize];
            } else {
                let a = m[i - 1][j as usize];
                let b = m[i - 1][(j - items[i - 1].weight) as usize] + items[i - 1].value;
                m[i][j as usize] = a.max(b);
            }
        }
    }

    let mut s = vec![false; n];
    let mut j = max_weight;
    for i in (1..=n).rev() {
        if m[i][j as usize] > m[i - 1][j as usize] {
            s[i - 1] = true;
            j -= items[i - 1].weight;
        }
    }

    s
}

fn main() {
    let items = vec![
        Item { name: "map".to_string(), weight: 9, value: 150 },
        Item { name: "compass".to_string(), weight: 13, value: 35 },
        Item { name: "water".to_string(), weight: 153, value: 200 },
        Item { name: "sandwich".to_string(), weight: 50, value: 160 },
        Item { name: "glucose".to_string(), weight: 15, value: 60 },
        Item { name: "tin".to_string(), weight: 68, value: 45 },
        Item { name: "banana".to_string(), weight: 27, value: 60 },
        Item { name: "apple".to_string(), weight: 39, value: 40 },
        Item { name: "cheese".to_string(), weight: 23, value: 30 },
        Item { name: "beer".to_string(), weight: 52, value: 10 },
        Item { name: "suntan cream".to_string(), weight: 11, value: 70 },
        Item { name: "camera".to_string(), weight: 32, value: 30 },
        Item { name: "T-shirt".to_string(), weight: 24, value: 15 },
        Item { name: "trousers".to_string(), weight: 48, value: 10 },
        Item { name: "umbrella".to_string(), weight: 73, value: 40 },
        Item { name: "waterproof trousers".to_string(), weight: 42, value: 70 },
        Item { name: "waterproof overclothes".to_string(), weight: 43, value: 75 },
        Item { name: "note-case".to_string(), weight: 22, value: 80 },
        Item { name: "sunglasses".to_string(), weight: 7, value: 20 },
        Item { name: "towel".to_string(), weight: 18, value: 12 },
        Item { name: "socks".to_string(), weight: 4, value: 50 },
        Item { name: "book".to_string(), weight: 30, value: 10 },
    ];

    let s = knapsack(&items, 400);
    let mut tw = 0;
    let mut tv = 0;

    for (i, &selected) in s.iter().enumerate() {
        if selected {
            println!("{:<22} {:>5} {:>5}", items[i].name, items[i].weight, items[i].value);
            tw += items[i].weight;
            tv += items[i].value;
        }
    }

    println!("{:<22} {:>5} {:>5}", "totals:", tw, tv);
}