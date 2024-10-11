use std::alloc::{alloc, Layout};
use std::process::Command;
use std::io::{self, BufRead};

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static mut CHR_IDX: [u8; 256] = [0; 256];
static mut IDX_CHR: [u8; 256] = [0; 256];

const FNAME: usize = 0;

struct Trie {
    next: [Option<Box<Trie>>; CHR_LEGAL.len() + 1],
    eow: bool,
}

impl Trie {
    fn new() -> Box<Self> {
        unsafe {
            let layout = Layout::new::<Self>();
            let ptr = alloc(layout) as *mut Self;
            Box::from_raw(ptr)
        }
    }

    fn trav(&mut self, str: &str, no_create: bool) -> Option<&mut Self> {
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
                root.next[idx] = Some(Trie::new());
            }
            root = root.next[idx].as_mut().unwrap();
        }
        if !root.eow && no_create {
            return None;
        }
        Some(root)
    }

    fn all(&self, path: &mut String, depth: usize, callback: &dyn Fn(&str) -> bool) -> bool {
        if self.eow && !callback(path) {
            return false;
        }

        for i in 1..CHR_LEGAL.len() + 1 {
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
            x.next[FNAME] = Some(Trie::new());
        }
        if let Some(fname_trie) = x.next[FNAME].as_mut() {
            fname_trie.trav(fname, false).unwrap().eow = true;
        }
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
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
    for (i, &c) in CHR_LEGAL.chars().enumerate() {
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

    root
}

fn search_index(root: &Trie, word: &str) {
    print!("Search for \"{}\": ", word);
    if let Some(found) = root.trav(word, true) {
        if let Some(fname_trie) = &found.next[FNAME] {
            let mut path = String::new();
            fname_trie.all(&mut path, 0, &print_path);
            println!();
        } else {
            println!("not found");
        }
    } else {
        println!("not found");
    }
}

fn main() {
    let root = init_tables();

    search_index(&root, "what");
    search_index(&root, "is");
    search_index(&root, "banana");
    search_index(&root, "boo");
}