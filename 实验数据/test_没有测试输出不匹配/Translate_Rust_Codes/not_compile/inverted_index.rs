use std::io::Write;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// Added: Include the `once_cell` crate in `Cargo.toml`
// [dependencies]
// once_cell = "1.8.0"  # Use the appropriate version number

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static CHR_IDX: Lazy<Mutex<[u8; 256]>> = Lazy::new(|| Mutex::new([0; 256]));
static IDX_CHR: Lazy<Mutex<[u8; 256]>> = Lazy::new(|| Mutex::new([0; 256]));

// Added: Define the constant `FNAME`
const FNAME: usize = 0;

struct Trie {
    next: Vec<Option<Box<Trie>>>,
    eow: bool,
}

impl Trie {
    fn new() -> Box<Self> {
        Box::new(Trie {
            // Modified: Initialize the `next` vector with a smaller capacity
            next: Vec::with_capacity(256),
            eow: false,
        })
    }

    fn trav(&mut self, str: &str, no_create: bool) -> Option<&mut Self> {
        let mut root = self;
        let mut chars = str.chars();
        while let Some(c) = chars.next() {
            let idx = CHR_IDX.lock().expect("Failed to lock CHR_IDX")[c as usize] as usize;
            if idx == 0 {
                // Modified: Ensure the loop has a condition to terminate
                if no_create {
                    return None;
                }
                continue;
            }

            if root.next.len() <= idx {
                root.next.resize_with(idx + 1, || None);
            }

            if root.next[idx].is_none() {
                if no_create {
                    return None;
                }
                root.next[idx] = Some(Trie::new());
            }
            // Modified: Ensure that the `unwrap` call is safe
            if let Some(next_trie) = root.next[idx].as_mut() {
                root = next_trie;
            } else {
                return None;
            }
        }
        if !root.eow && no_create {
            return None;
        }
        Some(root)
    }

    fn all<F>(&self, path: &mut String, depth: usize, callback: &F) -> bool
    where
        F: Fn(&str) -> bool,
    {
        if self.eow && !callback(path) {
            return false;
        }

        // Modified: Ensure the loop iterates over the correct range
        for i in 0..CHR_LEGAL.len() {
            if let Some(next) = &self.next[i] {
                let c = IDX_CHR.lock().expect("Failed to lock IDX_CHR")[i] as char;
                path.push(c);
                if !next.all(path, depth + 1, callback) {
                    return false;
                }
                path.pop();
            }
        }
        true
    }
}

fn add_index(root: &mut Trie, word: &str, fname: &str) {
    if let Some(x) = root.trav(word, false) {
        x.eow = true;
        if x.next[FNAME].is_none() {
            x.next[FNAME] = Some(Trie::new());
        }
        // Modified: Ensure that the `expect` call is safe
        if let Some(fname_trie) = x.next[FNAME].as_mut() {
            if let Some(fname_trie_node) = fname_trie.trav(fname, false) {
                fname_trie_node.eow = true;
            } else {
                // Handle the case where traversal fails
            }
        }
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    std::io::stdout().flush().unwrap();
    true
}

const FILES: [&str; 3] = ["f1.txt", "source/f2.txt", "other_file"];
const TEXT: [[&str; 5]; 3] = [
    ["it", "is", "what", "it", "is"],
    ["what", "is", "it", "", ""],
    ["it", "is", "a", "banana", ""],
];

fn init_tables() -> Box<Trie> {
    let mut root = Trie::new();
    {
        let mut chr_idx = CHR_IDX.lock().expect("Failed to lock CHR_IDX");
        let mut idx_chr = IDX_CHR.lock().expect("Failed to lock IDX_CHR");
        for (i, &c) in CHR_LEGAL.as_bytes().iter().enumerate() {
            chr_idx[c as usize] = (i + 1) as u8;
            idx_chr[i + 1] = c;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            if TEXT[i][j].is_empty() {
                break;
            }
            add_index(&mut root, TEXT[i][j], FILES[i]);
        }
    }

    root
}

fn search_index(root: &mut Trie, word: &str) {
    print!("Search for \"{}\": ", word);
    if let Some(found) = root.trav(word, true) {
        if found.next[FNAME].is_some() {
            let mut path = String::new();
            // Modified: Ensure that the `unwrap` call is safe
            if let Some(fname_trie) = found.next[FNAME].as_ref() {
                fname_trie.all(&mut path, 0, &print_path);
            } else {
                println!("not found");
            }
            println!();
        } else {
            println!("not found");
        }
    } else {
        println!("not found");
    }
}

fn main() {
    let mut root = init_tables();

    search_index(&mut root, "what");
    search_index(&mut root, "is");
    search_index(&mut root, "banana");
    search_index(&mut root, "boo");
}