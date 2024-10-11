use std::mem;

fn reverse_section(s: &mut [u8], length: usize) -> &mut [u8] {
    if length == 0 {
        return s;
    }

    for i in 0..length / 2 + 1 {
        // Ensure that length - i is within bounds
        if length >= i {
            s.swap(i, length - i);
        }
    }
    s
}

fn reverse_words_in_order(s: &mut [u8], delim: u8) -> &mut [u8] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    while i < s.len() - 1 {
        let mut j = 0;
        while i + j < s.len() && s[i + j] != delim && s[i + j] != 0 {
            j += 1;
        }
        // Ensure that i + j does not exceed the bounds of s
        if i + j <= s.len() {
            unsafe {
                reverse_section(&mut s[i..i + j], j - 1);
            }
        }
        i += j;
    }
    s
}

fn reverse_string(s: &mut [u8]) -> &mut [u8] {
    if !s.is_empty() {
        // Ensure that s.len() - 1 is within bounds
        if s.len() > 0 {
            unsafe {
                reverse_section(s, s.len() - 1);
            }
        }
    }
    s
}

fn reverse_order_of_words(s: &mut [u8], delim: u8) -> &mut [u8] {
    unsafe {
        reverse_string(s);
        reverse_words_in_order(s, delim);
    }
    s
}

fn main() {
    let str = "rosetta code phrase reversal";
    let lenstr = str.len();
    let mut scopy = str.to_string();
    let delim = ' ';

    // Original String
    println!("Original:       \"{}\"", str);

    // Reversed string
    let mut scopy_bytes = scopy.as_bytes_mut();
    unsafe {
        reverse_string(scopy_bytes);
    }
    println!("Reversed:       \"{}\"", String::from_utf8_lossy(scopy_bytes));

    // Reversed words in string
    scopy = str.to_string();
    let mut scopy_bytes = scopy.as_bytes_mut();
    unsafe {
        reverse_words_in_order(scopy_bytes, delim as u8);
    }
    println!("Reversed words: \"{}\"", String::from_utf8_lossy(scopy_bytes));

    // Reversed order of words in string
    scopy = str.to_string();
    let mut scopy_bytes = scopy.as_bytes_mut();
    unsafe {
        reverse_order_of_words(scopy_bytes, delim as u8);
    }
    println!("Reversed order: \"{}\"", String::from_utf8_lossy(scopy_bytes));
}