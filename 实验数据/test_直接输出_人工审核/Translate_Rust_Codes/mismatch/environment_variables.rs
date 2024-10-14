fn main() {
    if let Some(home) = std::env::var_os("HOME") {
        println!("{}", home.to_string_lossy());
    }
}