use std::collections::HashMap;
use std::ptr;

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
const FNAME: usize = 0;

struct Trie {
    next: Vec<*mut Trie>,
    eow: bool,
}

impl Trie {
    fn new() -> *mut Trie {
        let trie = Box::into_raw(Box::new(Trie {
            next: vec![ptr::null_mut(); CHR_LEGAL.len() + 1],
            eow: false,
        }));
        trie
    }

    fn find_word(root: *mut Trie, word: &str) -> *mut Trie {
        Trie::trie_trav(root, word, true)
    }

    fn trie_trav(root: *mut Trie, word: &str, no_create: bool) -> *mut Trie {
        let mut root = root;
        let mut word = word;
        while !root.is_null() {
            if word.is_empty() {
                if unsafe { !(*root).eow } && no_create {
                    return ptr::null_mut();
                }
                break;
            }

            let c = word.chars().next().unwrap() as usize;
            let idx = CHR_IDX.get(&c).copied().unwrap_or(0);
            if idx == 0 {
                word = &word[1..];
                continue;
            }

            if unsafe { (*root).next[idx].is_null() } {
                if no_create {
                    return ptr::null_mut();
                }
                unsafe { (*root).next[idx] = Trie::new() };
            }

            root = unsafe { (*root).next[idx] };
            word = &word[1..];
        }
        root
    }

    fn trie_all(root: *mut Trie, path: &mut String, depth: usize, callback: fn(&str) -> bool) -> bool {
        if unsafe { (*root).eow } && !callback(path) {
            return false;
        }

        for i in 1..CHR_LEGAL.len() + 1 {
            if unsafe { (*root).next[i].is_null() } {
                continue;
            }

            let c = IDX_CHR[i];
            path.push(c);
            if !Trie::trie_all(unsafe { (*root).next[i] }, path, depth + 1, callback) {
                return false;
            }
            path.pop();
        }
        true
    }

    fn add_index(root: *mut Trie, word: &str, fname: &str) {
        let x = Trie::trie_trav(root, word, false);
        unsafe { (*x).eow = true };

        if unsafe { (*x).next[FNAME].is_null() } {
            unsafe { (*x).next[FNAME] = Trie::new() };
        }
        let x = Trie::trie_trav(unsafe { (*x).next[FNAME] }, fname, false);
        unsafe { (*x).eow = true };
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    true
}

lazy_static::lazy_static! {
    static ref CHR_IDX: HashMap<usize, usize> = {
        let mut map = HashMap::new();
        for (i, &c) in CHR_LEGAL.as_bytes().iter().enumerate() {
            map.insert(c as usize, i + 1);
        }
        map
    };
    static ref IDX_CHR: Vec<char> = {
        let mut vec = vec!['\0'; CHR_LEGAL.len() + 1];
        for (i, &c) in CHR_LEGAL.chars().enumerate() {
            vec[i + 1] = c;
        }
        vec
    };
}

fn init_tables() -> *mut Trie {
    let root = Trie::new();

    const FILES: [&str; 3] = ["f1.txt", "source/f2.txt", "other_file"];
    const TEXT: [&[&str]; 3] = [
        &["it", "is", "what", "it", "is"],
        &["what", "is", "it"],
        &["it", "is", "a", "banana"],
    ];

    for (i, &file) in FILES.iter().enumerate() {
        for &word in TEXT[i].iter() {
            Trie::add_index(root, word, file);
        }
    }

    root
}

fn search_index(root: *mut Trie, word: &str) {
    print!("Search for \"{}\":", word);
    let found = Trie::find_word(root, word);

    if found.is_null() {
        println!(" not found");
    } else {
        let mut path = String::new();
        Trie::trie_all(unsafe { (*found).next[FNAME] }, &mut path, 0, print_path);
        println!();
    }
}

fn main() {
    let root = init_tables();

    search_index(root, "what");
    search_index(root, "is");
    search_index(root, "banana");
    search_index(root, "boo");

    // Clean up the memory manually since we used raw pointers
    unsafe {
        for i in 0..CHR_LEGAL.len() + 1 {
            if !(*root).next[i].is_null() {
                Box::from_raw((*root).next[i]);
            }
        }
        Box::from_raw(root);
    }
}