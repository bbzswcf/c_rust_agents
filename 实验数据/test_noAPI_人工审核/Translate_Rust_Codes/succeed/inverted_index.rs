use std::collections::HashMap;
use std::ptr;

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
const FNAME: usize = 0;

struct Trie {
    next: Vec<*mut Trie>,
    eow: bool,
}

impl Trie {
    unsafe fn new() -> *mut Trie {
        // Marked as unsafe as per feedback
        let trie = Box::into_raw(Box::new(Trie {
            next: vec![ptr::null_mut(); CHR_LEGAL.len() + 1],
            eow: false,
        }));
        trie
    }

    unsafe fn trav(root: *mut Trie, str: &str, no_create: bool) -> *mut Trie {
        // Marked as unsafe as per feedback
        let mut root = root;
        let mut str = str;
        while !root.is_null() {
            let c = str.chars().next().unwrap_or('\0');
            if c == '\0' {
                if !(*root).eow && no_create {
                    return ptr::null_mut();
                }
                break;
            }
            let idx = CHR_LEGAL.find(c);
            if idx.is_none() {
                str = &str[1..];
                continue;
            }
            let idx = idx.unwrap() + 1;
            if (*root).next[idx].is_null() {
                if no_create {
                    return ptr::null_mut();
                }
                (*root).next[idx] = Trie::new();
            }
            root = (*root).next[idx];
            str = &str[1..];
        }
        root
    }

    unsafe fn all(root: *mut Trie, path: &mut String, depth: usize, callback: fn(&str) -> bool) -> bool {
        // Marked as unsafe as per feedback
        if (*root).eow && !callback(path) {
            return false;
        }
        for i in 1..CHR_LEGAL.len() + 1 {
            if (*root).next[i].is_null() {
                continue;
            }
            path.push(CHR_LEGAL.chars().nth(i - 1).unwrap());
            if !Trie::all((*root).next[i], path, depth + 1, callback) {
                return false;
            }
            path.pop();
        }
        true
    }
}

unsafe fn add_index(root: *mut Trie, word: &str, fname: &str) {
    // Marked as unsafe as per feedback
    let x = Trie::trav(root, word, false);
    (*x).eow = true;
    if (*x).next[FNAME].is_null() {
        (*x).next[FNAME] = Trie::new();
    }
    let x = Trie::trav((*x).next[FNAME], fname, false);
    (*x).eow = true;
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    true
}

fn init_tables() -> *mut Trie {
    unsafe {
        // Wrapped in unsafe block as per feedback
        let root = Trie::new();
        let mut chr_idx: HashMap<char, usize> = HashMap::new();
        let mut idx_chr: Vec<char> = vec!['\0'; CHR_LEGAL.len() + 1];

        for (i, &c) in CHR_LEGAL.as_bytes().iter().enumerate() {
            chr_idx.insert(c as char, i + 1);
            idx_chr[i + 1] = c as char;
        }

        let files = vec!["f1.txt", "source/f2.txt", "other_file"];
        let text = vec![
            vec!["it", "is", "what", "it", "is"],
            vec!["what", "is", "it"],
            vec!["it", "is", "a", "banana"],
        ];

        for i in 0..3 {
            for j in 0..5 {
                if text[i].len() <= j {
                    break;
                }
                add_index(root, text[i][j], files[i]);
            }
        }

        root
    }
}

unsafe fn search_index(root: *mut Trie, word: &str) {
    // Marked as unsafe as per feedback
    print!("Search for \"{}\": ", word);
    let found = Trie::trav(root, word, true);
    if found.is_null() {
        println!("not found");
    } else {
        let mut path = String::new();
        Trie::all((*found).next[FNAME], &mut path, 0, print_path);
        println!();
    }
}

fn main() {
    let root = init_tables();

    unsafe {
        // Wrapped in unsafe block as per feedback
        search_index(root, "what");
        search_index(root, "is");
        search_index(root, "banana");
        search_index(root, "boo");

        // Clean up memory
        Box::from_raw(root);
    }
}