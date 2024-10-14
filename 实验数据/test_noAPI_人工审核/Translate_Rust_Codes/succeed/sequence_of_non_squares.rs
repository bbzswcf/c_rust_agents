use std::f64;

fn nonsqr(n: i32) -> i32 {
    n + (0.5 + (n as f64).sqrt()).floor() as i32
}

fn main() {
    for i in 1..23 {
        print!("{} ", nonsqr(i));
    }
    println!();

    for i in 1..1000000 {
        let j = (nonsqr(i) as f64).sqrt();
        assert!((j - j.floor()).abs() > f64::EPSILON);
    }
}