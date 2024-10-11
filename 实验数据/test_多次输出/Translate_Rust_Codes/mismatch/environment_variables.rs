fn main() {
    if let Some(home) = std::env::var("HOME").ok() {
        println!("{}", home);
    }
}