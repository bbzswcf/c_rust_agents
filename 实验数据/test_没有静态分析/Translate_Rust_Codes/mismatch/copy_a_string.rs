use std::ffi::CString;
use std::process::exit;

fn main() {
    let src = "Hello";
    let mut dst1 = vec![0u8; 80];
    let mut dst2 = vec![0u8; 80];
    let dst3;
    let ref_src;

    // Option 1. Use str::clone_from_slice.
    if src.len() + 1 > dst1.len() {
        eprintln!("The buffer is too small!");
        exit(1);
    }
    dst1[..src.len()].clone_from_slice(src.as_bytes());
    dst1[src.len()] = 0; // Null-terminate the string

    // Option 2. Use str.len() and slice::copy_from_slice.
    let len = src.len();
    if len + 1 > dst2.len() {
        eprintln!("The buffer is too small!");
        exit(1);
    }
    dst2[..len].copy_from_slice(src.as_bytes());
    dst2[len] = 0; // Null-terminate the string

    // Option 3. Use CString::new.
    dst3 = match CString::new(src) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to allocate memory!");
            exit(1);
        }
    };

    // Create another reference to the source string.
    ref_src = src;

    // Modify the source string, not its copies.
    let mut modified_src = src.as_bytes().to_vec(); // Copy original string
    modified_src.push(0); // Null-terminate the string correctly

    println!(" src: {}", String::from_utf8_lossy(&modified_src));
    println!("dst1: {}", String::from_utf8_lossy(&dst1));
    println!("dst2: {}", String::from_utf8_lossy(&dst2));
    match dst3.to_str() {
        Ok(s) => println!("dst3: {}", s),
        Err(_) => eprintln!("Failed to convert CString to str!"),
    }
    println!(" ref: {}", ref_src); // Print the original src string
}