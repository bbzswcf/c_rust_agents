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

fn knapsack(items: &[Item], n: usize, w: i32) -> Vec<i32> {
    // Ensure the numeric type is explicitly specified as usize
    let mut mm = vec![0_usize; (n + 1) * (w as usize + 1)];
    // Ensure the type of the vector matches the expected type usize
    let mut m = vec![0_usize; (n + 1)];
    // Ensure the pointer arithmetic is performed with the correct types
    m[0] = mm.as_ptr() as *mut usize;
    unsafe {
        for i in 1..=n {
            // Ensure the pointer arithmetic is performed with the correct types
            m[i] = m[0].add(i * (w as usize + 1));
            for j in 0..=w as usize {
                // Ensure the comparison is performed with the correct types
                if items[i - 1].weight > j as i32 {
                    // Ensure the pointer dereference is performed with the correct types
                    *m[i].add(j) = *m[i - 1].add(j) as usize;
                } else {
                    // Ensure the pointer dereference is performed with the correct types
                    let a = *m[i - 1].add(j) as usize;
                    // Ensure the pointer dereference is performed with the correct types
                    let b = *m[i - 1].add(j - items[i - 1].weight as usize) as usize + items[i - 1].value as usize;
                    // Ensure the pointer dereference is performed with the correct types
                    *m[i].add(j) = if a > b { a } else { b };
                }
            }
        }
    }
    let mut s = vec![0; n];
    let mut j = w as usize;
    for i in (1..=n).rev() {
        // Ensure the pointer dereference is performed with the correct types
        if unsafe { *m[i].add(j) as usize > *m[i - 1].add(j) as usize } {
            s[i - 1] = 1;
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
        if s[i] == 1 {
            println!("{:<22} {:>5} {:>5}", ITEMS[i].name, ITEMS[i].weight, ITEMS[i].value);
            tw += ITEMS[i].weight;
            tv += ITEMS[i].value;
        }
    }
    println!("{:<22} {:>5} {:>5}", "totals:", tw, tv);
}