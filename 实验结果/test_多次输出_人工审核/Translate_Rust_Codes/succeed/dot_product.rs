fn dot_product(a: &[i32], b: &[i32]) -> i32 {
    a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum()
}

fn main() {
    let a = [1, 3, -5];
    let b = [4, -2, -1];

    println!("{}", dot_product(&a, &b));
}