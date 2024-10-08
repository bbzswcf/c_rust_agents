use std::alloc::{alloc, Layout};
use std::process::Command;
use std::io::{self, BufRead};
use std::ptr::null_mut;
// 修改: 使用once_cell crate中的Lazy，需要在Cargo.toml中添加once_cell依赖
// 不需要使用extern crate once_cell;
use once_cell::sync::Lazy;

const CHR_LEGAL: &str = "abcdefghijklmnopqrstuvwxyz0123456789_-./";
static CHR_IDX: Lazy<[u8; 256]> = Lazy::new(|| {
    let mut chr_idx = [0; 256];
    for (i, &ch) in CHR_LEGAL.as_bytes().iter().enumerate() {
        chr_idx[ch as usize] = (i + 1) as u8;
    }
    chr_idx
});
static IDX_CHR: Lazy<[char; 256]> = Lazy::new(|| {
    let mut idx_chr = ['\0'; 256];
    for (i, ch) in CHR_LEGAL.chars().enumerate() { // 修改: 使用&ch会导致类型不匹配，直接使用ch
        idx_chr[i + 1] = ch;
    }
    idx_chr
});

const FNAME: usize = 1; // 修改: FNAME的定义位置

struct Trie {
    next: [Option<Box<Trie>>; CHR_LEGAL.len() + 1],
    eow: bool,
}

impl Trie {
    fn new() -> Box<Self> {
        // 修改: 使用std::array::from_fn初始化next
        Box::new(Trie {
            next: std::array::from_fn(|_| None),
            eow: false,
        })
    }
}

// 修改: 添加生命周期'a
fn trie_trav<'a>(root: &'a mut Box<Trie>, str: &'a str, no_create: bool) -> Option<&'a mut Box<Trie>> {
    let mut root = Some(root);
    let mut str = str;
    while let Some(node) = root {
        if str.is_empty() {
            if !node.eow && no_create {
                return None;
            }
            break;
        }
        if let Some(c) = str.chars().next() { // 修改: 使用if let解构Option
            // 修改: 将c转换为u8再转换为usize
            let c = c as u8 as usize;
            if CHR_IDX[c] == 0 {
                str = &str[1..];
                continue;
            }
            let idx = CHR_IDX[c] as usize;
            if node.next[idx].is_none() {
                if no_create {
                    return None;
                }
                node.next[idx] = Some(Trie::new());
            }
            root = node.next[idx].as_mut();
            str = &str[1..];
        } else {
            break;
        }
    }
    root
}

fn trie_all(root: &Box<Trie>, path: &mut String, depth: usize, callback: fn(&str) -> bool) -> bool {
    if root.eow && !callback(path) {
        return false;
    }
    // 修改: 使用for i in 1..=CHR_LEGAL.len()
    for i in 1..=CHR_LEGAL.len() {
        if let Some(next) = &root.next[i] {
            path.push(IDX_CHR[i]);
            if !trie_all(next, path, depth + 1, callback) {
                return false;
            }
            path.pop();
        }
    }
    true
}

fn add_index(root: &mut Box<Trie>, word: &str, fname: &str) {
    if let Some(x) = trie_trav(root, word, false) {
        x.eow = true;
        if x.next[FNAME].is_none() {
            x.next[FNAME] = Some(Trie::new());
        }
        if let Some(fname_trie) = x.next[FNAME].as_mut() {
            if let Some(node) = trie_trav(fname_trie, fname, false) { // 修改: 使用if let解构Option
                node.eow = true;
            }
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
    for (i, ch) in CHR_LEGAL.chars().enumerate() { // 修改: 使用&ch会导致类型不匹配，直接使用ch
        CHR_IDX[ch as usize] = (i + 1) as u8;
        IDX_CHR[i + 1] = ch;
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

fn search_index(root: &mut Box<Trie>, word: &str) {
    print!("Search for \"{}\": ", word);
    // 修改: 使用&mut Box<Trie>
    if let Some(found) = trie_trav(root, word, true) {
        if found.next[FNAME].is_none() {
            println!("not found");
        } else {
            let mut path = String::new();
            // 修改: 使用if let避免unwrap panic
            if let Some(next) = found.next[FNAME].as_ref() {
                trie_all(next, &mut path, 0, print_path);
            }
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