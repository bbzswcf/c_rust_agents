fn main() {
    if let Some(home) = std::env::var_os("HOME") {
        // Modified: Correctly call `to_string_lossy` on the result of `home.to_string_lossy()`
        // and print it directly without unnecessary conversion
        println!("{}", home.to_string_lossy());
    }
}