use std::io::{self, Write};
use std::ffi::CStr;

fn roman(mut s: &mut [u8], mut n: u32) -> Result<(), &'static str> {
    if n == 0 {
        return Err("Roman numeral for zero requested.");
    }

    // Modified: Ensure the buffer is large enough to hold the largest possible Roman numeral
    if s.len() < 15 {
        return Err("Buffer too small to hold Roman numeral.");
    }

    // Modified: Use a single mutable slice and keep track of the current position
    let mut pos = 0;

    while n >= 1000 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'M';
        }
        pos += 1;
        n -= 1000;
    }

    if n >= 900 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'C';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'M';
        }
        pos += 2;
        n -= 900;
    }

    if n >= 500 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'D';
        }
        pos += 1;
        n -= 500;
    }

    if n >= 400 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'C';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'D';
        }
        pos += 2;
        n -= 400;
    }

    while n >= 100 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'C';
        }
        pos += 1;
        n -= 100;
    }

    if n >= 90 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'X';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'C';
        }
        pos += 2;
        n -= 90;
    }

    if n >= 50 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'L';
        }
        pos += 1;
        n -= 50;
    }

    if n >= 40 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'X';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'L';
        }
        pos += 2;
        n -= 40;
    }

    while n >= 10 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'X';
        }
        pos += 1;
        n -= 10;
    }

    if n >= 9 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'I';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'X';
        }
        pos += 2;
        n -= 9;
    }

    if n >= 5 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'V';
        }
        pos += 1;
        n -= 5;
    }

    if n >= 4 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'I';
        }
        if pos + 1 < s.len() { // Modified: Ensure pos + 1 is within bounds
            s[pos + 1] = b'V';
        }
        pos += 2;
        n -= 4;
    }

    while n >= 1 {
        if pos < s.len() { // Modified: Ensure pos is within bounds
            s[pos] = b'I';
        }
        pos += 1;
        n -= 1;
    }

    if pos < s.len() { // Modified: Ensure pos is within bounds before setting s[pos] to 0
        s[pos] = 0; // Ensure the buffer is null-terminated
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut buffer = [0u8; 15]; // Modified: Increased buffer size to 15 bytes to handle the largest Roman numeral

    for i in 1..4000 {
        if let Err(e) = roman(&mut buffer, i) {
            eprintln!("Error: {}", e);
            continue;
        }
        print!("{:4}: ", i);
        // Modified: Use CStr to handle null-terminated strings
        if let Ok(cstr) = CStr::from_bytes_until_nul(&buffer) {
            print!("{}", cstr.to_string_lossy());
        }
        println!();
    }
    Ok(())
}