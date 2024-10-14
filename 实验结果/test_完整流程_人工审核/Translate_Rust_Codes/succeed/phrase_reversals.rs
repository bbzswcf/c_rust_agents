use std::mem;

fn reverse_section(s: &mut [char], length: usize) -> &mut [char] {
    if length == 0 {
        return s;
    }

    // Modified: Ensure the loop correctly iterates over the half of the length to reverse the section
    for i in 0..length / 2 {
        s.swap(i, length - i - 1);
    }
    s
}

fn reverse_words_in_order(s: &mut [char], delim: char) -> &mut [char] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    while i < s.len() {
        let mut j = 0;
        while i + j < s.len() && s[i + j] != delim {
            j += 1;
        }
        // Modified: Ensure the correct length of the word is passed to reverse_section
        if j > 0 {
            reverse_section(&mut s[i..i + j], j);
        }
        i += j + 1;
    }
    s
}

fn reverse_string(s: &mut [char]) -> &mut [char] {
    // Modified: Ensure the correct length of the string is passed to reverse_section
    if !s.is_empty() {
        reverse_section(s, s.len());
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
    let mut scopy: Vec<char> = str.chars().collect();
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