use std::alloc::{alloc, Layout};
use std::ptr::null_mut;
use std::slice;
use std::str;
use std::process::Command;
use std::io::{self, BufRead, Write};

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static mut CHR_IDX: [u8; 256] = [0; 256];
static mut IDX_CHR: [u8; 256] = [0; 256];

const FNAME: usize = 0;

struct Trie {
    next: Vec<*mut Trie>,
    eow: i32,
}

impl Trie {
    fn new() -> *mut Trie {
        unsafe {
            let layout = Layout::new::<Trie>();
            let ptr = alloc(layout) as *mut Trie;
            if ptr.is_null() {
                panic!("Memory allocation failed");
            }
            (*ptr).next = vec![null_mut(); CHR_LEGAL.len() + 1];
            (*ptr).eow = 0;
            ptr
        }
    }
}

fn trie_trav(root: *mut Trie, str: &str, no_create: i32) -> *mut Trie {
    let mut root = root;
    let mut str = str;
    unsafe {
        while !root.is_null() {
            let c = str.chars().next().unwrap_or('\0') as u8;
            if c == b'\0' {
                if (*root).eow == 0 && no_create != 0 {
                    return null_mut();
                }
                break;
            }
            let c = CHR_IDX[c as usize];
            if c == 0 {
                str = &str[1..];
                continue;
            }
            let c = c as usize;
            if (*root).next[c].is_null() {
                if no_create != 0 {
                    return null_mut();
                }
                (*root).next[c] = Trie::new();
            }
            root = (*root).next[c];
            str = &str[1..];
        }
    }
    root
}

fn trie_all(root: *mut Trie, path: &mut [u8], depth: usize, callback: fn(&str) -> bool) -> bool {
    unsafe {
        // Modified: Removed unnecessary `unwrap_or` call and directly checked the result of the callback function
        if (*root).eow != 0 && !callback(str::from_utf8_unchecked(&path[..depth])) {
            return false;
        }
        for i in 1..CHR_LEGAL.len() + 1 {
            if (*root).next[i].is_null() {
                continue;
            }
            path[depth] = IDX_CHR[i];
            path[depth + 1] = b'\0';
            if !trie_all((*root).next[i], path, depth + 1, callback) {
                return false;
            }
        }
    }
    true
}

fn add_index(root: *mut Trie, word: &str, fname: &str) {
    let x = trie_trav(root, word, 0);
    unsafe {
        (*x).eow = 1;
        if (*x).next[FNAME].is_null() {
            (*x).next[FNAME] = Trie::new();
        }
        let x = trie_trav((*x).next[FNAME], fname, 0);
        (*x).eow = 1;
    }
}

fn print_path(path: &str) -> bool {
    print!(" {}", path);
    io::stdout().flush().unwrap();
    true
}

const FILES: [&str; 3] = ["f1.txt", "source/f2.txt", "other_file"];
const TEXT: [[&str; 5]; 3] = [
    ["it", "is", "what", "it", "is"],
    ["what", "is", "it", "", ""],
    ["it", "is", "a", "banana", ""],
];

fn init_tables() -> *mut Trie {
    let root = Trie::new();
    unsafe {
        for (i, &ch) in CHR_LEGAL.as_bytes().iter().enumerate() {
            CHR_IDX[ch as usize] = (i + 1) as u8;
            IDX_CHR[i + 1] = ch;
        }

        for i in 0..3 {
            for j in 0..5 {
                if TEXT[i][j].is_empty() {
                    break;
                }
                add_index(root, TEXT[i][j], FILES[i]);
            }
        }
    }
    root
}

fn search_index(root: *mut Trie, word: &str) {
    print!("Search for \"{}\": ", word);
    let found = trie_trav(root, word, 1);
    unsafe {
        if found.is_null() {
            println!("not found");
        } else {
            let mut path = vec![0u8; 1024];
            trie_all((*found).next[FNAME], &mut path, 0, print_path);
            println!();
        }
    }
}

fn main() {
    let root = init_tables();
    search_index(root, "what");
    search_index(root, "is");
    search_index(root, "banana");
    search_index(root, "boo");
}