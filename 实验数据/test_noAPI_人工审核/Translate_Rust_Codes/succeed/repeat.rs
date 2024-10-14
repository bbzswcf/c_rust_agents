fn repeat(f: fn(), n: u32) {
    for _ in 0..n {
        f();
    }
}

fn example() {
    println!("Example");
}

fn main() {
    repeat(example, 4);
}