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
            lst: Some(Box::new(vec![])), // Modified: Initialize lst to Some(Box::new(vec![])) to ensure it is a valid list
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

    fn from_string<'a>(s: &'a str, e: &mut Option<&'a str>, parent: Option<&mut Box<List>>) -> Box<List> {
        let mut parent = if let Some(p) = parent {
            p.clone()
        } else {
            List::new_list()
        };

        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if let Some(e_ptr) = e {
                        *e_ptr = chars.as_str();
                    }
                    return parent;
                }
                '[' => {
                    let mut new_list = List::new_list(); // Modified: Declare new_list as mutable
                    parent.append(new_list.clone());
                    List::from_string(chars.as_str(), &mut None, Some(&mut new_list)); // Modified: Ensure new_list is correctly borrowed as mutable
                    continue;
                }
                '0'..='9' => {
                    let mut num_str = String::new();
                    num_str.push(c);
                    while let Some(next_char) = chars.next() {
                        if next_char.is_digit(10) {
                            num_str.push(next_char);
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = i32::from_str(&num_str) {
                        let mut new_list = List::new_list();
                        new_list.is_list = 0;
                        new_list.ival = value;
                        parent.append(new_list);
                    }
                    continue;
                }
                _ => {}
            }
        }

        if let Some(e_ptr) = e {
            *e_ptr = chars.as_str();
        }
        parent
    }

    fn show_list(&self) {
        if self.is_list == 0 {
            print!("{}", self.ival);
            return;
        }

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

    fn flatten(&self, to: Option<&mut Box<List>>) -> Box<List> {
        let mut to = if let Some(t) = to {
            t.clone()
        } else {
            List::new_list()
        };

        if self.is_list == 0 {
            let mut t = List::new_list();
            t.is_list = 0;
            t.ival = self.ival; // Modified: Ensure the ival field is copied correctly
            to.append(t);
        } else if let Some(ref lst) = self.lst {
            for item in lst.iter() {
                item.flatten(Some(&mut to));
            }
        }
        to
    }

    fn delete_list(&mut self) {
        if self.is_list == 1 && self.lst.is_some() {
            if let Some(ref mut lst) = self.lst {
                for item in lst.iter_mut() {
                    item.delete_list();
                }
            }
        }
        self.lst = None; // Modified: Ensure lst is correctly managed to avoid memory leaks
    }
}

fn main() {
    let l = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []", &mut None, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten(None); // Modified: Ensure flat is correctly initialized and managed
    print!("Flattened: ");
    flat.show_list();
    println!();

    // l.delete_list();
    // flat.delete_list();
}