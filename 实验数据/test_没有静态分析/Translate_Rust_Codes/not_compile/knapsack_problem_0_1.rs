use std::alloc::{alloc, Layout};
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

fn knapsack(items: &[Item], n: usize, w: i32) -> *mut i32 {
    let layout = Layout::from_size_align((n + 1) * (w as usize + 1) * mem::size_of::<i32>(), mem::align_of::<i32>()).unwrap();
    let mm = unsafe { alloc(layout) as *mut i32 };
    let m_layout = Layout::from_size_align((n + 1) * mem::size_of::<*mut i32>(), mem::align_of::<*mut i32>()).unwrap();
    let m = unsafe { alloc(m_layout) as *mut *mut i32 };
    unsafe {
        *m = mm;
        for i in 1..=n {
            *m.add(i) = mm.add(i * (w as usize + 1));
            for j in 0..=w as usize {
                if items[i - 1].weight > j as i32 {
                    *m.add(i).add(j) = *m.add(i - 1).add(j);
                } else {
                    let a = *m.add(i - 1).add(j);
                    // Modified: Dereference the pointer to get the `i32` value before performing the addition
                    let b = *(*m.add(i - 1).add(j.wrapping_sub(items[i - 1].weight as usize))) + items[i - 1].value;
                    // Modified: Dereference the pointer to assign the `i32` value
                    *(*m.add(i).add(j)) = if a > b { a } else { b };
                }
            }
        }
    }
    let s_layout = Layout::from_size_align(n * mem::size_of::<i32>(), mem::align_of::<i32>()).unwrap();
    let s = unsafe { alloc(s_layout) as *mut i32 };
    unsafe {
        for i in (0..n).rev() {
            if *m.add(i + 1).add(w as usize) > *m.add(i).add(w as usize) {
                *s.add(i) = 1;
                let new_w = w - items[i].weight;
                if new_w < 0 {
                    break;
                }
                w = new_w;
            }
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
        unsafe {
            if *s.add(i) != 0 {
                println!("{:<22} {:>5} {:>5}", ITEMS[i].name, ITEMS[i].weight, ITEMS[i].value);
                tw += ITEMS[i].weight;
                tv += ITEMS[i].value;
            }
        }
    }
    println!("{:<22} {:>5} {:>5}", "totals:", tw, tv);
}