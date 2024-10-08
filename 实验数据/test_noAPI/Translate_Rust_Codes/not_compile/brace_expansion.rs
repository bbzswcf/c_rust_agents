extern crate libc; // Added: Declare the `libc` crate

use std::mem;
use std::ptr;
use std::slice;
use std::str;

const BUFFER_SIZE: usize = 128;

type Character = u8;
type CStringPtr = *mut Character; // Modified: Renamed type alias to avoid conflict with Rust's String type

#[derive(Debug, PartialEq)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

struct Node {
    tag: Tag,
    data: Data,
    next: *mut Node,
}

union Data {
    str: CStringPtr,
    root: *mut Node,
}

fn allocate_node(tag: Tag) -> *mut Node {
    let node = unsafe {
        let node = libc::malloc(mem::size_of::<Node>()) as *mut Node;
        if node.is_null() {
            eprintln!("Failed to allocate node for tag: {:?}", tag);
            std::process::exit(1);
        }
        (*node).tag = tag;
        (*node).next = ptr::null_mut();
        node
    };
    node
}

fn make_leaf(str: CStringPtr) -> *mut Node {
    let node = allocate_node(Tag::NodeLeaf);
    unsafe {
        (*node).data.str = str;
    }
    node
}

fn make_tree() -> *mut Node {
    let node = allocate_node(Tag::NodeTree);
    unsafe {
        (*node).data.root = ptr::null_mut();
    }
    node
}

fn make_seq() -> *mut Node {
    let node = allocate_node(Tag::NodeSeq); // Removed: Unnecessary mutability
    unsafe {
        (*node).data.root = ptr::null_mut();
    }
    node
}

fn deallocate_node(node: *mut Node) {
    if node.is_null() {
        return;
    }

    deallocate_node(unsafe { (*node).next });
    unsafe {
        (*node).next = ptr::null_mut();

        match (*node).tag {
            Tag::NodeLeaf => {
                libc::free((*node).data.str as *mut libc::c_void);
                (*node).data.str = ptr::null_mut();
            }
            Tag::NodeTree | Tag::NodeSeq => {
                deallocate_node((*node).data.root);
                (*node).data.root = ptr::null_mut();
            }
            Tag::NodeLeaf => {
                // Handle the NodeLeaf case here
                todo!("Implement handling for NodeLeaf");
            }
        }

        libc::free(node as *mut libc::c_void);
    }
}

fn append(root: *mut Node, elem: *mut Node) {
    if root.is_null() {
        eprintln!("Cannot append to uninitialized node.");
        std::process::exit(1);
    }
    if elem.is_null() {
        return;
    }

    unsafe {
        match (*root).tag {
            Tag::NodeSeq | Tag::NodeTree => {
                if (*root).data.root.is_null() {
                    (*root).data.root = elem;
                } else {
                    let mut it = (*root).data.root;
                    while !(*it).next.is_null() {
                        it = (*it).next;
                    }
                    (*it).next = elem;
                }
            }
        }
    }
}

fn count(node: *mut Node) -> usize {
    if node.is_null() {
        return 0;
    }

    unsafe {
        match (*node).tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                let mut it = (*node).data.root;
                while !it.is_null() {
                    sum += count(it);
                    it = (*it).next;
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1; // Modified: Changed to mutable
                let mut it = (*node).data.root;
                while !it.is_null() {
                    prod *= count(it);
                    it = (*it).next;
                }
                prod
            }
        }
    }
}

fn expand(node: *mut Node, pos: &mut usize) { // Modified: Changed function signature to accept `pos` as a mutable reference
    if node.is_null() {
        return;
    }

    unsafe {
        match (*node).tag {
            Tag::NodeLeaf => {
                let str_slice = slice::from_raw_parts((*node).data.str, BUFFER_SIZE);
                // Modified: Use str::from_utf8 with proper error handling
                match str::from_utf8(str_slice) {
                    Ok(str_str) => print!("{}", str_str),
                    Err(_) => {
                        eprintln!("Invalid UTF-8 sequence in NodeLeaf");
                        std::process::exit(1);
                    }
                }
            }
            Tag::NodeTree => {
                let mut it = (*node).data.root;
                loop {
                    let cnt = count(it);
                    if *pos < cnt {
                        expand(it, pos);
                        break;
                    }
                    *pos -= cnt; // Modified: Changed to mutable reference
                    it = (*it).next;
                }
            }
            Tag::NodeSeq => {
                let prod = *pos; // Removed: Unnecessary mutability
                let mut it = (*node).data.root;
                while !it.is_null() {
                    let cnt = count(it);
                    let mut rem = prod % cnt; // Modified: Changed to mutable
                    expand(it, &mut rem);
                    it = (*it).next;
                }
            }
        }
    }
}

fn allocate_string(src: CStringPtr) -> CStringPtr {
    let len = unsafe { libc::strlen(src as *const libc::c_char) };
    let out = unsafe {
        let out = libc::calloc(len + 1, mem::size_of::<Character>()) as CStringPtr;
        if out.is_null() {
            eprintln!("Failed to allocate a copy of the string.");
            std::process::exit(1);
        }
        libc::strcpy(out as *mut libc::c_char, src as *const libc::c_char);
        out
    };
    out
}

fn parse_seq(input: CStringPtr, pos: &mut usize) -> *mut Node {
    let root = make_seq(); // Removed: Unnecessary mutability

    let mut buffer: [Character; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut bufpos = 0;

    while unsafe { *input.offset(*pos as isize) } != 0 {
        let c = unsafe { *input.offset((*pos) as isize) };
        *pos += 1;
        if c == b'\\' {
            let next_c = unsafe { *input.offset((*pos) as isize) };
            *pos += 1;
            if next_c == 0 {
                break;
            }
            buffer[bufpos] = b'\\';
            bufpos += 1;
            buffer[bufpos] = next_c;
            bufpos += 1;
        } else if c == b'{' {
            let tree = parse_tree(input, pos);
            if bufpos > 0 {
                append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                bufpos = 0;
            }
            append(root, tree);
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
    }

    root
}

fn parse_tree(input: CStringPtr, pos: &mut usize) -> *mut Node {
    let root = make_tree(); // Removed: Unnecessary mutability

    let mut buffer: [Character; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut bufpos = 0;
    let mut depth = 0;
    let mut as_seq = false;

    while unsafe { *input.offset(*pos as isize) } != 0 {
        let c = unsafe { *input.offset((*pos) as isize) };
        *pos += 1;
        if c == b'\\' {
            let next_c = unsafe { *input.offset((*pos) as isize) };
            *pos += 1;
            if next_c == 0 {
                break;
            }
            buffer[bufpos] = b'\\';
            bufpos += 1;
            buffer[bufpos] = next_c;
            bufpos += 1;
        } else if c == b'{' {
            buffer[bufpos] = c;
            bufpos += 1;
            as_seq = true;
            depth += 1;
        } else if c == b'}' {
            if depth > 0 {
                depth -= 1;
                buffer[bufpos] = c;
                bufpos += 1;
            } else {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(buffer.as_mut_ptr(), &mut new_pos);
                    append(root, seq);
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(buffer.as_mut_ptr(), &mut new_pos);
                    append(root, seq);
                    bufpos = 0;
                    as_seq = false;
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_mut_ptr())));
                    bufpos = 0;
                }
            } else {
                buffer[bufpos] = c;
                bufpos += 1;
            }
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    root
}

fn test(input: &str) {
    let input_cstr = std::ffi::CString::new(input).unwrap();
    let input_ptr = input_cstr.as_ptr() as CStringPtr;
    let mut pos = 0;
    let node = parse_seq(input_ptr, &mut pos);
    let cnt = count(node);

    println!("Pattern: {}", input);

    for mut i in 0..cnt { // Modified: Declare `i` as mutable
        expand(node, &mut i);
        println!();
    }
    println!();

    deallocate_node(node);
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}