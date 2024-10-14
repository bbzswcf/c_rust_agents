use std::env;

fn main() {
    // Modified: Changed return type to unit type `()` for simplicity in main function
    if let Some(home) = env::var_os("HOME") {
        // Modified: Use {:?} for formatting OsString, which implements Debug
        println!("{:?}", home);
    }
    // Removed: Unnecessary Ok(()) since main now returns `()`
}