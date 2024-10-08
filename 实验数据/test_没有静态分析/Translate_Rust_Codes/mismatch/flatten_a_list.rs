#[derive(Clone)] // Added: Derive Clone trait for List struct
struct List {
    is_list: bool,
    ival: i32,
    lst: Vec<Box<List>>,
}

impl List {
    fn new_list() -> Box<List> {
        Box::new(List {
            is_list: true,
            ival: 0,
            lst: Vec::new(),
        })
    }

    fn append(&mut self, child: Box<List>) {
        self.lst.push(child);
        self.ival += 1;
    }

    fn from_string(s: &str, e: &mut Option<String>, parent: Option<Box<List>>) -> Box<List> {
        let mut parent = if let Some(p) = parent { p } else { List::new_list() };
        let mut chars = s.chars().peekable();
        let mut remaining_chars = String::new();

        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if let Some(e_ref) = e {
                        *e_ref = chars.clone().collect(); // Clone chars before calling collect
                    }
                    return parent;
                }
                '[' => {
                    if chars.peek() == Some(&']') {
                        chars.next(); // Consume the ']'
                        parent.append(List::new_list());
                    } else {
                        let nested_list = List::from_string(&remaining_chars, &mut None, Some(List::new_list()));
                        parent.append(nested_list);
                    }
                    remaining_chars = chars.clone().collect(); // Clone chars before calling collect
                }
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_digit(10) {
                            num_str.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let ival = num_str.parse::<i32>().unwrap_or(0); // Handle parsing error gracefully
                    let ret = Box::new(List {
                        is_list: false,
                        ival,
                        lst: Vec::new(),
                    });
                    parent.append(ret);
                }
                _ => {}
            }
            remaining_chars.push(c); // Update the String with the current character
        }

        if let Some(e_ref) = e {
            *e_ref = remaining_chars; // Store the entire String
        }
        parent
    }

    fn show_list(&self) {
        if !self.is_list {
            print!("{}", self.ival);
            return;
        }

        print!("[");
        for (i, item) in self.lst.iter().enumerate() {
            item.show_list();
            if i < self.lst.len() - 1 {
                print!(", ");
            }
        }
        print!("]");
    }

    fn flatten(from: &Box<List>, to: &mut Box<List>) {
        if !from.is_list {
            to.append(Box::new(List {
                is_list: false,
                ival: from.ival,
                lst: Vec::new(),
            }));
        } else {
            for item in &from.lst {
                if !item.lst.is_empty() || !item.is_list {
                    List::flatten(item, to);
                }
            }
        }
    }
}

fn main() {
    let input = "[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []";
    let l = List::from_string(input, &mut None, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let mut to = List::new_list();
    List::flatten(&l, &mut to);
    print!("Flattened: ");
    to.show_list();
    println!();
}