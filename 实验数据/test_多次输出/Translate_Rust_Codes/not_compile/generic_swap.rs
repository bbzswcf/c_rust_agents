#[derive(Debug, Clone, Copy)]
struct Test {
    a: i32,
    b: i32,
    c: i32,
}

fn swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut temp: T = std::mem::uninitialized();
        std::ptr::copy_nonoverlapping(x, &mut temp, 1);
        std::ptr::copy_nonoverlapping(y, x, 1);
        std::ptr::copy_nonoverlapping(&temp, y, 1);
        std::mem::forget(temp);
    }
}

fn main() {
    let mut t = Test { a: 1, b: 2, c: 3 };
    let mut h = Test { a: 4, b: 5, c: 6 };
    let mut alfa = 0.45f64;
    let mut omega = 9.98f64;

    let mut pt = &mut t;
    let mut th = &mut h;

    println!("{} {} {}", t.a, t.b, t.c);
    swap(&mut t, &mut h);
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    println!("{:.2}", alfa);
    swap(&mut alfa, &mut omega);
    println!("{:.2}", alfa);

    println!("{}", pt.a);
    swap(&mut pt, &mut th);
    println!("{}", pt.a);
}