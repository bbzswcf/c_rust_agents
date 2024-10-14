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

    // Directly use `t` and `h` in the `swap` function calls to avoid multiple mutable borrows
    println!("{} {} {}", t.a, t.b, t.c);
    swap(&mut t, &mut h);
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    println!("{}", alfa);
    swap(&mut alfa, &mut omega);
    println!("{}", alfa);

    // Ensure all mutable borrows are completed before any immutable borrows
    let pt = &mut t;
    let th = &mut h;
    swap(pt, th);
    println!("{}", pt.a);
}