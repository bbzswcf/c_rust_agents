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
        *n += i.round() as i32;  // Modified: Ensure the conversion from f64 to i32 is done correctly
        *n
    })
}

fn z(i: u8) -> char {
    Z.with(|n| {
        let mut n = n.borrow_mut();
        *n = ((*n as u8) + i) as char;  // Modified: Ensure the conversion from char to u8 and back to char is done correctly
        *n
    })
}

fn main() {
    println!("{:.6}", x(5.0));  // Modified: Use formatting specifiers to ensure the output matches the expected precision
    println!("{:.6}", x(2.3));  // Modified: Use formatting specifiers to ensure the output matches the expected precision
    println!("{}", y(5.0));
    println!("{}", y(3.3));
    println!("{}", z(5));
}