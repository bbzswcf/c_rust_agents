fn can_make_words(b: &mut [&str], word: &str) -> bool {
    // Handle the case where the word is empty
    if word.is_empty() {
        return true;
    }

    // Convert the first character of the word to uppercase to ensure consistent case handling
    let c = word.chars().next().unwrap().to_ascii_uppercase();

    // Use a boolean array to track which blocks have been used
    let mut used = vec![false; b.len()];

    for i in 0..b.len() {
        // Check for the character in a case-insensitive manner within the block
        if b[i].to_ascii_uppercase().contains(c.to_ascii_uppercase()) {
            used[i] = true; // Mark the block as used
            let ret = if word.len() > 1 {
                can_make_words(&mut b[..], &word[1..])
            } else {
                true
            };
            used[i] = false; // Restore the block to its original state
            if ret {
                return true;
            }
        }
    }

    false
}

fn main() {
    let blocks = [
        "BO", "XK", "DQ", "CP", "NA", 
        "GT", "RE", "TG", "QD", "FS", 
        "JW", "HU", "VI", "AN", "OB", 
        "ER", "FS", "LY", "PC", "ZM",
    ];

    let words = [
        "", "A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "Confuse",
    ];

    for &word in words.iter() {
        let mut blocks_copy = blocks.to_vec();
        println!("{}:\t{}", word, can_make_words(&mut blocks_copy, word));
    }
}