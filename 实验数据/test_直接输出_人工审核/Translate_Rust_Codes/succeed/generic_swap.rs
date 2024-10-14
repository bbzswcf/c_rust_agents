use std::mem;

fn swap(va: *mut u8, vb: *mut u8, s: usize) {
    unsafe {
        let mut t;
        let a = va as *mut u8;
        let b = vb as *mut u8;
        for i in 0..s {
            t = *a.add(i);
            *a.add(i) = *b.add(i);
            *b.add(i) = t;
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

    let mut pt: *mut Test = &mut t;
    let mut th: *mut Test = &mut h;

    println!("{} {} {}", t.a, t.b, t.c);
    mem::swap(&mut t, &mut h);
    println!("{} {} {}", t.a, t.b, t.c);
    println!("{} {} {}", h.a, h.b, h.c);

    println!("{:.2}", alfa);
    mem::swap(&mut alfa, &mut omega);
    println!("{:.2}", alfa);

    unsafe {
        println!("{}", (*pt).a);
        mem::swap(&mut pt, &mut th);
        println!("{}", (*pt).a);
    }
}