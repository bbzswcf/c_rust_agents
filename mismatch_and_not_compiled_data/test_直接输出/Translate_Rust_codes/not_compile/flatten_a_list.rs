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

    fn from_string(s: &str, e: &mut usize, parent: Option<Box<List>>) -> Box<List> {
        let mut parent = parent.unwrap_or_else(List::new_list);
        let mut s = s;

        while !s.is_empty() {
            if s.starts_with(']') {
                if e.is_some() {
                    *e = s.len();
                }
                return parent;
            }
            if s.starts_with('[') {
                let ret = List::new_list();
                parent.append(ret);
                let (_, rest) = s.split_at(1);
                s = rest;
                let (_, rest) = rest.split_at(List::from_string(rest, e, Some(parent)).len());
                s = rest;
                continue;
            }
            if s.chars().next().unwrap().is_digit(10) {
                let ret = List::new_list();
                ret.is_list = false;
                let num_end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
                ret.ival = i32::from_str(&s[..num_end]).unwrap();
                parent.append(ret);
                s = &s[num_end..];
                continue;
            }
            s = &s[1..];
        }

        if e.is_some() {
            *e = s.len();
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
            if i < self.ival as usize - 1 {
                print!(", ");
            }
        }
        print!("]");
    }

    fn flatten(&self, to: Option<Box<List>>) -> Box<List> {
        let mut to = to.unwrap_or_else(List::new_list);
        if !self.is_list {
            let t = List::new_list();
            *t = *self;
            to.append(t);
        } else {
            for item in &self.lst {
                item.flatten(Some(to.clone()));
            }
        }
        to
    }
}

fn main() {
    let s = "[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []";
    let mut e = 0;
    let l = List::from_string(s, &mut e, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten(None);
    print!("Flattened: ");
    flat.show_list();
}