fn x(i: f64) -> f64 {
    static mut _n: f64 = 1.0;
    unsafe {
        _n += i;
        _n
    }
}

fn y(i: i32) -> i32 {
    static mut _n: i32 = 3;
    unsafe {
        _n += i;
        _n
    }
}

fn z(i: u8) -> u8 {
    static mut _n: u8 = b'a';
    unsafe {
        _n += i;
        _n
    }
}

fn main() {
    println!("{}", x(5.0));   // 6.000000
    println!("{}", x(2.3));   // 8.300000
    println!("{}", y(5));     // 8
    println!("{}", y(3));     // 11
    println!("{}", z(5) as char); // f
}