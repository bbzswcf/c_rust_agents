fn reverse_section(s: &mut [char], length: usize) -> &mut [char] {
    if length == 0 {
        return s;
    }

    // Modified: Adjusted loop condition to avoid iterating beyond the midpoint
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
        while i + j < s.len() && s[i + j] != delim && s[i + j] != '\0' {
            j += 1;
        }
        // Modified: Ensure j is greater than 1 before performing subtraction
        if j > 1 {
            reverse_section(&mut s[i..i + j], j - 1);
        }
        i += j + 1;
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
    if !s.is_empty() {
        reverse_string(s);
        reverse_words_in_order(s, delim);
    }
    s
}

fn main() {
    let str = "rosetta code phrase reversal".chars().collect::<Vec<char>>();
    // Modified: Ensure str is not empty before using its length
    if !str.is_empty() {
        let lenstr = str.len();
        let mut scopy = str.clone();
        let delim = ' ';

        // Original String
        println!("Original:       \"{}\"", str.iter().collect::<String>());

        // Reversed string
        scopy.clone_from_slice(&str);
        reverse_string(&mut scopy);
        println!("Reversed:       \"{}\"", scopy.iter().collect::<String>());

        // Reversed words in string
        scopy.clone_from_slice(&str);
        reverse_words_in_order(&mut scopy, delim);
        println!("Reversed words: \"{}\"", scopy.iter().collect::<String>());

        // Reversed order of words in string
        scopy.clone_from_slice(&str);
        reverse_order_of_words(&mut scopy, delim);
        println!("Reversed order: \"{}\"", scopy.iter().collect::<String>());
    } else {
        println!("Input string is empty.");
    }
}