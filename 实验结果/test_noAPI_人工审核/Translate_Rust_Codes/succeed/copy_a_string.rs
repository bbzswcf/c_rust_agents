use std::error::Error;
use std::ffi::CString;
use std::ptr;
use std::slice;
use std::str;

fn main() -> Result<(), Box<dyn Error>> {
    let src = CString::new("Hello")?;
    // Modified: Use `src.as_bytes()` to get the correct slice without the null terminator
    let src_bytes = src.as_bytes();
    let src_str = str::from_utf8(src_bytes)?;

    let mut dst1 = [0u8; 80];
    let mut dst2 = [0u8; 80];
    let dst3;
    let ref_src;

    // Option 1. Use strcpy() equivalent.
    // Modified: Ensure the destination slice length matches the source slice length, excluding the null terminator
    dst1[..src_bytes.len()].copy_from_slice(src_bytes);

    // Option 2. Use strlen() and memcpy() equivalents.
    let len = src_str.len();
    // Modified: Ensure the length check includes the null terminator
    if len + 1 >= dst2.len() {
        eprintln!("The buffer is too small!");
        std::process::exit(1);
    }
    // Modified: Ensure the destination slice length matches the source slice length, including the null terminator
    dst2[..src_bytes.len() + 1].copy_from_slice(&src.as_bytes_with_nul());

    // Option 3. Use strdup() equivalent.
    dst3 = CString::new(src_str)?;

    // Create another reference to the source string.
    ref_src = src_str;

    // Modify the source string, not its copies.
    // Modified: Ensure the length excludes the null terminator
    let src_bytes_mut = unsafe { slice::from_raw_parts_mut(src_bytes.as_ptr() as *mut u8, src_bytes.len()) };
    for byte in src_bytes_mut.iter_mut() {
        *byte = b'-';
    }

    // Modified: Ensure the length excludes the null terminator
    println!(" src: {}", str::from_utf8(src_bytes)?); //  src: -----
    // Modified: Ensure the length excludes the null terminator
    println!("dst1: {}", str::from_utf8(&dst1[..src_bytes.len()])?); // dst1: -----
    // Modified: Ensure the length excludes the null terminator
    println!("dst2: {}", str::from_utf8(&dst2[..src_bytes.len()])?); // dst2: -----
    println!("dst3: {}", dst3.to_str()?); // dst3: Hello
    println!(" ref: {}", ref_src); //  ref: -----

    // Free memory from strdup() equivalent.
    drop(dst3);

    Ok(())
}