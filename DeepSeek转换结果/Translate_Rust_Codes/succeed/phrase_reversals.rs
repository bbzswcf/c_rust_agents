fn reverse_section(s: &mut [u8], length: usize) -> &mut [u8] {
    if length == 0 {
        return s;
    }

    for i in 0..length / 2 {
        s.swap(i, length - i - 1);
    }
    s
}

fn reverse_words_in_order(s: &mut [u8], delim: u8) -> &mut [u8] {
    if s.is_empty() {
        return s;
    }

    let mut i = 0;
    while i < s.len() {
        let mut j = 0;
        while i + j < s.len() && s[i + j] != delim {
            j += 1;
        }
        reverse_section(&mut s[i..i + j], j);
        i += j + 1;
    }
    s
}

fn reverse_string(s: &mut [u8]) -> &mut [u8] {
    if !s.is_empty() {
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
    let str = b"rosetta code phrase reversal";
    let lenstr = str.len();
    let mut scopy = vec![0; lenstr];
    let delim = b' ';

    // Original String
    print!("Original:       \"{}\"\n", String::from_utf8_lossy(str));

    // Reversed string
    scopy.copy_from_slice(str);
    reverse_string(&mut scopy);
    print!("Reversed:       \"{}\"\n", String::from_utf8_lossy(&scopy));

    // Reversed words in string
    scopy.copy_from_slice(str);
    reverse_words_in_order(&mut scopy, delim);
    print!("Reversed words: \"{}\"\n", String::from_utf8_lossy(&scopy));

    // Reversed order of words in string
    scopy.copy_from_slice(str);
    reverse_order_of_words(&mut scopy, delim);
    print!("Reversed order: \"{}\"\n", String::from_utf8_lossy(&scopy));
}