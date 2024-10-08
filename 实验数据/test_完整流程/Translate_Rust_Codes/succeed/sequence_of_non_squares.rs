fn nonsqr(n: i32) -> i32 {
    n + (0.5 + f64::sqrt(n as f64)) as i32
}

fn main() {
    for i in 1..23 {
        print!("{} ", nonsqr(i));
    }
    println!();

    for i in 1..1000000 {
        let j = f64::sqrt(nonsqr(i) as f64);
        assert!(j != j.floor());
    }
}