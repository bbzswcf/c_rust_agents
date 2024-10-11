fn main() {
    let fruit: [&str; 2] = ["apples", "oranges"];

    let length = std::mem::size_of_val(&fruit) / std::mem::size_of_val(&fruit[0]);

    println!("{}", length);
}