use std::mem;

fn swap<T>(va: &mut T, vb: &mut T) {
    mem::swap(va, vb);
}

#[derive(Debug)]
struct Test {
    a: i32,
    b: i32,
    c: i32,
}

fn main() {
    let mut t = Test { a: 1, b: 2, c: 3 };
    let mut h = Test { a: 4, b: 5, c: 6 };
    let mut alfa = 0.45;
    let mut omega = 9.98;

    // Perform the swap operation before creating mutable references
    println!("{} {} {}", t.a, t.b, t.c);
    swap(&mut t, &mut h);
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    // Modified: Print floating-point numbers with 6 decimal places
    println!("{:.6}", alfa);
    swap(&mut alfa, &mut omega);
    println!("{:.6}", alfa);

    // Create mutable references after the swap operation
    let mut pt = &mut t;
    let mut th = &mut h;

    // Print values before calling swap on mutable references
    println!("{}", pt.a);
    swap(&mut pt, &mut th);
    // Print values after swap to avoid using invalidated references
    println!("{}", pt.a);
}