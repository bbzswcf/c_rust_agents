use std::mem;

fn reverse_section(s: &mut [u8], length: usize) -> &mut [u8] {
    if length == 0 {
        return s;
    }

    // Modified: Corrected the loop condition to avoid off-by-one error
    for i in 0..length / 2 {
        // Modified: Corrected the index calculation to correctly reverse the section
        s.swap(i, length - i - 1);
    }
    s
}

fn reverse_words_in_order(s: &mut [u8], delim: u8) -> &mut [u8] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    // Modified: Corrected the loop condition to avoid skipping the last character
    while i < s.len() {
        let mut j = 0;
        // Modified: Removed unnecessary null terminator check
        while i + j < s.len() && s[i + j] != delim {
            j += 1;
        }
        // Modified: Corrected the length calculation to correctly reverse the section
        reverse_section(&mut s[i..i + j], j);
        i += j + 1; // Modified: Increment i to skip the delimiter
    }
    s
}

fn reverse_string(s: &mut [u8]) -> &mut [u8] {
    if !s.is_empty() {
        // Modified: Corrected the length calculation to correctly reverse the entire string
        reverse_section(s, s.len());
    }
    s
}

fn reverse_order_of_words(s: &mut [u8], delim: u8) -> &mut [u8] {
    reverse_string(s);
    reverse_words_in_order(s, delim);
    s
}

fn main() {
    // Modified: Removed unnecessary null terminator
    let str = b"rosetta code phrase reversal";
    let lenstr = str.len();
    let mut scopy = str.to_vec();
    let delim = b' ';

    // Original String
    // Modified: Corrected the string handling to avoid null terminator
    println!("Original:       \"{}\"", String::from_utf8_lossy(str));

    // Reversed string
    reverse_string(&mut scopy);
    println!("Reversed:       \"{}\"", String::from_utf8_lossy(&scopy));

    // Reversed words in string
    scopy.copy_from_slice(str);
    reverse_words_in_order(&mut scopy, delim);
    println!("Reversed words: \"{}\"", String::from_utf8_lossy(&scopy));

    // Reversed order of words in string
    scopy.copy_from_slice(str);
    reverse_order_of_words(&mut scopy, delim);
    println!("Reversed order: \"{}\"", String::from_utf8_lossy(&scopy));
}