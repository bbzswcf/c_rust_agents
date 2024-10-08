use std::process;

const BUFFER_SIZE: usize = 128;

type Character = u8;
type String = Vec<Character>;

#[derive(Debug)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

#[derive(Debug)]
enum Data {
    Str(String),
    Root(Box<Node>),
}

#[derive(Debug)]
struct Node {
    tag: Tag,
    data: Data,
    next: Option<Box<Node>>,
}

fn allocate_node(tag: Tag) -> Box<Node> {
    Box::new(Node {
        tag,
        data: Data::Root(Box::new(Node {
            tag: Tag::NodeLeaf,
            data: Data::Str(Vec::new()),
            next: None,
        })),
        next: None,
    })
}

fn make_leaf(str: String) -> Box<Node> {
    let mut n = allocate_node(Tag::NodeLeaf);
    n.data = Data::Str(str);
    n
}

fn make_tree() -> Box<Node> {
    let mut n = allocate_node(Tag::NodeTree);
    n.data = Data::Root(Box::new(Node {
        tag: Tag::NodeLeaf,
        data: Data::Str(Vec::new()),
        next: None,
    }));
    n
}

fn make_seq() -> Box<Node> {
    let mut n = allocate_node(Tag::NodeSeq);
    n.data = Data::Root(Box::new(Node {
        tag: Tag::NodeLeaf,
        data: Data::Str(Vec::new()),
        next: None,
    }));
    n
}

fn deallocate_node(n: Option<Box<Node>>) {
    if let Some(mut n) = n {
        deallocate_node(n.next.take());
        match n.data {
            Data::Str(_) => {}
            Data::Root(root) => deallocate_node(Some(root)),
        }
    }
}

fn append(root: &mut Box<Node>, elem: Box<Node>) {
    if root.tag == Tag::NodeSeq || root.tag == Tag::NodeTree {
        if let Data::Root(ref mut root_data) = root.data {
            if root_data.next.is_none() {
                root_data.next = Some(elem);
            } else {
                let mut it = root_data.next.as_mut().unwrap();
                while it.next.is_some() {
                    it = it.next.as_mut().unwrap();
                }
                it.next = Some(elem);
            }
        }
    } else {
        eprintln!("Cannot append to node with tag: {:?}", root.tag);
        process::exit(1);
    }
}

fn count(n: &Box<Node>) -> usize {
    match n.tag {
        Tag::NodeLeaf => 1,
        Tag::NodeTree => {
            let mut sum = 0;
            let mut it = n.data.as_ref().unwrap_root();
            while let Some(node) = it {
                sum += count(node);
                it = node.next.as_ref().map(|n| n.as_ref());
            }
            sum
        }
        Tag::NodeSeq => {
            let mut prod = 1;
            let mut it = n.data.as_ref().unwrap_root();
            while let Some(node) = it {
                prod *= count(node);
                it = node.next.as_ref().map(|n| n.as_ref());
            }
            prod
        }
    }
}

fn expand(n: &Box<Node>, pos: usize) {
    match n.tag {
        Tag::NodeLeaf => {
            if let Data::Str(ref str) = n.data {
                print!("{}", String::from_utf8_lossy(str));
            }
        }
        Tag::NodeTree => {
            let mut it = n.data.as_ref().unwrap_root();
            loop {
                let cnt = count(it.as_ref().unwrap());
                if pos < cnt {
                    expand(it.as_ref().unwrap(), pos);
                    break;
                }
                pos -= cnt;
                it = it.and_then(|node| node.next.as_ref().map(|n| n.as_ref()));
            }
        }
        Tag::NodeSeq => {
            let mut prod = pos;
            let mut it = n.data.as_ref().unwrap_root();
            while let Some(node) = it {
                let cnt = count(node);
                let rem = prod % cnt;
                expand(node, rem);
                it = node.next.as_ref().map(|n| n.as_ref());
            }
        }
    }
}

fn allocate_string(src: &[Character]) -> String {
    let len = src.len();
    let mut out = Vec::with_capacity(len + 1);
    out.extend_from_slice(src);
    out
}

fn parse_seq(input: &[Character], pos: &mut usize) -> Box<Node> {
    let mut root = make_seq();
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
            let tree = parse_tree(input, pos);
            if bufpos > 0 {
                append(&mut root, make_leaf(allocate_string(&buffer[..bufpos])));
                bufpos = 0;
            }
            append(&mut root, tree);
        } else {
            buffer[bufpos] = c;
            bufpos += 1;
        }
    }

    if bufpos > 0 {
        append(&mut root, make_leaf(allocate_string(&buffer[..bufpos])));
    }

    root
}

fn parse_tree(input: &[Character], pos: &mut usize) -> Box<Node> {
    let mut root = make_tree();
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
                buffer[bufpos] = c;
                bufpos += 1;
                depth -= 1;
            } else {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(&buffer[..bufpos], &mut new_pos);
                    append(&mut root, seq);
                } else {
                    append(&mut root, make_leaf(allocate_string(&buffer[..bufpos])));
                }
                break;
            }
        } else if c == b',' {
            if depth == 0 {
                if as_seq {
                    let mut new_pos = 0;
                    let seq = parse_seq(&buffer[..bufpos], &mut new_pos);
                    append(&mut root, seq);
                    bufpos = 0;
                    as_seq = false;
                } else {
                    append(&mut root, make_leaf(allocate_string(&buffer[..bufpos])));
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
    let n = parse_seq(input_bytes, &mut pos);
    let cnt = count(&n);
    println!("Pattern: {}", input);

    for i in 0..cnt {
        expand(&n, i);
        println!();
    }
    println!();

    deallocate_node(Some(n));
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}