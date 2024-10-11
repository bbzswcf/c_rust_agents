use std::process;
use std::str;

const BUFFER_SIZE: usize = 128;

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
    str: std::mem::ManuallyDrop<String>, // Wrapped String in ManuallyDrop
    root: std::mem::ManuallyDrop<Option<Box<Node>>>, // Wrapped Option<Box<Node>> in ManuallyDrop
}

impl Node {
    fn allocate_node(tag: Tag) -> Box<Node> {
        let node = Box::new(Node {
            tag,
            data: Data { root: std::mem::ManuallyDrop::new(None) }, // Used ManuallyDrop
            next: None,
        });
        node
    }

    fn make_leaf(str: String) -> Box<Node> {
        let mut node = Self::allocate_node(Tag::NodeLeaf);
        node.data.str = std::mem::ManuallyDrop::new(str); // Used ManuallyDrop
        node
    }

    fn make_tree() -> Box<Node> {
        Self::allocate_node(Tag::NodeTree)
    }

    fn make_seq() -> Box<Node> {
        Self::allocate_node(Tag::NodeSeq)
    }

    fn deallocate_node(node: Option<Box<Node>>) {
        if let Some(mut node) = node {
            Self::deallocate_node(node.next.take());
            match node.tag {
                Tag::NodeLeaf => unsafe { std::mem::ManuallyDrop::into_inner(node.data.str).clear() }, // Used ManuallyDrop
                Tag::NodeTree | Tag::NodeSeq => Self::deallocate_node(unsafe { std::mem::ManuallyDrop::into_inner(node.data.root).take() }), // Added unsafe block
            }
        }
    }

    fn append(&mut self, elem: Option<Box<Node>>) {
        if let Some(elem) = elem {
            match self.tag {
                Tag::NodeSeq | Tag::NodeTree => {
                    if unsafe { (*self.data.root).is_none() } { // Explicitly dereferenced ManuallyDrop
                        self.data.root = std::mem::ManuallyDrop::new(Some(elem)); // Used ManuallyDrop
                    } else {
                        let mut it = unsafe { (*self.data.root).as_mut().unwrap() }; // Explicitly dereferenced ManuallyDrop
                        while it.next.is_some() {
                            it = it.next.as_mut().unwrap();
                        }
                        it.next = Some(elem);
                    }
                }
                _ => {
                    eprintln!("Cannot append to node with tag: {:?}", self.tag);
                    process::exit(1);
                }
            }
        }
    }

    fn count(&self) -> usize {
        match self.tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                let mut it = unsafe { (*self.data.root).as_ref() }; // Explicitly dereferenced ManuallyDrop
                while let Some(node) = it {
                    sum += node.count();
                    it = node.next.as_ref();
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1;
                let mut it = unsafe { (*self.data.root).as_ref() }; // Explicitly dereferenced ManuallyDrop
                while let Some(node) = it {
                    prod *= node.count();
                    it = node.next.as_ref();
                }
                prod
            }
        }
    }

    fn expand(&self, pos: usize) {
        match self.tag {
            Tag::NodeLeaf => print!("{}", unsafe { &*self.data.str }), // Added unsafe block
            Tag::NodeTree => {
                let mut it = unsafe { (*self.data.root).as_ref() }; // Explicitly dereferenced ManuallyDrop
                let mut pos = pos;
                while let Some(node) = it {
                    let cnt = node.count();
                    if pos < cnt {
                        node.expand(pos);
                        break;
                    }
                    pos -= cnt;
                    it = node.next.as_ref();
                }
            }
            Tag::NodeSeq => {
                let mut prod = pos;
                let mut it = unsafe { (*self.data.root).as_ref() }; // Explicitly dereferenced ManuallyDrop
                while let Some(node) = it {
                    let cnt = node.count();
                    let rem = prod % cnt;
                    node.expand(rem);
                    prod /= cnt;
                    it = node.next.as_ref();
                }
            }
        }
    }
}

fn allocate_string(src: &str) -> String {
    let len = src.len();
    let mut out = String::with_capacity(len);
    out.push_str(src);
    out
}

fn parse_seq(input: &str, pos: &mut usize) -> Option<Box<Node>> {
    let mut root = Node::make_seq();
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut bufpos = 0;

    while *pos < input.len() {
        let c = input.as_bytes()[*pos];
        *pos += 1;
        if c == b'\\' {
            if *pos < input.len() {
                buffer[bufpos] = input.as_bytes()[*pos];
                bufpos += 1;
                *pos += 1;
            }
        } else if c == b'{' {
            if bufpos > 0 {
                root.append(Some(Node::make_leaf(allocate_string(
                    str::from_utf8(&buffer[..bufpos]).unwrap(),
                ))));
                bufpos = 0;
            }
            root.append(parse_tree(input, pos));
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        root.append(Some(Node::make_leaf(allocate_string(
            str::from_utf8(&buffer[..bufpos]).unwrap(),
        ))));
    }

    Some(root)
}

fn parse_tree(input: &str, pos: &mut usize) -> Option<Box<Node>> {
    let mut root = Node::make_tree();
    let mut buffer = [0u8; BUFFER_SIZE];
    let mut bufpos = 0;
    let mut depth = 0;
    let mut as_seq = false;

    while *pos < input.len() {
        let c = input.as_bytes()[*pos];
        *pos += 1;
        if c == b'\\' {
            if *pos < input.len() {
                buffer[bufpos] = input.as_bytes()[*pos];
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
                buffer[bufpos] = c;
                bufpos += 1;
                depth -= 1;
            } else {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(str::from_utf8(&buffer[..bufpos]).unwrap(), &mut new_pos);
                    root.append(seq);
                } else {
                    root.append(Some(Node::make_leaf(allocate_string(
                        str::from_utf8(&buffer[..bufpos]).unwrap(),
                    ))));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(str::from_utf8(&buffer[..bufpos]).unwrap(), &mut new_pos);
                    root.append(seq);
                    bufpos = 0;
                    as_seq = false;
                } else {
                    root.append(Some(Node::make_leaf(allocate_string(
                        str::from_utf8(&buffer[..bufpos]).unwrap(),
                    ))));
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

    Some(root)
}

fn test(input: &str) {
    let mut pos = 0;
    let node = parse_seq(input, &mut pos).unwrap();
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