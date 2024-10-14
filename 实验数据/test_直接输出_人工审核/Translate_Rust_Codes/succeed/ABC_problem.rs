fn can_make_words(b: &mut [&str], word: &str) -> bool {
    let c = word.chars().next().map(|ch| ch.to_ascii_uppercase()).unwrap_or('\0');

    if c == '\0' {
        return true;
    }
    if b.is_empty() {
        return false;
    }

    for i in 0..b.len() {
        if b[i].chars().any(|ch| ch.to_ascii_uppercase() == c) {
            b.swap(i, 0);
            let ret = can_make_words(&mut b[1..], &word[1..]);
            b.swap(i, 0);
            if ret {
                return true;
            }
        }
    }

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