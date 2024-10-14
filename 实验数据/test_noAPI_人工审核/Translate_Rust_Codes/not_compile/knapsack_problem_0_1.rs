use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

#[derive(Clone)]
struct Item {
    name: &'static str, // Modified: Replaced `String` with `&'static str` to avoid non-const function call in statics
    weight: i32,
    value: i32,
}

// Modified: Replaced `String::from` with `&'static str` for the `name` field
static ITEMS: [Item; 22] = [
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
    let mm_layout = Layout::array::<i32>((n + 1) * (w as usize + 1)).unwrap();
    let mm = unsafe { alloc(mm_layout) as *mut i32 };
    let m_layout = Layout::array::<*mut i32>(n + 1).unwrap();
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
                    // Modified: Corrected pointer arithmetic
                    let b = *m.add(i - 1).add((j - items[i - 1].weight as usize) + items[i - 1].value as usize) as i32;
                    // Modified: Dereferenced the pointer on the left-hand side to match the types
                    **m.add(i).add(j) = if a > b { a } else { b };
                }
            }
        }
    }

    let s_layout = Layout::array::<i32>(n).unwrap();
    let s = unsafe { alloc(s_layout) as *mut i32 };

    unsafe {
        for i in 0..n {
            *s.add(i) = 0;
        }
        let mut j = w as usize;
        for i in (1..=n).rev() {
            if *m.add(i).add(j) > *m.add(i - 1).add(j) {
                *s.add(i - 1) = 1;
                j -= items[i - 1].weight as usize;
            }
        }
    }

    unsafe {
        dealloc(mm as *mut u8, mm_layout);
        dealloc(m as *mut u8, m_layout);
    }

    s
}

fn main() {
    let n = ITEMS.len();
    let s = knapsack(&ITEMS, n, 400);

    let mut tw = 0;
    let mut tv = 0;

    unsafe {
        for i in 0..n {
            if *s.add(i) == 1 {
                println!("{:<22} {:>5} {:>5}", ITEMS[i].name, ITEMS[i].weight, ITEMS[i].value);
                tw += ITEMS[i].weight;
                tv += ITEMS[i].value;
            }
        }
        // Modified: Used the correct layout for deallocation
        dealloc(s as *mut u8, s_layout);
    }

    println!("{:<22} {:>5} {:>5}", "totals:", tw, tv);
}