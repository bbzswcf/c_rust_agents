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

fn z(i: u8) -> char {
    static mut _n: u8 = b'a';
    unsafe {
        _n += i;
        _n as char
    }
}

fn main() {
    print!("{:.6}\n", x(5.0));   // 6.000000
    print!("{:.6}\n", x(2.3));   // 8.300000
    print!("{}\n", y(5));        // 8
    print!("{}\n", y(3));        // 11
    print!("{}\n", z(5));        // f
}