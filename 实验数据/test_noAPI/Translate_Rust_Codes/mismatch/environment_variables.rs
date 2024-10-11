use std::env;

fn main() {
    println!("{}", env::var("HOME").unwrap_or_else(|_| String::from("HOME not set")));
}