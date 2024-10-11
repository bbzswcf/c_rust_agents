use std::mem;

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

    // Print values before mutable borrow to avoid conflicts
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    // Perform the swap operation before borrowing `t` and `h` mutably again
    mem::swap(&mut t, &mut h);

    // Print values after swapping to show the changes
    // Modified: Ensure the print statements after the swap operation are in the correct order
    println!("{} {} {}", h.a, h.b, h.c);
    println!("{} {} {}", t.a, t.b, t.c);

    println!("{:.6}", alfa);
    mem::swap(&mut alfa, &mut omega);
    // Modified: Ensure the correct value is printed after the swap operation
    println!("{:.6}", omega);

    // Use mutable borrow before swapping to avoid conflicts
    let mut pt = &mut t;
    let mut th = &mut h;

    println!("{}", pt.a);
    println!("{}", th.a);

    // Perform the swap operation before borrowing `pt` and `th` mutably again
    mem::swap(&mut pt, &mut th);

    // Modified: Ensure the correct value is printed after the swap operation
    println!("{}", th.a);

    // Removed: Extra print statement at the end
}