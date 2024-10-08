fn main() {
    let src = "Hello";
    let mut dst1 = [0u8; 80];
    let mut dst2 = [0u8; 80];
    let dst3;
    let ref_str;

    // Option 1. Use `str::copy_from_slice`.
    // DANGER! This can overflow the destination buffer.
    // This is only safe if the source string is shorter than
    // the destination buffer. We know that "Hello" (6 characters
    // with the final '\0') easily fits in dst1 (80 characters).
    dst1[..src.len()].copy_from_slice(src.as_bytes());
    dst1[src.len()] = 0; // Null-terminate the string

    // Option 2. Use `str::len` and `slice::copy_from_slice`, to copy
    // `src.len() + 1` bytes including the final '\0'.
    let len = src.len();
    if len >= dst2.len() {
        eprintln!("The buffer is too small!");
        std::process::exit(1);
    }
    dst2[..len].copy_from_slice(src.as_bytes());
    dst2[len] = 0; // Null-terminate the string

    // Option 3. Use `String::from` to allocate a copy.
    dst3 = String::from(src);

    // Create another reference to the source string.
    ref_str = src;

    // Modify the source string, not its copies.
    let mut src_bytes = [b'-'; 5];
    src_bytes.copy_from_slice(src.as_bytes());

    println!(" src: {}", std::str::from_utf8(&src_bytes).unwrap()); //  src: -----
    println!("dst1: {}", std::str::from_utf8(&dst1).unwrap());     // dst1: Hello
    println!("dst2: {}", std::str::from_utf8(&dst2).unwrap());     // dst2: Hello
    println!("dst3: {}", dst3);                                    // dst3: Hello
    println!(" ref: {}", ref_str);                                 //  ref: -----

    // Free memory from `String::from`.
    drop(dst3);
}