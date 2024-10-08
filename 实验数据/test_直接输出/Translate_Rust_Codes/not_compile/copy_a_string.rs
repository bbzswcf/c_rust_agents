use std::ffi::CString;
use std::ptr;
use std::process;

fn main() {
    let src = "Hello";
    let mut dst1 = vec![0u8; 80];
    let mut dst2 = vec![0u8; 80];
    let dst3;
    let ref_src;

    // Option 1. Use str::clone_from_slice.
    dst1[..src.len()].clone_from_slice(src.as_bytes());

    // Option 2. Use str.len() and slice::copy_from_slice.
    let len = src.len();
    if len >= dst2.len() {
        eprintln!("The buffer is too small!");
        process::exit(1);
    }
    dst2[..len + 1].copy_from_slice(src.as_bytes());
    dst2[len] = 0; // Null-terminate the string

    // Option 3. Use CString::new and CString::into_raw.
    dst3 = match CString::new(src) {
        Ok(cstr) => cstr,
        Err(_) => {
            eprintln!("Error: Failed to create CString");
            process::exit(1);
        }
    };

    // Create another reference to the source string.
    ref_src = src;

    // Modify the source string, not its copies.
    let mut src_bytes = src.as_bytes().to_vec();
    for i in 0..5 {
        src_bytes[i] = b'-';
    }
    let modified_src = String::from_utf8_lossy(&src_bytes);

    println!(" src: {}", modified_src);  //  src: -----
    println!("dst1: {}", String::from_utf8_lossy(&dst1));  // dst1: Hello
    println!("dst2: {}", String::from_utf8_lossy(&dst2));  // dst2: Hello
    println!("dst3: {}", dst3.to_str().unwrap());  // dst3: Hello
    println!(" ref: {}", modified_src);  //  ref: -----

    // Free memory from strdup().
    // In Rust, the memory is automatically managed, so no explicit free is needed.
}