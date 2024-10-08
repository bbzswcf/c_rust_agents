fn can_make_words(blocks: &mut [&str], word: &str) -> bool {
    let c = word.chars().next().map(|ch| ch.to_ascii_uppercase());

    if c.is_none() {
        return true;
    }

    let c = c.unwrap();

    if blocks.is_empty() {
        return false;
    }

    for i in 0..blocks.len() {
        if blocks[i].chars().any(|ch| ch.to_ascii_uppercase() == c) {
            blocks.swap(i, 0);
            let result = can_make_words(&mut blocks[1..], &word[1..]);
            blocks.swap(i, 0);
            if result {
                return true;
            }
        }
    }

    false
}

fn main() {
    let mut blocks = [
        "BO", "XK", "DQ", "CP", "NA", 
        "GT", "RE", "TG", "QD", "FS", 
        "JW", "HU", "VI", "AN", "OB", 
        "ER", "FS", "LY", "PC", "ZM",
    ];

    let words = [
        "", "A", "BARK", "BOOK", "TREAT", "COMMON", "SQUAD", "Confuse",
    ];

    for &word in words.iter() {
        println!("{}:\t{}", word, can_make_words(&mut blocks, word));
    }
}