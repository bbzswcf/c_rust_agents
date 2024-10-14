fn nonsqr(n: i32) -> i32 {
    n + (0.5 + (n as f64).sqrt()).floor() as i32
}

fn main() {
    // First 22 values (as a list) has no squares:
    for i in 1..23 {
        print!("{} ", nonsqr(i));
    }
    println!();

    // The following check shows no squares up to one million:
    for i in 1..1_000_000 {
        let j = (nonsqr(i) as f64).sqrt();
        assert_ne!(j, j.floor());
    }
}