use std::error::Error;
use std::ffi::CString;
use std::ptr;
use std::slice;

fn string_repeat(n: usize, s: &str) -> Result<CString, Box<dyn Error>> {
    let slen = s.len();
    let dest_len = n * slen + 1;
    let mut dest = vec![0u8; dest_len];

    unsafe {
        let s_ptr = s.as_ptr();
        let mut p = dest.as_mut_ptr();

        for _ in 0..n {
            ptr::copy_nonoverlapping(s_ptr, p, slen);
            p = p.add(slen);
        }
        *p = 0;
    }

    Ok(CString::from_vec_with_nul(dest)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result = string_repeat(5, "ha")?;
    println!("{}", result.to_str()?);
    Ok(())
}