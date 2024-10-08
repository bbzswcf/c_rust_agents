use std::process::exit;

const BUFFER_SIZE: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tag {
    NodeLeaf,
    NodeTree,
    NodeSeq,
}

// 修改: 确保 `Node` 和 `NodeData` 实现 `Debug` 和 `PartialEq`
#[derive(Debug, PartialEq)]
struct Node {
    tag: Tag,
    data: NodeData,
    next: Option<Box<Node>>,
}

// 修改: 确保 `NodeData` 实现 `Debug` 和 `PartialEq`
#[derive(Debug, PartialEq)]
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

    fn make_leaf(str: &str) -> Box<Node> {
        let mut n = Node::allocate_node(Tag::NodeLeaf);
        // 修改: 将 &str 转换为 String 并存储在 NodeData::Str 中
        n.data = NodeData::Str(str.to_owned());
        n
    }

    fn make_tree() -> Box<Node> {
        Node::allocate_node(Tag::NodeTree)
    }

    fn make_seq() -> Box<Node> {
        Node::allocate_node(Tag::NodeSeq)
    }

    fn deallocate_node(mut n: Box<Node>) {
        if let Some(next) = n.next.take() {
            Node::deallocate_node(next);
        }

        match n.tag {
            Tag::NodeLeaf => {
                if let NodeData::Str(ref mut s) = n.data {
                    s.clear();
                }
            }
            Tag::NodeTree | Tag::NodeSeq => {
                if let NodeData::Root(ref mut root) = n.data {
                    // 修改: 确保正确处理 child_root
                    if let Some(child_root) = root.take() {
                        Node::deallocate_node(child_root);
                    }
                }
            }
        }
    }

    fn append(&mut self, elem: Box<Node>) {
        if let (Tag::NodeSeq | Tag::NodeTree, NodeData::Root(ref mut root)) = (self.tag, &mut self.data) {
            if let Some(root) = root.as_mut() {
                let mut it = root;
                while it.next.is_some() {
                    it = it.next.as_mut().unwrap();
                }
                it.next = Some(elem);
            } else {
                *root = Some(elem);
            }
        } else {
            // 修改: 使用 eprintln! 和 eprint! 输出错误信息
            eprintln!("Cannot append to node with tag: {:?}", self.tag);
            exit(1);
        }
    }

    fn count(&self) -> usize {
        match self.tag {
            Tag::NodeLeaf => 1,
            Tag::NodeTree => {
                let mut sum = 0;
                if let NodeData::Root(ref root) = self.data {
                    let mut it = root.as_ref().map(|n| n.as_ref());
                    while let Some(node) = it {
                        sum += node.count();
                        it = node.next.as_ref().map(|n| n.as_ref());
                    }
                }
                sum
            }
            Tag::NodeSeq => {
                let mut prod = 1;
                if let NodeData::Root(ref root) = self.data {
                    let mut it = root.as_ref().map(|n| n.as_ref());
                    while let Some(node) = it {
                        prod *= node.count();
                        it = node.next.as_ref().map(|n| n.as_ref());
                    }
                }
                prod
            }
        }
    }

    fn expand(&self, pos: usize) {
        match self.tag {
            Tag::NodeLeaf => {
                if let NodeData::Str(ref s) = self.data {
                    print!("{}", s);
                }
            }
            Tag::NodeTree => {
                if let NodeData::Root(ref root) = self.data {
                    let mut it = root.as_ref().map(|n| n.as_ref());
                    let mut pos = pos;
                    while let Some(node) = it {
                        let cnt = node.count();
                        if pos < cnt {
                            node.expand(pos);
                            break;
                        }
                        pos -= cnt;
                        it = node.next.as_ref().map(|n| n.as_ref());
                    }
                }
            }
            Tag::NodeSeq => {
                if let NodeData::Root(ref root) = self.data {
                    let mut it = root.as_ref().map(|n| n.as_ref());
                    let mut prod = pos;
                    while let Some(node) = it {
                        let cnt = node.count();
                        let rem = prod % cnt;
                        node.expand(rem);
                        prod /= cnt;
                        it = node.next.as_ref().map(|n| n.as_ref());
                    }
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

fn parse_seq(input: &str, pos: &mut usize) -> Box<Node> {
    let mut root = Node::make_seq();
    let mut buffer = String::with_capacity(BUFFER_SIZE);

    while *pos < input.len() {
        // 修改: 使用 if let Some(c) 来处理字符
        if let Some(c) = input.chars().nth(*pos) {
            *pos += 1;
            if c == '\\' {
                // 修改: 使用 if let Some(next_c) 来处理转义字符
                if let Some(next_c) = input.chars().nth(*pos) {
                    *pos += 1;
                    buffer.push('\\');
                    buffer.push(next_c);
                } else {
                    buffer.push('\\');
                }
            } else if c == '{' {
                let tree = parse_tree(input, pos);
                if !buffer.is_empty() {
                    root.append(Node::make_leaf(&buffer));
                    buffer.clear();
                }
                root.append(tree);
            } else {
                buffer.push(c);
            }
        } else {
            break;
        }
    }

    if !buffer.is_empty() {
        root.append(Node::make_leaf(&buffer));
    }

    root
}

fn parse_tree(input: &str, pos: &mut usize) -> Box<Node> {
    let mut root = Node::make_tree();
    let mut buffer = String::with_capacity(BUFFER_SIZE);
    let mut depth = 0;
    let mut as_seq = false;

    while *pos < input.len() {
        // 修改: 使用 if let Some(c) 来处理字符
        if let Some(c) = input.chars().nth(*pos) {
            *pos += 1;
            if c == '\\' {
                // 修改: 使用 if let Some(next_c) 来处理转义字符
                if let Some(next_c) = input.chars().nth(*pos) {
                    *pos += 1;
                    buffer.push('\\');
                    buffer.push(next_c);
                } else {
                    buffer.push('\\');
                }
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
                        root.append(seq);
                    } else {
                        root.append(Node::make_leaf(&buffer));
                    }
                    break;
                }
            } else if c == ',' {
                if depth == 0 {
                    if as_seq {
                        let mut new_pos = 0;
                        let seq = parse_seq(&buffer, &mut new_pos);
                        root.append(seq);
                        buffer.clear();
                        as_seq = false;
                    } else {
                        root.append(Node::make_leaf(&buffer));
                        buffer.clear();
                    }
                } else {
                    buffer.push(c);
                }
            } else {
                buffer.push(c);
            }
        } else {
            break;
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

    Node::deallocate_node(n);
}

fn main() {
    test("~/{Downloads,Pictures}/*.{jpg,gif,png}");
    test("It{{em,alic}iz,erat}e{d,}, please.");
    test("{,{,gotta have{ ,\\, again\\, }}more }cowbell!");
}