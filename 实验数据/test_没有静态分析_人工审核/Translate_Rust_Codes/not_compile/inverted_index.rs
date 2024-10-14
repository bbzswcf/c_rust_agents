use std::io; // Import the `io` module from the standard library
use std::io::Write; // Import the `Write` trait to use `flush` method

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static mut CHR_IDX: [u8; 256] = [0; 256];
static mut IDX_CHR: [u8; 256] = [0; 256];

const FNAME: usize = 0;

struct Trie {
    next: Vec<Option<Box<Trie>>>,
    eow: bool,
}

impl Trie {
    fn new() -> Self {
        // Initialize the vector with `None` values for each character in `CHR_LEGAL`
        // Modified: Use a loop to initialize the vector instead of `vec![]` to avoid the need for `Clone`
        let mut next = Vec::with_capacity(CHR_LEGAL.len());
        for _ in 0..CHR_LEGAL.len() {
            next.push(None);
        }
        let eow = false;
        Trie { next, eow }
    }

    fn trav(&mut self, str: &str, no_create: bool) -> Option<&mut Trie> {
        let mut root = self;
        let mut chars = str.chars();
        while let Some(c) = chars.next() {
            let idx = unsafe { CHR_IDX[c as usize] } as usize;
            if idx == 0 {
                continue;
            }
            if root.next[idx].is_none() {
                if no_create {
                    return None;
                }
                root.next[idx] = Some(Box::new(Trie::new()));
            }
            root = root.next[idx].as_mut().unwrap();
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
        for i in 1..CHR_LEGAL.len() {
            if let Some(next) = &self.next[i] {
                let c = unsafe { IDX_CHR[i] } as char;
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
            x.next[FNAME] = Some(Box::new(Trie::new()));
        }
        if let Some(y) = x.next[FNAME].as_mut().unwrap().trav(fname, false) {
            y.eow = true;
        }
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    io::stdout().flush().unwrap(); // Use the imported `io` module
    true
}

const FILES: [&str; 3] = ["f1.txt", "source/f2.txt", "other_file"];
const TEXT: [[&str; 5]; 3] = [
    ["it", "is", "what", "it", "is"],
    ["what", "is", "it", "", ""],
    ["it", "is", "a", "banana", ""],
];

fn init_tables() -> Trie {
    let mut root = Trie::new();
    for (i, c) in CHR_LEGAL.chars().enumerate() {
        unsafe {
            CHR_IDX[c as usize] = (i + 1) as u8;
            IDX_CHR[i + 1] = c as u8;
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

    root // Return the `Trie` instance directly
}

fn search_index(root: &mut Trie, word: &str) {
    print!("Search for \"{}\": ", word);
    if let Some(found) = root.trav(word, true) {
        if found.next[FNAME].is_none() {
            println!("not found");
        } else {
            let mut path = String::new();
            found.next[FNAME].as_ref().unwrap().all(&mut path, 0, &print_path);
            println!();
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