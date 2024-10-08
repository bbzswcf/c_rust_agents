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
    fn new() -> Self {
        let size = CHR_LEGAL.len();
        let mut next = Vec::with_capacity(size); // Modified: Initialize `next` with `None` values using a loop
        for _ in 0..size {
            next.push(None);
        }
        Trie {
            next,
            eow: false,
        }
    }
}

fn trie_new() -> Box<Trie> {
    Box::new(Trie::new())
}

fn trie_trav<'a>(root: &'a mut Box<Trie>, str: &str, no_create: bool) -> Option<&'a mut Box<Trie>> {
    let mut current = root;
    let mut chars = str.chars();
    while let Some(c) = chars.next() {
        if c as usize >= unsafe { CHR_IDX.len() } { // Modified: Ensure `c` is within the valid range of indices for `CHR_IDX`
            continue;
        }
        let idx = unsafe { CHR_IDX[c as usize] } as usize;
        if idx == 0 {
            continue;
        }

        if current.next.len() <= idx {
            current.next.resize_with(idx + 1, || None);
        }

        if current.next[idx].is_none() {
            if no_create {
                return None;
            }
            current.next[idx] = Some(trie_new());
        }
        if let Some(next) = current.next[idx].as_mut() { // Modified: Handle the None case instead of using unwrap
            current = next;
        } else {
            return None;
        }
    }
    if !current.eow && no_create {
        return None;
    }
    Some(current)
}

fn trie_all(root: &Box<Trie>, path: &mut String, depth: usize, callback: fn(&str) -> bool) -> bool {
    if root.eow && !callback(path) {
        return false;
    }

    for i in 0..root.next.len() { // Modified: Ensure the loop bounds match the length of `root.next`
        if let Some(next) = &root.next[i] { // Modified: Ensure `root.next[i]` is properly initialized and of the expected type
            let c = unsafe { IDX_CHR[i] } as char;
            path.push(c);
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

        if x.next.len() <= FNAME {
            x.next.resize_with(FNAME + 1, || None);
        }

        if x.next[FNAME].is_none() {
            x.next[FNAME] = Some(trie_new());
        }
        if let Some(fname_trie) = trie_trav(x.next[FNAME].as_mut().unwrap(), fname, false) { // Modified: Handle the None case instead of using unwrap
            fname_trie.eow = true;
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
    let mut root = trie_new();
    for (i, c) in CHR_LEGAL.chars().enumerate() { // Modified: Removed `&` from the pattern
        unsafe {
            CHR_IDX[c as usize] = (i + 1) as u8;
            IDX_CHR[i + 1] = c as u8;
        }
    }

    for i in 0..3 {
        for j in 0..5 {
            if i < TEXT.len() && j < TEXT[i].len() && TEXT[i][j].is_empty() { // Modified: Ensure the indices `i` and `j` are within bounds
                break;
            }
            add_index(&mut root, TEXT[i][j], FILES[i]);
        }
    }

    root
}

fn search_index(root: &mut Box<Trie>, word: &str) {
    print!("Search for \"{}\": ", word);
    if let Some(found) = trie_trav(root, word, true) {
        if found.next.len() <= FNAME || found.next[FNAME].is_none() { // Modified: Ensure `found.next[FNAME]` is properly initialized and of the expected type
            println!("not found");
        } else {
            let mut path = String::new();
            if let Some(fname_trie) = found.next[FNAME].as_ref() { // Modified: Handle the None case instead of using unwrap
                trie_all(fname_trie, &mut path, 0, print_path);
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