// 修改: 添加 #[macro_use] 以使用 lazy_static 宏
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

#[derive(Clone)]
struct Item {
    name: String,
    weight: i32,
    value: i32,
}

// 修改: 使用 lazy_static! 宏定义 ITEMS
lazy_static! {
    static ref ITEMS: Mutex<Vec<Item>> = Mutex::new(vec![
        Item { name: String::from("map"), weight: 9, value: 150 },
        Item { name: String::from("compass"), weight: 13, value: 35 },
        Item { name: String::from("water"), weight: 153, value: 200 },
        Item { name: String::from("sandwich"), weight: 50, value: 160 },
        Item { name: String::from("glucose"), weight: 15, value: 60 },
        Item { name: String::from("tin"), weight: 68, value: 45 },
        Item { name: String::from("banana"), weight: 27, value: 60 },
        Item { name: String::from("apple"), weight: 39, value: 40 },
        Item { name: String::from("cheese"), weight: 23, value: 30 },
        Item { name: String::from("beer"), weight: 52, value: 10 },
        Item { name: String::from("suntan cream"), weight: 11, value: 70 },
        Item { name: String::from("camera"), weight: 32, value: 30 },
        Item { name: String::from("T-shirt"), weight: 24, value: 15 },
        Item { name: String::from("trousers"), weight: 48, value: 10 },
        Item { name: String::from("umbrella"), weight: 73, value: 40 },
        Item { name: String::from("waterproof trousers"), weight: 42, value: 70 },
        Item { name: String::from("waterproof overclothes"), weight: 43, value: 75 },
        Item { name: String::from("note-case"), weight: 22, value: 80 },
        Item { name: String::from("sunglasses"), weight: 7, value: 20 },
        Item { name: String::from("towel"), weight: 18, value: 12 },
        Item { name: String::from("socks"), weight: 4, value: 50 },
        Item { name: String::from("book"), weight: 30, value: 10 },
    ]);
}

fn knapsack(items: &[Item], n: usize, w: i32) -> Vec<i32> {
    let mut mm = vec![0; (n + 1) * (w as usize + 1)];
    let mut m = vec![0; (n + 1)];
    // 修改: 使用 wrapping_add 以避免溢出
    m[0] = mm.as_mut_ptr() as usize;
    for i in 1..=n {
        m[i] = (m[0]).wrapping_add(i * (w as usize + 1));
        for j in 0..=w as usize {
            if items[i - 1].weight > j as i32 {
                unsafe { *((m[i] + j) as *mut i32) = *((m[i - 1] + j) as *mut i32); }
            } else {
                let a = unsafe { *((m[i - 1] + j) as *mut i32) };
                let b = unsafe { *((m[i - 1] + (j - items[i - 1].weight as usize)) as *mut i32) + items[i - 1].value };
                unsafe { *((m[i] + j) as *mut i32) = if a > b { a } else { b }; }
            }
        }
    }
    let mut s = vec![0; n];
    let mut j = w as usize;
    for i in (1..=n).rev() {
        if unsafe { *((m[i] + j) as *mut i32) } > unsafe { *((m[i - 1] + j) as *mut i32) } {
            s[i - 1] = 1;
            j -= items[i - 1].weight as usize;
        }
    }
    s
}

fn main() {
    // 修改: 使用 unwrap_or_else 处理 Mutex::lock() 失败的情况
    let items = ITEMS.lock().unwrap_or_else(|e| {
        eprintln!("Failed to lock ITEMS: {}", e);
        std::process::exit(1);
    });
    let n = items.len();
    let s = knapsack(&items, n, 400);
    let mut tw = 0;
    let mut tv = 0;
    for i in 0..n {
        if s[i] == 1 {
            print!("{:<22} {:5} {:5}\n", items[i].name, items[i].weight, items[i].value);
            tw += items[i].weight;
            tv += items[i].value;
        }
    }
    print!("{:<22} {:5} {:5}\n", "totals:", tw, tv);
}