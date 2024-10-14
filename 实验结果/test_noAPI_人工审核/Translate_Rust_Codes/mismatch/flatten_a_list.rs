use std::str::FromStr;

#[derive(Clone)]
struct List {
    is_list: i32,
    ival: i32,
    lst: Option<Box<Vec<List>>>,
}

impl List {
    fn new_list() -> Box<Self> {
        Box::new(List {
            is_list: 1,
            ival: 0,
            lst: None,
        })
    }

    fn append(&mut self, child: Box<List>) {
        if let Some(ref mut lst) = self.lst {
            lst.push(*child);
        } else {
            self.lst = Some(Box::new(vec![*child]));
        }
        self.ival += 1;
    }

    fn from_string(s: &str) -> Box<List> {
        let mut chars = s.chars();
        let mut stack: Vec<Box<List>> = vec![List::new_list()];

        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    let mut current = stack.pop().unwrap();
                    if let Some(parent) = stack.last_mut() {
                        parent.append(current);
                    }
                }
                '[' => {
                    stack.push(List::new_list());
                }
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some(next_char) = chars.next() {
                        if next_char.is_digit(10) {
                            num_str.push(next_char);
                        } else {
                            break;
                        }
                    }
                    let mut num_list = List::new_list();
                    num_list.is_list = 0;
                    num_list.ival = i32::from_str(&num_str).unwrap();
                    if let Some(parent) = stack.last_mut() {
                        parent.append(num_list);
                    }
                }
                _ => continue,
            }
        }

        stack.pop().unwrap()
    }

    fn show_list(&self) {
        if self.is_list == 0 {
            print!("{}", self.ival);
        } else {
            print!("[");
            if let Some(ref lst) = self.lst {
                for (i, item) in lst.iter().enumerate() {
                    item.show_list();
                    if i < lst.len() - 1 {
                        print!(", ");
                    }
                }
            }
            print!("]");
        }
    }

    fn flatten(&self) -> Box<List> {
        let mut flat_list = List::new_list();

        fn flatten_recursive(item: &List, flat_list: &mut Box<List>) {
            if item.is_list == 0 {
                let mut num_list = List::new_list();
                num_list.is_list = 0;
                num_list.ival = item.ival;
                flat_list.append(num_list);
            } else if let Some(ref lst) = item.lst {
                for sub_item in lst.iter() {
                    flatten_recursive(sub_item, flat_list);
                }
            }
        }

        flatten_recursive(self, &mut flat_list);
        flat_list
    }

    fn delete_list(&mut self) {
        if self.is_list == 1 && self.lst.is_some() {
            if let Some(ref mut lst) = self.lst {
                for item in lst.iter_mut() {
                    item.delete_list();
                }
            }
        }
        self.lst = None;
    }
}

fn main() {
    let l = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []");

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten();
    print!("Flattened: ");
    flat.show_list();
    println!();

    // l.delete_list();
    // flat.delete_list();
}