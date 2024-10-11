fn main() {
    // Handle the case where the "HOME" environment variable is not set
    if let Ok(home) = std::env::var("HOME") {
        // Print the value of the "HOME" environment variable when it is set
        println!("{}", home);
    } else {
        println!("HOME environment variable is not set");
    }
}