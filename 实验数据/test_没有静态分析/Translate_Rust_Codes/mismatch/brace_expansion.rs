use std::process::exit;
use std::str::from_utf8_unchecked;

const BUFFER_SIZE: usize = 128;

type Character = u8;
type StringPtr = *mut Character;

#[derive(Debug, PartialEq)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

struct Node {
    tag: Tag,
    data: Data,
    next: Option<Box<Node>>,
}

union Data {
    str: StringPtr,
    root: *mut Node,
}

impl Node {
    fn allocate_node(tag: Tag) -> Box<Node> {
        let node = Box::new(Node {
            tag,
            data: Data { root: std::ptr::null_mut() },
            next: None,
        });
        node
    }

    fn make_leaf(str: StringPtr) -> Box<Node> {
        let mut node = Node::allocate_node(Tag::NodeLeaf);
        // Modified: Declare node as mutable before assigning to node.data.str
        node.data.str = str;
        node
    }

    fn make_tree() -> Box<Node> {
        Node::allocate_node(Tag::NodeTree)
    }

    fn make_seq() -> Box<Node> {
        Node::allocate_node(Tag::NodeSeq)
    }

    fn deallocate_node(node: Option<Box<Node>>) {
        if let Some(mut node) = node {
            Node::deallocate_node(node.next.take());
            match node.tag {
                Tag::NodeLeaf => unsafe {
                    if !node.data.str.is_null() {
                        std::mem::drop(Box::from_raw(node.data.str));
                        node.data.str = std::ptr::null_mut();
                    }
                },
                Tag::NodeTree | Tag::NodeSeq => unsafe {
                    if !node.data.root.is_null() {
                        Node::deallocate_node(Some(Box::from_raw(node.data.root)));
                        node.data.root = std::ptr::null_mut();
                    }
                },
            }
        }
    }

    fn append(&mut self, elem: Box<Node>) {
        if self.tag == Tag::NodeSeq || self.tag == Tag::NodeTree {
            if unsafe { self.data.root.is_null() } {
                // Modified: The assignment to self.data.root is safe if self is mutable
                self.data.root = Box::into_raw(elem);
            } else {
                let mut it = unsafe { &mut *self.data.root };
                while it.next.is_some() {
                    it = it.next.as_mut().unwrap();
                }
                it.next = Some(elem);
            }
        } else {
            eprintln!("Cannot append to node with tag: {:?}", self.tag);
            exit(1);
        }
    }

    fn count(&self) -> usize {
        match self.tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                let mut it = unsafe { self.data.root.as_ref() };
                while let Some(node) = it {
                    sum += node.count();
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1;
                let mut it = unsafe { self.data.root.as_ref() };
                while let Some(node) = it {
                    prod *= node.count();
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
                prod
            }
        }
    }

    fn expand(&self, mut pos: usize) { // Modified: Make `pos` argument mutable
        match self.tag {
            Tag::NodeLeaf => unsafe {
                // Modified: Calculate the correct length of the string data
                let len = {
                    let mut len = 0;
                    while *self.data.str.add(len) != 0 {
                        len += 1;
                    }
                    len
                };
                // Modified: Ensure the raw pointer is valid and points to a valid memory location
                if !self.data.str.is_null() && len > 0 {
                    print!("{} ", from_utf8_unchecked(std::slice::from_raw_parts(self.data.str, len)));
                } else {
                    eprintln!("Invalid string pointer or length.");
                }
            },
            Tag::NodeTree => {
                let mut it = unsafe { self.data.root.as_ref() };
                while let Some(node) = it {
                    let cnt = node.count();
                    if cnt == 0 {
                        break;
                    }
                    if pos < cnt {
                        node.expand(pos);
                        break;
                    }
                    it = node.next.as_ref().map(|n| n.as_ref());
                    if it.is_none() {
                        break;
                    }
                    pos -= cnt; // Modified: `pos` is now mutable
                }
            }
            Tag::NodeSeq => {
                let prod = pos; // Modified: `prod` does not need to be mutable
                let mut it = unsafe { self.data.root.as_ref() };
                while let Some(node) = it {
                    let cnt = node.count();
                    if cnt == 0 {
                        break;
                    }
                    let rem = prod % cnt;
                    node.expand(rem);
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
            }
        }
    }
}

fn allocate_string(src: &[Character]) -> StringPtr {
    let len = src.len();
    let out = unsafe {
        let ptr = std::alloc::alloc(std::alloc::Layout::array::<Character>(len + 1).unwrap())
            as *mut Character;
        if ptr.is_null() {
            eprintln!("Failed to allocate a copy of the string.");
            exit(1);
        }
        std::ptr::copy_nonoverlapping(src.as_ptr(), ptr, len);
        ptr.add(len).write(0);
        ptr
    };
    out // Modified: Directly return `out` as a raw pointer
}

fn parse_seq(input: &[Character], pos: &mut usize) -> Box<Node> {
    let mut root = Node::make_seq();
    let mut buffer = [0; BUFFER_SIZE];
    let mut bufpos = 0;

    while *pos < input.len() {
        let c = input[*pos];
        *pos += 1;
        if c == b'\\' {
            if *pos < input.len() {
                buffer[bufpos] = input[*pos];
                bufpos += 1;
                *pos += 1;
            }
        } else if c == b'{' {
            if bufpos > 0 {
                root.append(Node::make_leaf(allocate_string(&buffer[..bufpos])));
                bufpos = 0;
            }
            let tree = parse_tree(input, pos);
            root.append(tree);
        } else if c == b',' {
            if bufpos > 0 {
                root.append(Node::make_leaf(allocate_string(&buffer[..bufpos])));
                bufpos = 0;
            }
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        root.append(Node::make_leaf(allocate_string(&buffer[..bufpos])));
    }

    root
}

fn parse_tree(input: &[Character], pos: &mut usize) -> Box<Node> {
    let mut root = Node::make_tree();
    let mut buffer = [0; BUFFER_SIZE];
    let mut bufpos = 0;
    let mut depth = 0;
    let mut as_seq = false;

    while *pos < input.len() {
        let c = input[*pos];
        *pos += 1;
        if c == b'\\' {
            if *pos < input.len() {
                buffer[bufpos] = input[*pos];
                bufpos += 1;
                *pos += 1;
            }
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
                    let seq = parse_seq(&buffer[..bufpos], &mut new_pos);
                    root.append(seq);
                } else {
                    root.append(Node::make_leaf(allocate_string(&buffer[..bufpos])));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(&buffer[..bufpos], &mut new_pos);
                    root.append(seq);
                    bufpos = 0;
                    as_seq = false;
                } else {
                    root.append(Node::make_leaf(allocate_string(&buffer[..bufpos])));
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
    let input_bytes = input.as_bytes();
    let mut pos = 0;
    let node = parse_seq(input_bytes, &mut pos);
    let cnt = node.count();

    println!("Pattern: {}", input);

    for i in 0..cnt {
        node.expand(i);
        println!();
    }
    println!();

    Node::deallocate_node(Some(node));
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}