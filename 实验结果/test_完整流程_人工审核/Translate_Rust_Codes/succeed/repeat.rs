fn repeat<F>(f: F, n: usize)
where
    F: Fn(),
{
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