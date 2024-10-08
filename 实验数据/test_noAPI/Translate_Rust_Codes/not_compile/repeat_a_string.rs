use std::error::Error;
use std::ffi::CString;
use std::slice;

fn string_repeat(n: usize, s: &CString) -> Result<CString, Box<dyn Error>> {
    let slen = s.as_bytes().len();
    // Ensure dest_len does not exceed the maximum capacity of the vector and prevent overflow
    let dest_len = n.checked_mul(slen.checked_add(1).ok_or("Destination length overflow")?)
                    .ok_or("Destination length overflow")?;
    let mut dest = vec![0; dest_len];

    let s_bytes = s.as_bytes();
    for i in 0..n {
        let p = &mut dest[i * (slen + 1)..i * (slen + 1) + slen];
        p.copy_from_slice(s_bytes);
        // Place null byte after each segment
        dest[i * (slen + 1) + slen] = 0;
    }

    // CString::new(dest)? will check for internal null bytes, no need for additional check
    Ok(CString::new(dest)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let s = CString::new("ha")?;
    let result = string_repeat(5, &s)?;
    let c_str = result.as_c_str();
    let bytes = c_str.to_bytes();
    
    // Ensure bytes array is null-terminated and contains valid UTF-8 data
    if bytes.last() == Some(&0) && std::str::from_utf8(&bytes[..bytes.len() - 1]).is_ok() {
        // Wrap the call to slice::from_raw_parts in an unsafe block
        unsafe {
            // Ensure the length passed to slice::from_raw_parts is correct
            let str_slice = slice::from_raw_parts(bytes.as_ptr() as *const u8, bytes.len() - 1);
            // Use str_slice safely within this block
            println!("{}", std::str::from_utf8(str_slice)?);
        }
    } else {
        return Err("Invalid UTF-8 or not null-terminated".into());
    }
    
    Ok(())
}