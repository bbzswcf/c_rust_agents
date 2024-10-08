use std::mem;

#[derive(Clone)]
struct List {
    is_list: bool,
    ival: i32,
    lst: Option<Vec<Box<List>>>,
}

impl List {
    fn new_list() -> Box<List> {
        Box::new(List {
            is_list: true,
            ival: 0,
            lst: None,
        })
    }

    fn append(&mut self, child: Box<List>) {
        if let Some(ref mut lst) = self.lst {
            lst.push(child);
        } else {
            self.lst = Some(vec![child]);
        }
        self.ival += 1;
    }

    fn from_string(s: &str, e: &mut Option<&str>, parent: Option<Box<List>>) -> Box<List> {
        let mut parent = if let Some(p) = parent { p } else { List::new_list() };

        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if let Some(e) = e {
                        *e = chars.as_str();
                    }
                    return parent;
                }
                '[' => {
                    let ret = List::new_list();
                    parent.append(ret.clone());
                    List::from_string(chars.as_str(), &mut Some(chars.as_str()), Some(ret));
                }
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some(next_char) = chars.next() {
                        if next_char.is_digit(10) {
                            num_str.push(next_char);
                        } else {
                            chars = next_char.to_string().chars().chain(chars);
                            break;
                        }
                    }
                    let ret = List::new_list();
                    ret.is_list = false;
                    ret.ival = num_str.parse().unwrap();
                    parent.append(ret);
                }
                _ => {}
            }
        }

        if let Some(e) = e {
            *e = chars.as_str();
        }
        parent
    }

    fn show_list(&self) {
        if !self.is_list {
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

    fn flatten(&self, to: Option<Box<List>>) -> Box<List> {
        let mut to = if let Some(t) = to { t } else { List::new_list() };

        if !self.is_list {
            let t = List::new_list();
            *t = self.clone();
            to.append(t);
        } else if let Some(ref lst) = self.lst {
            for item in lst {
                item.flatten(Some(to.clone()));
            }
        }

        to
    }

    fn delete_list(&mut self) {
        if let Some(ref mut lst) = self.lst {
            for item in lst.iter_mut() {
                item.delete_list();
            }
            self.lst = None;
        }
    }
}

fn main() {
    let l = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []", &mut None, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten(None);
    print!("Flattened: ");
    flat.show_list();
    println!();

    // l.delete_list();
    // flat.delete_list();
}