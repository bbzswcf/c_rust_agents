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

    // Modified: Ensure the integer values are printed with a consistent format
    println!("{} {} {}", t.a, t.b, t.c); // Immutable borrow of `t`
    swap(&mut t, &mut h); // Mutable borrow of `t` and `h`
    println!("{} {} {}", t.a, t.b, t.c); // Immutable borrow of `t`
    println!("{} {} {}", h.a, h.b, h.c); // Immutable borrow of `h`

    // Modified: Ensure the floating-point numbers are printed with the desired precision
    println!("{:.6}", alfa); // Immutable borrow of `alfa`
    swap(&mut alfa, &mut omega); // Mutable borrow of `alfa` and `omega`
    println!("{:.6}", alfa); // Immutable borrow of `alfa`

    // Modified: Ensure the final print statements match the expected output order
    println!("{}", t.a); // Immutable borrow of `t`

    let mut pt = &mut t;
    let mut th = &mut h;

    swap(&mut pt, &mut th); // Mutable borrow of `pt` and `th`
    println!("{}", h.a); // Immutable borrow of `h`
}