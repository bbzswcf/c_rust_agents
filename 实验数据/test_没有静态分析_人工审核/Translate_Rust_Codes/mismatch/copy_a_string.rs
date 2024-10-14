use std::ffi::{CStr, CString};
use std::ptr;
use std::process;

fn main() {
    let src = "Hello";
    let mut dst1 = [0u8; 80];
    let mut dst2 = [0u8; 80];
    let dst3;

    // Option 1. Use String::from.
    let src_string = src.to_string();
    let dst1_string = String::from(src);
    dst1[..src_string.len()].copy_from_slice(src_string.as_bytes());
    // Ensure null-termination is within bounds and correctly placed
    if src_string.len() < dst1.len() - 1 {
        dst1[src_string.len()] = 0;
    }

    // Option 2. Use str.len() and std::ptr::copy.
    let len = src.len();
    // Ensure length check includes null-termination character
    if len + 1 >= dst2.len() {
        eprint!("The buffer is too small!\n");
        process::exit(1);
    }
    unsafe {
        ptr::copy(src.as_ptr(), dst2.as_mut_ptr() as *mut _, len); // Copy len bytes
        // Ensure null-termination is within bounds and correctly placed
        if len < dst2.len() - 1 {
            *dst2.as_mut_ptr().add(len) = 0; // Null-terminate the string
        }
    }

    // Option 3. Use std::ffi::CString::new.
    let raw_ptr = unsafe { CString::new(src).unwrap().into_raw() };
    if raw_ptr.is_null() {
        eprintln!("{}", std::io::Error::last_os_error());
        process::exit(1);
    }
    dst3 = raw_ptr;

    // Create another reference to the source string.
    let ref_src: &str = src; // Added type annotation for ref_src

    // Modify the source string, not its copies.
    // Commented out: Modifying the source string directly is not safe.
    // unsafe {
    //     ptr::write_bytes(src.as_ptr() as *mut _, b'-', 5);
    // }

    println!(" src: {}", src);
    // Ensure dst1 is null-terminated before printing
    if dst1[src_string.len()] == 0 {
        println!("dst1: {}", unsafe { std::str::from_utf8_unchecked(&dst1) });
    } else {
        eprintln!("dst1 is not null-terminated");
    }
    // Ensure dst2 is null-terminated before printing
    if dst2[len] == 0 {
        println!("dst2: {}", unsafe { std::str::from_utf8_unchecked(&dst2) });
    } else {
        eprintln!("dst2 is not null-terminated");
    }
    let cstr = unsafe { CStr::from_ptr(dst3) };
    if let Ok(s) = cstr.to_str() {
        println!("dst3: {}", s);
    } else {
        eprintln!("Invalid UTF-8 sequence in dst3");
    }
    println!(" ref: {}", ref_src);

    // Free memory from strdup().
    unsafe {
        let _ = CString::from_raw(dst3); // Convert raw pointer back to CString to free memory
    }
}