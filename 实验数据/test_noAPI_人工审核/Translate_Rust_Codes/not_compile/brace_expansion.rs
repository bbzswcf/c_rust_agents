use std::mem;
use std::ptr;
use std::slice;
use std::str;

const BUFFER_SIZE: usize = 128;

type Character = u8;
type String = *mut Character;

#[derive(Debug, Copy, Clone)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

#[derive(Debug)]
struct Node {
    tag: Tag,
    data: Data,
    next: *mut Node,
}

#[derive(Copy, Clone)]
union Data {
    str: String,
    root: *mut Node,
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            match self {
                Data { str: s } => write!(f, "Data {{ str: {:?} }}", s),
                Data { root: r } => write!(f, "Data {{ root: {:?} }}", r),
            }
        }
    }
}

fn allocate_node(tag: Tag) -> *mut Node {
    let n = unsafe { std::alloc::alloc(std::alloc::Layout::new::<Node>()) as *mut Node };
    if n.is_null() {
        eprintln!("Failed to allocate node for tag: {:?}", tag);
        std::process::exit(1);
    }
    unsafe {
        (*n).tag = tag;
        (*n).next = ptr::null_mut();
    }
    n
}

fn make_leaf(str: String) -> *mut Node {
    let n = allocate_node(Tag::NodeLeaf);
    unsafe {
        (*n).data.str = str;
    }
    n
}

fn make_tree() -> *mut Node {
    let n = allocate_node(Tag::NodeTree);
    unsafe {
        (*n).data.root = ptr::null_mut();
    }
    n
}

fn make_seq() -> *mut Node {
    let n = allocate_node(Tag::NodeSeq);
    unsafe {
        (*n).data.root = ptr::null_mut();
    }
    n
}

fn deallocate_node(n: *mut Node) {
    if n.is_null() {
        return;
    }

    deallocate_node(unsafe { (*n).next });
    unsafe {
        (*n).next = ptr::null_mut();
    }

    match unsafe { (*n).tag } {
        Tag::NodeLeaf => {
            unsafe {
                // Modified: Corrected str::len to expect a &str reference
                let len = unsafe { str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, 0))))))))) + 1 };
                std::alloc::dealloc(
                    (*n).data.str as *mut u8,
                    std::alloc::Layout::from_size_align_unchecked(len, 1),
                );
                (*n).data.str = ptr::null_mut();
            }
        }
        Tag::NodeTree | Tag::NodeSeq => {
            deallocate_node(unsafe { (*n).data.root });
            unsafe {
                (*n).data.root = ptr::null_mut();
            }
        }
    }

    unsafe {
        std::alloc::dealloc(n as *mut u8, std::alloc::Layout::new::<Node>());
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

    match unsafe { (*root).tag } {
        Tag::NodeSeq | Tag::NodeTree => {
            if unsafe { (*root).data.root.is_null() } {
                unsafe {
                    (*root).data.root = elem;
                }
            } else {
                let mut it = unsafe { (*root).data.root };
                while !unsafe { (*it).next.is_null() } {
                    it = unsafe { (*it).next };
                }
                unsafe {
                    (*it).next = elem;
                }
            }
        }
        _ => {
            eprintln!("Cannot append to node with tag: {:?}", unsafe { (*root).tag });
            std::process::exit(1);
        }
    }
}

fn count(n: *mut Node) -> usize {
    if n.is_null() {
        return 0;
    }

    match unsafe { (*n).tag } {
        Tag::NodeLeaf => 1,
        Tag::NodeTree => {
            let mut sum = 0;
            let mut it = unsafe { (*n).data.root };
            while !it.is_null() {
                sum += count(it);
                it = unsafe { (*it).next };
            }
            sum
        }
        Tag::NodeSeq => {
            let mut prod = 1;
            let mut it = unsafe { (*n).data.root };
            while !it.is_null() {
                prod *= count(it);
                it = unsafe { (*it).next };
            }
            prod
        }
    }
}

fn expand(n: *mut Node, pos: usize) {
    if n.is_null() {
        return;
    }

    match unsafe { (*n).tag } {
        Tag::NodeLeaf => {
            // Modified: Corrected str::len to expect a &str reference
            let len = unsafe { str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, 0))))))))) };
            print!("{}", unsafe { str::from_utf8_unchecked(slice::from_raw_parts((*n).data.str as *const u8, len)) });
        }
        Tag::NodeTree => {
            let mut it = unsafe { (*n).data.root };
            loop {
                let cnt = count(it);
                if pos < cnt {
                    expand(it, pos);
                    break;
                }
                pos -= cnt;
                it = unsafe { (*it).next };
            }
        }
        Tag::NodeSeq => {
            let mut prod = pos;
            let mut it = unsafe { (*n).data.root };
            while !it.is_null() {
                let cnt = count(it);
                let rem = prod % cnt;
                expand(it, rem);
                it = unsafe { (*it).next };
            }
        }
    }
}

fn allocate_string(src: String) -> String {
    // Modified: Corrected str::len to expect a &str reference
    let len = unsafe { str::len(str::from_utf8_unchecked(slice::from_raw_parts(src as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts(src as *const u8, str::len(str::from_utf8_unchecked(slice::from_raw_parts(src as *const u8, 0))))))))) };
    let out = unsafe {
        std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(len + 1, 1)) as String
    };
    if out.is_null() {
        eprintln!("Failed to allocate a copy of the string.");
        std::process::exit(1);
    }
    unsafe {
        ptr::copy_nonoverlapping(src as *const u8, out as *mut u8, len);
        *out.add(len) = 0;
    }
    out
}

fn parse_seq(input: String, pos: &mut usize) -> *mut Node {
    if input.is_null() {
        eprintln!("Input pointer is null.");
        std::process::exit(1);
    }

    let root = make_seq();

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut bufpos = 0;

    while unsafe { *input.add(*pos) } != 0 {
        let c = unsafe { *input.add(*pos) };
        *pos += 1;
        if c == b'\\' {
            let next_c = unsafe { *input.add(*pos) };
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
                append(root, make_leaf(allocate_string(buffer.as_ptr() as String)));
                bufpos = 0;
            }
            append(root, tree);
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        append(root, make_leaf(allocate_string(buffer.as_ptr() as String)));
    }

    root
}

fn parse_tree(input: String, pos: &mut usize) -> *mut Node {
    if input.is_null() {
        eprintln!("Input pointer is null.");
        std::process::exit(1);
    }

    let root = make_tree();

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut bufpos = 0;
    let mut depth = 0;
    let mut as_seq = false;

    while unsafe { *input.add(*pos) } != 0 {
        let c = unsafe { *input.add(*pos) };
        *pos += 1;
        if c == b'\\' {
            let next_c = unsafe { *input.add(*pos) };
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
                buffer[bufpos] = c;
                bufpos += 1;
                depth -= 1;
            } else {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(buffer.as_ptr() as String, &mut new_pos);
                    append(root, seq);
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_ptr() as String)));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(buffer.as_ptr() as String, &mut new_pos);
                    append(root, seq);
                    bufpos = 0;
                    as_seq = false;
                } else {
                    append(root, make_leaf(allocate_string(buffer.as_ptr() as String)));
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
    // Modified: Ensure input is not null before converting to raw pointer
    if input.is_empty() {
        eprintln!("Input string is empty.");
        std::process::exit(1);
    }

    let input_ptr = input.as_ptr() as String;
    let mut pos = 0;
    let n = parse_seq(input_ptr, &mut pos);
    let cnt = count(n);

    println!("Pattern: {}", input);

    for i in 0..cnt {
        expand(n, i);
        println!();
    }
    println!();

    deallocate_node(n);
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}