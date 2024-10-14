fn can_make_words(b: &mut Vec<&str>, word: &str) -> bool {
    // Modified: The condition should be `if word.is_empty() {` because if the word is empty, it should always return true regardless of the blocks list.
    if word.is_empty() {
        return true;
    }

    let c = word.chars().next().map(|ch| ch.to_ascii_uppercase());
    if c.is_none() {
        return true;
    }
    let c = c.unwrap();

    // Modified: The function should return `false` immediately if the word contains non-alphabetic characters.
    if !c.is_ascii_alphabetic() {
        return false; // Return false if the word contains non-alphabetic characters
    }

    // Modified: The comparison should be `if word.len() > b.len() {` because the number of unique blocks is not relevant; the function should check against the total number of blocks available.
    if word.len() > b.len() {
        return false; // Return false if the word length exceeds the number of blocks
    }

    for i in 0..b.len() {
        // Modified: Ensure that the comparison is case-insensitive
        if b[i].chars().any(|ch| ch.to_ascii_uppercase() == c) {
            let block = b.remove(i); // Remove the block from further consideration
            let ret = can_make_words(b, &word[1..]);
            b.insert(i, block); // Reinsert the block for backtracking
            if ret {
                return true;
            }
        }
    }

    // Modified: The function should return `false` if no block matches the first character of the word.
    false
}

fn main() {
    let blocks = vec![
        "BO", "XK", "DQ", "CP", "NA", 
        "GT", "RE", "TG", "QD", "FS", 
        "JW", "HU", "VI", "AN", "OB", 
        "ER", "FS", "LY", "PC", "ZM",
    ];

    let words = vec![
        "", "A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "Confuse",
    ];

    for word in words {
        println!("{}\t{}", word, can_make_words(&mut blocks.clone(), word));
    }
}