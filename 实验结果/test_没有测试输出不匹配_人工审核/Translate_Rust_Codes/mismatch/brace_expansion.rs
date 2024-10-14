use std::process;

const BUFFER_SIZE: usize = 128;

#[derive(Debug, PartialEq)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

#[derive(Debug)]
struct Node {
    tag: Tag,
    data: Data,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
enum Data {
    Str(String),
    Root(Option<Box<Node>>),
}

impl Node {
    fn allocate_node(tag: Tag) -> Box<Node> {
        Box::new(Node {
            tag,
            data: Data::Root(None),
            next: None,
        })
    }

    fn make_leaf(str: String) -> Box<Node> {
        let mut n = Node::allocate_node(Tag::NodeLeaf);
        n.data = Data::Str(str);
        n
    }

    fn make_tree() -> Box<Node> {
        Node::allocate_node(Tag::NodeTree)
    }

    fn make_seq() -> Box<Node> {
        Node::allocate_node(Tag::NodeSeq)
    }

    fn deallocate_node(n: Option<Box<Node>>) {
        if let Some(mut n) = n {
            Node::deallocate_node(n.next.take());
            match n.data {
                Data::Str(_) => {}
                Data::Root(root) => Node::deallocate_node(root),
            }
        }
    }

    fn append(&mut self, elem: Box<Node>) {
        if self.tag == Tag::NodeSeq || self.tag == Tag::NodeTree {
            if let Data::Root(ref mut root) = self.data {
                if root.is_none() {
                    *root = Some(elem);
                } else {
                    let mut it = root.as_mut().unwrap();
                    while it.next.is_some() {
                        it = it.next.as_mut().unwrap();
                    }
                    it.next = Some(elem);
                }
            }
        } else {
            eprint!("Cannot append to node with tag: {:?}\n", self.tag);
            process::exit(1);
        }
    }

    fn count(&self) -> usize {
        match self.tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                let mut it = self.data.unwrap_root();
                while let Some(node) = it {
                    sum += node.count();
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1;
                let mut it = self.data.unwrap_root();
                while let Some(node) = it {
                    prod *= node.count();
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
                prod
            }
        }
    }

    fn expand(&self, pos: usize) {
        match self.tag {
            Tag::NodeLeaf => {
                if let Data::Str(ref str) = self.data {
                    print!("{}", str);
                }
            }
            Tag::NodeTree => {
                let mut it = self.data.unwrap_root();
                while let Some(node) = it {
                    let cnt = node.count();
                    if pos < cnt {
                        node.expand(pos);
                        break;
                    }
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
            }
            Tag::NodeSeq => {
                let prod = pos; // Removed `mut` as it is not necessary
                let mut it = self.data.unwrap_root();
                while let Some(node) = it {
                    let cnt = node.count();
                    let rem = prod % cnt;
                    node.expand(rem);
                    it = node.next.as_ref().map(|n| n.as_ref());
                }
            }
        }
    }
}

impl Data {
    // Modified: Changed the return type to Option<&Node>
    fn unwrap_root(&self) -> Option<&Node> {
        match self {
            Data::Root(root) => root.as_ref().map(|b| b.as_ref()),
            _ => None,
        }
    }
}

fn allocate_string(src: &str) -> String {
    let len = src.len();
    let mut out = String::with_capacity(len + 1);
    out.push_str(src);
    out
}

fn parse_seq(input: &str, pos: &mut usize) -> Box<Node> {
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
                root.append(Node::make_leaf(allocate_string(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"))));
                bufpos = 0;
            }
            let tree = parse_tree(input, pos);
            root.append(tree);
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        root.append(Node::make_leaf(allocate_string(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"))));
    }

    root
}

fn parse_tree(input: &str, pos: &mut usize) -> Box<Node> {
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
                buffer[bufpos] = b'\\';
                bufpos += 1;
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
                    let seq = parse_seq(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"), &mut new_pos);
                    root.append(seq);
                } else {
                    root.append(Node::make_leaf(allocate_string(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"))));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"), &mut new_pos);
                    root.append(seq);
                    bufpos = 0;
                } else {
                    root.append(Node::make_leaf(allocate_string(std::str::from_utf8(&buffer[..bufpos]).expect("Invalid UTF-8 sequence"))));
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
    let mut pos = 0;
    let n = parse_seq(input, &mut pos);
    let cnt = n.count();

    println!("Pattern: {}", input);

    for i in 0..cnt {
        n.expand(i);
        println!();
    }
    println!();

    Node::deallocate_node(Some(n));
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}