fn accumulator<T: std::ops::AddAssign + Copy>(initial: T) -> impl FnMut(T) -> T {
    let mut n = initial;
    move |i| {
        n += i;
        n
    }
}

fn main() {
    let mut x = accumulator(1.0);
    let mut y = accumulator(3);
    let mut z = accumulator('a');

    println!("{}", x(5.0)); // 6.000000
    println!("{}", x(2.3)); // 8.300000
    println!("{}", y(5));   // 8
    println!("{}", y(3));   // 11
    println!("{}", z(5));   // f
}