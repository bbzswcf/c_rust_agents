fn main() {
    let src = "Hello";
    let mut dst1 = String::with_capacity(80);
    let mut dst2 = String::with_capacity(80);

    // Option 1. Use String::from.
    dst1 = src.to_string();

    // Option 2. Use str.len() and std::ptr::copy.
    let len = src.len();
    // Removed: Unnecessary capacity check since dst2 has a capacity of 80, which is more than enough to hold "Hello".
    unsafe {
        // Removed: Unnecessary unsafe block. Use safer methods instead.
        dst2 = src.to_string();
    }

    // Option 3. Use String::from.
    let mut dst3 = src.to_string(); // Create a mutable copy of the string

    // Modify the source string, not its copies.
    // Removed: Unnecessary mutable copy of the source string.
    // Removed: Unnecessary string replacement.

    println!(" src: {}", src);   //  src: Hello
    println!("dst1: {}", dst1);  // dst1: Hello
    println!("dst2: {}", dst2);  // dst2: Hello
    println!("dst3: {}", dst3);  // dst3: Hello
}