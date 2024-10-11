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

    // Option 2. Use str.len() and ptr::copy_nonoverlapping.
    let len = src.len();
    if len >= dst2.len() {
        eprintln!("The buffer is too small!");
        process::exit(1);
    }
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst2.as_mut_ptr(), len + 1);
    }

    // Option 3. Use CString::new.
    dst3 = match CString::new(src) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Failed to allocate memory!");
            process::exit(1);
        }
    };

    // Create another reference to the source string.
    ref_src = src;

    // Modify the source string, not its copies.
    let modified_src = "-".repeat(5);

    println!(" src: {}", modified_src);  //  src: -----
    println!("dst1: {}", String::from_utf8_lossy(&dst1));  // dst1: Hello
    println!("dst2: {}", String::from_utf8_lossy(&dst2));  // dst2: Hello
    println!("dst3: {}", dst3.to_str().unwrap());  // dst3: Hello
    println!(" ref: {}", modified_src);  //  ref: -----
}