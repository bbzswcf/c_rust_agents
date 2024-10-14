fn main() {
    let src = "Hello";
    let mut dst1 = [0u8; 80];
    let mut dst2 = [0u8; 80];
    let dst3;
    let ref_src;

    // Option 1. Use `str::copy_within` to copy the string.
    // This is safe because we know the source string fits in the destination buffer.
    dst1[..src.len()].copy_from_slice(src.as_bytes());
    dst1[src.len()] = 0; // Null-terminate the string

    // Option 2. Use `str::len` and `slice::copy_from_slice` to copy the string.
    let len = src.len();
    if len >= dst2.len() {
        eprintln!("The buffer is too small!");
        std::process::exit(1);
    }
    dst2[..len].copy_from_slice(src.as_bytes());
    dst2[len] = 0; // Null-terminate the string

    // Option 3. Use `String::from` to create a heap-allocated copy.
    dst3 = String::from(src);

    // Create another reference to the source string.
    ref_src = src;

    // Modify the source string, not its copies.
    let mut src_bytes = src.as_bytes().to_vec();
    for byte in &mut src_bytes[..5] {
        *byte = b'-';
    }
    let modified_src = std::str::from_utf8(&src_bytes).unwrap();

    println!(" src: {}", modified_src); //  src: -----
    println!("dst1: {}", std::str::from_utf8(&dst1).unwrap()); // dst1: Hello
    println!("dst2: {}", std::str::from_utf8(&dst2).unwrap()); // dst2: Hello
    println!("dst3: {}", dst3); // dst3: Hello
    println!(" ref: {}", ref_src); //  ref: -----
}