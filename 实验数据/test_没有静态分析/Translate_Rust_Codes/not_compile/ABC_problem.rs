fn can_make_words(b: &mut [&str], word: &str) -> bool {
    // Check if the word is empty and return true immediately
    if word.is_empty() {
        return true;
    }

    let c = word.chars().next().unwrap(); // Safe due to previous check for empty word

    // Ensure the function checks if the remaining word length is greater than the number of remaining blocks
    if word.len() > b.len() {
        return false;
    }

    for i in 0..b.len() {
        // Convert both characters to lowercase for case-insensitive comparison
        if b[i].chars().any(|ch| ch.to_ascii_lowercase() == c.to_ascii_lowercase()) {
            // Use a temporary vector to handle the removal and insertion of blocks
            // Modified: Use `to_vec` to create a new vector that clones the contents of the slice
            let mut temp_blocks = b.to_vec();
            let block = temp_blocks.remove(i);
            let ret = can_make_words(&mut temp_blocks, &word[1..]);
            b.copy_from_slice(&temp_blocks);
            if ret {
                // Return true when a valid combination is found
                return true;
            }
        }
    }

    // Return false if no combination of blocks can form the word
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
        let mut blocks_copy = blocks.clone();
        println!("{}\t{}", word, can_make_words(&mut blocks_copy, word));
    }
}