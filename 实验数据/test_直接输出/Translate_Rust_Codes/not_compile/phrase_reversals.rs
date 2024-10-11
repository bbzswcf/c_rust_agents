use std::mem;

fn reverse_section(s: &mut [char], length: usize) -> &mut [char] {
    if length == 0 {
        return s;
    }

    for i in 0..length / 2 + 1 {
        s.swap(i, length - i);
    }
    s
}

fn reverse_words_in_order(s: &mut [char], delim: char) -> &mut [char] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    while i < s.len() - 1 {
        let mut j = 0;
        while i + j < s.len() && s[i + j] != '\0' && s[i + j] != delim {
            j += 1;
        }
        reverse_section(&mut s[i..i + j], j - 1);
        i += j;
    }
    s
}

fn reverse_string(s: &mut [char]) -> &mut [char] {
    if !s.is_empty() {
        reverse_section(s, s.len() - 1);
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
    let mut reversed: Vec<char> = str.chars().collect();
    reverse_string(&mut reversed);
    println!("Reversed:       \"{}\"", reversed.iter().collect::<String>());

    // Reversed words in string
    let mut reversed_words: Vec<char> = str.chars().collect();
    reverse_words_in_order(&mut reversed_words, delim);
    println!("Reversed words: \"{}\"", reversed_words.iter().collect::<String>());

    // Reversed order of words in string
    let mut reversed_order: Vec<char> = str.chars().collect();
    reverse_order_of_words(&mut reversed_order, delim);
    println!("Reversed order: \"{}\"", reversed_order.iter().collect::<String>());
}