use std::mem;
use std::ptr;

fn reverse_section(s: &mut [char], length: usize) -> &mut [char] {
    if length == 0 {
        return s;
    }

    // Modified: Changed loop condition to `for i in 0..length / 2` to avoid overflow
    for i in 0..length / 2 {
        s.swap(i, length - i - 1); // Modified: Subtract 1 to avoid overflow
    }
    s
}

fn reverse_words_in_order(s: &mut [char], delim: char) -> &mut [char] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    // Modified: Changed loop condition to `while i < s.len()` to ensure full iteration
    while i < s.len() {
        let mut j = 0;
        while i + j < s.len() && s[i + j] != '\0' && s[i + j] != delim {
            j += 1;
        }
        reverse_section(&mut s[i..i + j], j); // Modified: Removed `- 1` to avoid incorrect length calculation
        i += j + 1; // Modified: Increment by `j + 1` to skip the delimiter
    }
    s
}

fn reverse_string(s: &mut [char]) -> &mut [char] {
    if !s.is_empty() {
        reverse_section(s, s.len()); // Modified: Changed to `s.len()` to match function signature
    }
    s
}

fn reverse_order_of_words(s: &mut [char], delim: char) -> &mut [char] {
    reverse_string(s);
    reverse_words_in_order(s, delim);
    s
}

fn main() {
    let str = "rosetta code phrase reversal";
    let lenstr = str.len();
    let mut scopy = str.chars().collect::<Vec<char>>();
    let delim = ' ';

    // Original String
    println!("Original:       \"{}\"", str);

    // Reversed string
    let mut reversed = scopy.clone();
    reverse_string(&mut reversed);
    println!("Reversed:       \"{}\"", reversed.iter().collect::<String>());

    // Reversed words in string
    let mut reversed_words = scopy.clone();
    reverse_words_in_order(&mut reversed_words, delim);
    println!("Reversed words: \"{}\"", reversed_words.iter().collect::<String>());

    // Reversed order of words in string
    let mut reversed_order = scopy.clone();
    reverse_order_of_words(&mut reversed_order, delim);
    println!("Reversed order: \"{}\"", reversed_order.iter().collect::<String>());
}