use std::alloc::{alloc, dealloc, Layout};

fn bell_index(row: i32, col: i32) -> usize {
    (row * (row - 1) / 2 + col) as usize
}

fn get_bell(bell_tri: &[i32], row: i32, col: i32) -> i32 {
    let index = bell_index(row, col);
    bell_tri[index]
}

fn set_bell(bell_tri: &mut [i32], row: i32, col: i32, value: i32) {
    let index = bell_index(row, col);
    bell_tri[index] = value;
}

fn bell_triangle(n: i32) -> *mut i32 {
    let length = (n * (n + 1) / 2) as usize;
    let layout = Layout::array::<i32>(length).unwrap();
    let tri = unsafe { alloc(layout) as *mut i32 };

    unsafe {
        for i in 0..length {
            *tri.add(i) = 0;
        }
    }

    unsafe {
        set_bell(std::slice::from_raw_parts_mut(tri, length), 1, 0, 1);
        for i in 2..=n {
            set_bell(std::slice::from_raw_parts_mut(tri, length), i, 0, get_bell(std::slice::from_raw_parts(tri, length), i - 1, i - 2));
            for j in 1..i {
                let value = get_bell(std::slice::from_raw_parts(tri, length), i, j - 1) + get_bell(std::slice::from_raw_parts(tri, length), i - 1, j - 1);
                set_bell(std::slice::from_raw_parts_mut(tri, length), i, j, value);
            }
        }
    }

    tri
}

fn main() {
    const ROWS: i32 = 15;
    let bt = bell_triangle(ROWS);

    println!("First fifteen Bell numbers:");
    for i in 1..=ROWS {
        println!("{:2}: {}", i, get_bell(unsafe { std::slice::from_raw_parts(bt, (ROWS * (ROWS + 1) / 2) as usize) }, i, 0));
    }

    println!("\nThe first ten rows of Bell's triangle:");
    for i in 1..=10 {
        print!("{}", get_bell(unsafe { std::slice::from_raw_parts(bt, (ROWS * (ROWS + 1) / 2) as usize) }, i, 0));
        for j in 1..i {
            print!(", {}", get_bell(unsafe { std::slice::from_raw_parts(bt, (ROWS * (ROWS + 1) / 2) as usize) }, i, j));
        }
        println!();
    }

    let length = (ROWS * (ROWS + 1) / 2) as usize;
    let layout = Layout::array::<i32>(length).unwrap();
    unsafe {
        dealloc(bt as *mut u8, layout);
    }
}