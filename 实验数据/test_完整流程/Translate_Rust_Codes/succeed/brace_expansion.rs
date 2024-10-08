use std::process;

const BUFFER_SIZE: usize = 128;

#[derive(Debug, PartialEq)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

#[derive(Debug)] // Modified: Added `#[derive(Debug)]` to implement the `Debug` trait for `Node`
struct Node {
    tag: Tag,
    data: NodeData,
    next: Option<Box<Node>>,
}

#[derive(Debug)]
enum NodeData {
    Str(String),
    Root(Option<Box<Node>>),
}

impl Node {
    fn allocate_node(tag: Tag) -> Box<Node> {
        Box::new(Node {
            tag,
            data: NodeData::Root(None),
            next: None,
        })
    }

    fn make_leaf(str: String) -> Box<Node> {
        let mut n = Node::allocate_node(Tag::NodeLeaf);
        n.data = NodeData::Str(str);
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
                NodeData::Str(_) => {}
                NodeData::Root(root) => Node::deallocate_node(root),
            }
        }
    }

    fn append(root: &mut Box<Node>, elem: Box<Node>) {
        if root.tag == Tag::NodeSeq || root.tag == Tag::NodeTree {
            if let NodeData::Root(ref mut root_node) = root.data {
                if root_node.is_none() {
                    *root_node = Some(elem);
                } else {
                    let mut it = root_node.as_mut().unwrap();
                    while it.next.is_some() {
                        it = it.next.as_mut().unwrap();
                    }
                    it.next = Some(elem);
                }
            }
        } else {
            eprint!("Cannot append to node with tag: {:?}\n", root.tag);
            process::exit(1);
        }
    }

    fn count(n: &Box<Node>) -> usize {
        match n.tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                let mut it = n.data.unwrap_root();
                while let Some(node) = it {
                    sum += Node::count(node);
                    it = node.next.as_ref();
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1;
                let mut it = n.data.unwrap_root();
                while let Some(node) = it {
                    prod *= Node::count(node);
                    it = node.next.as_ref();
                }
                prod
            }
        }
    }

    fn expand(n: &Box<Node>, mut pos: usize) { // Modified: Made `pos` argument mutable
        match n.tag {
            Tag::NodeLeaf => {
                if let NodeData::Str(ref str) = n.data {
                    print!("{}", str);
                }
            }
            Tag::NodeTree => {
                let mut it = n.data.unwrap_root();
                while let Some(node) = it {
                    let cnt = Node::count(node);
                    if pos < cnt {
                        Node::expand(node, pos);
                        break;
                    }
                    it = node.next.as_ref();
                    pos -= cnt;
                }
            }
            Tag::NodeSeq => {
                let prod = pos; // Modified: Removed `mut` keyword since `prod` does not need to be mutable
                let mut it = n.data.unwrap_root();
                while let Some(node) = it {
                    let cnt = Node::count(node);
                    let rem = prod % cnt;
                    Node::expand(node, rem);
                    it = node.next.as_ref();
                }
            }
        }
    }
}

impl NodeData {
    fn as_ref(&self) -> &NodeData {
        self
    }

    // Modified: Implemented `unwrap_root` method to return `Option<&Box<Node>>`
    fn unwrap_root(&self) -> Option<&Box<Node>> {
        match self {
            NodeData::Root(root) => root.as_ref(),
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
    let mut buffer = String::with_capacity(BUFFER_SIZE);

    while *pos < input.len() {
        let c = input.chars().nth(*pos).unwrap();
        *pos += 1;
        if c == '\\' {
            let next_c = input.chars().nth(*pos).unwrap();
            *pos += 1;
            buffer.push('\\');
            buffer.push(next_c);
        } else if c == '{' {
            let tree = parse_tree(input, pos);
            if !buffer.is_empty() {
                Node::append(&mut root, Node::make_leaf(buffer.clone()));
                buffer.clear();
            }
            Node::append(&mut root, tree);
        } else {
            buffer.push(c);
        }
    }

    if !buffer.is_empty() {
        Node::append(&mut root, Node::make_leaf(buffer.clone()));
        buffer.clear();
    }

    root
}

fn parse_tree(input: &str, pos: &mut usize) -> Box<Node> {
    let mut root = Node::make_tree();
    let mut buffer = String::with_capacity(BUFFER_SIZE);
    let mut depth = 0;
    let mut as_seq = false;

    while *pos < input.len() {
        let c = input.chars().nth(*pos).unwrap();
        *pos += 1;
        if c == '\\' {
            let next_c = input.chars().nth(*pos).unwrap();
            *pos += 1;
            buffer.push('\\');
            buffer.push(next_c);
        } else if c == '{' {
            buffer.push(c);
            as_seq = true;
            depth += 1;
        } else if c == '}' {
            if depth > 0 {
                buffer.push(c);
                depth -= 1;
            } else {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(&buffer, &mut new_pos);
                    Node::append(&mut root, seq);
                } else {
                    Node::append(&mut root, Node::make_leaf(buffer.clone()));
                }
                break;
            }
        } else if c == ',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(&buffer, &mut new_pos);
                    Node::append(&mut root, seq);
                    buffer.clear();
                    as_seq = false;
                } else {
                    Node::append(&mut root, Node::make_leaf(buffer.clone()));
                    buffer.clear();
                }
            } else {
                buffer.push(c);
            }
        } else {
            buffer.push(c);
        }
    }

    root
}

fn test(input: &str) {
    let mut pos = 0;
    let n = parse_seq(input, &mut pos);
    let cnt = Node::count(&n);
    let mut i = 0;

    println!("Pattern: {}", input);

    while i < cnt {
        Node::expand(&n, i);
        println!();
        i += 1;
    }
    println!();

    Node::deallocate_node(Some(n));
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}