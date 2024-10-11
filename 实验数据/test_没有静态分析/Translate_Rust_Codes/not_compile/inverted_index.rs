use std::alloc::{alloc, Layout};

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static mut CHR_IDX: [u8; 256] = [0; 256];
static mut IDX_CHR: [u8; 256] = [0; 256];

const FNAME: usize = 0;

#[derive(Debug)]
struct Trie {
    next: Vec<Option<Box<Trie>>>,
    eow: bool,
}

impl Trie {
    fn new() -> Box<Self> {
        let layout = Layout::new::<Trie>();
        unsafe {
            let ptr = alloc(layout) as *mut Trie;
            Box::from_raw(ptr)
        }
    }

    // Modified: Changed the method signature to return `Option<&Trie>` when `no_create` is `true`
    fn find_word(&mut self, word: &str) -> Option<&Trie> {
        self.trie_trav(word, true)
    }

    // Modified: Changed the method signature to return `Option<&Trie>` when `no_create` is `true`
    fn trie_trav(&mut self, word: &str, no_create: bool) -> Option<&Trie> {
        let mut root = self;
        let mut chars = word.chars();
        while let Some(c) = chars.next() {
            let idx = unsafe { CHR_IDX[c as usize] } as usize;
            if idx == 0 {
                continue;
            }

            if root.next[idx].is_none() {
                if no_create {
                    return None;
                }
                root.next[idx] = Some(Trie::new());
            }
            root = root.next[idx].as_mut().unwrap();
        }
        if !root.eow && no_create {
            return None;
        }
        Some(root)
    }

    fn trie_all<F>(&self, path: &mut String, depth: usize, callback: &F) -> bool
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
                if !next.trie_all(path, depth + 1, callback) {
                    return false;
                }
                path.pop();
            }
        }
        true
    }
}

fn add_index(root: &mut Trie, word: &str, fname: &str) {
    if let Some(x) = root.trie_trav(word, false) {
        x.eow = true;

        if x.next[FNAME].is_none() {
            x.next[FNAME] = Some(Trie::new());
        }
        if let Some(y) = x.next[FNAME].as_mut().unwrap().trie_trav(fname, false) {
            y.eow = true;
        }
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    true
}

const FILES: [&str; 3] = ["f1.txt", "source/f2.txt", "other_file"];
const TEXT: [&[&str]; 3] = [
    &["it", "is", "what", "it", "is"],
    &["what", "is", "it"],
    &["it", "is", "a", "banana"],
];

fn init_tables() -> Box<Trie> {
    let mut root = Trie::new();
    unsafe {
        for (i, &c) in CHR_LEGAL.as_bytes().iter().enumerate() {
            CHR_IDX[c as usize] = (i + 1) as u8;
            IDX_CHR[i + 1] = c;
        }
    }

    for (i, &file) in FILES.iter().enumerate() {
        for &word in TEXT[i] {
            add_index(&mut root, word, file);
        }
    }

    root
}

fn search_index(root: &mut Trie, word: &str) {
    print!("Search for \"{}\": ", word);
    if let Some(found) = root.find_word(word) {
        if let Some(next) = found.next[FNAME].as_ref() {
            let mut path = String::new();
            next.trie_all(&mut path, 0, &print_path);
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