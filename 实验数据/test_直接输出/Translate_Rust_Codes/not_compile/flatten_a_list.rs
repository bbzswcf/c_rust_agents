use std::str::FromStr;

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

    fn from_string(s: &str, e: &mut Option<&str>, parent: Option<&mut Box<List>>) -> Box<List> {
        let mut ret = None;
        let mut parent = if let Some(p) = parent {
            p.as_mut()
        } else {
            Some(Self::new_list())
        };

        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if let Some(e_ptr) = e {
                        *e_ptr = chars.as_str();
                    }
                    return parent.take().unwrap();
                }
                '[' => {
                    ret = Some(Self::new_list());
                    if let Some(p) = parent.as_mut() {
                        p.append(ret.take().unwrap());
                    }
                    let new_parent = parent.as_mut().unwrap().lst.last_mut().unwrap();
                    Self::from_string(chars.as_str(), &mut None, Some(new_parent));
                    continue;
                }
                '0'..='9' => {
                    ret = Some(Self::new_list());
                    if let Some(r) = ret.as_mut() {
                        r.is_list = false;
                        let mut num_str = c.to_string();
                        while let Some(&next_char) = chars.peek() {
                            if next_char.is_digit(10) {
                                num_str.push(chars.next().unwrap());
                            } else {
                                break;
                            }
                        }
                        r.ival = num_str.parse().unwrap();
                        if let Some(p) = parent.as_mut() {
                            p.append(ret.take().unwrap());
                        }
                    }
                    continue;
                }
                _ => {}
            }
        }

        if let Some(e_ptr) = e {
            *e_ptr = chars.as_str();
        }
        parent.take().unwrap()
    }

    fn show_list(&self) {
        if !self.is_list {
            print!("{}", self.ival);
            return;
        }

        print!("[");
        for (i, item) in self.lst.iter().enumerate() {
            item.show_list();
            if i < self.ival as usize - 1 {
                print!(", ");
            }
        }
        print!("]");
    }

    fn flatten(&self, to: Option<&mut Box<List>>) -> Box<List> {
        let mut to = if let Some(t) = to {
            t.as_mut()
        } else {
            Some(Self::new_list())
        };

        if !self.is_list {
            let t = Self::new_list();
            *t = *self;
            if let Some(to_list) = to.as_mut() {
                to_list.append(t);
            }
        } else {
            for item in &self.lst {
                item.flatten(to.as_mut());
            }
        }

        to.take().unwrap()
    }
}

fn main() {
    let input = "[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []";
    let mut end = None;
    let l = List::from_string(input, &mut end, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten(None);
    print!("Flattened: ");
    flat.show_list();
    println!();
}