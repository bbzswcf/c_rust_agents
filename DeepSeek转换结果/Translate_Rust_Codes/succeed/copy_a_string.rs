use std::ffi::{CString, CStr};
use std::ptr;
use std::process;
use std::os::raw::c_char;

fn main() {
    let src = CString::new("Hello").unwrap();
    let src_ptr = src.as_ptr();
    let mut dst1: [c_char; 80] = [0; 80];
    let mut dst2: [c_char; 80] = [0; 80];
    let dst3: *mut c_char;
    let ref_ptr: *const c_char;

    // Option 1. Use strcpy() from <string.h>.
    unsafe {
        ptr::copy_nonoverlapping(src_ptr as *const u8, dst1.as_mut_ptr() as *mut u8, 6);
    }

    // Option 2. Use strlen() and memcpy() from <string.h>, to copy strlen(src) + 1 bytes including the final '\0'.
    let len = src.to_bytes().len();
    if len >= dst2.len() {
        eprint!("The buffer is too small!\n");
        process::exit(1);
    }
    unsafe {
        ptr::copy_nonoverlapping(src_ptr as *const u8, dst2.as_mut_ptr() as *mut u8, len + 1);
    }

    // Option 3. Use strdup() from <string.h>, to allocate a copy.
    dst3 = unsafe { CString::from_raw(CString::new(src.to_bytes()).unwrap().into_raw()) }.into_raw();
    if dst3.is_null() {
        eprintln!("{}", std::io::Error::last_os_error());
        process::exit(1);
    }

    // Create another reference to the source string.
    ref_ptr = src_ptr;

    // Modify the source string, not its copies.
    unsafe {
        ptr::write_bytes(src_ptr as *mut u8, b'-' as u8, 5);
    }

    print!(" src: {}\n", unsafe { CStr::from_ptr(src_ptr) }.to_str().unwrap());  //  src: -----
    print!("dst1: {}\n", unsafe { CStr::from_ptr(dst1.as_ptr()) }.to_str().unwrap());  // dst1: Hello
    print!("dst2: {}\n", unsafe { CStr::from_ptr(dst2.as_ptr()) }.to_str().unwrap());  // dst2: Hello
    print!("dst3: {}\n", unsafe { CStr::from_ptr(dst3) }.to_str().unwrap());  // dst3: Hello
    print!(" ref: {}\n", unsafe { CStr::from_ptr(ref_ptr) }.to_str().unwrap());  //  ref: -----

    // Free memory from strdup().
    unsafe {
        CString::from_raw(dst3);
    }
}