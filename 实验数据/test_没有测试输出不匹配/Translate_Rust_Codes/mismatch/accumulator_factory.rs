use std::cell::RefCell;

thread_local! {
    static X: RefCell<f64> = RefCell::new(1.0);
    static Y: RefCell<i32> = RefCell::new(3);
    static Z: RefCell<char> = RefCell::new('a');
}

fn x(i: f64) -> f64 {
    X.with(|n| {
        let mut n = n.borrow_mut();
        *n += i;
        *n
    })
}

fn y(i: f64) -> i32 {
    Y.with(|n| {
        let mut n = n.borrow_mut();
        *n += i as i32;
        *n
    })
}

fn z(i: u8) -> char {
    Z.with(|n| {
        let mut n = n.borrow_mut();
        *n = (*n as u8 + i) as char;
        *n
    })
}

fn main() {
    println!("{}", x(5.0));   // 6.000000
    println!("{}", x(2.3));   // 8.300000
    println!("{}", y(5.0));   // 8
    println!("{}", y(3.3));   // 11
    println!("{}", z(5));     // f
}