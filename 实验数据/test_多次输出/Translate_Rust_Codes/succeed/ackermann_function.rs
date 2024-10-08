fn ackermann(m: i32, n: i32) -> i32 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        ackermann(m - 1, 1)
    } else {
        ackermann(m - 1, ackermann(m, n - 1))
    }
}

fn main() {
    for m in 0..=3 {
        for n in 0..(7 - m) {
            println!("A({}, {}) = {}", m, n, ackermann(m, n));
        }
    }
}