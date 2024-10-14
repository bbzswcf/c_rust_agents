use std::cell::RefCell;

// Modified: Changed the type bound to u8, which implements AddAssign for ASCII characters
fn accumulator<T: Copy + std::ops::AddAssign>(initial: T) -> impl FnMut(T) -> T {
    let state = RefCell::new(initial);
    move |i: T| {
        let mut state = state.borrow_mut();
        *state += i;
        *state
    }
}

fn main() {
    let mut x = accumulator(1.0f64);
    let mut y = accumulator(3i32);
    // Modified: Changed the type to u8 to ensure it supports addition
    let mut z = accumulator('a' as u8);

    // Modified: Use format specifier to ensure floating-point numbers are printed with desired precision
    println!("{:.6}", x(5.0));   // 6.000000
    println!("{:.6}", x(2.3));   // 8.300000
    println!("{}", y(5));     // 8
    println!("{}", y(3));     // 11

    // Modified: Ensure the character conversion is correctly handled
    // Directly convert the result of z(5) to a character
    println!("{}", (z(5) as char)); // f
}