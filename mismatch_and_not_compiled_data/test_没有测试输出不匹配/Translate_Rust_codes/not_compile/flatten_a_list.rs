use std::rc::Rc;
use std::cell::RefCell;
use std::str::FromStr;

struct List {
    is_list: bool,
    ival: i32,
    lst: Vec<Rc<RefCell<List>>>,
}

impl List {
    fn new_list() -> Rc<RefCell<List>> {
        Rc::new(RefCell::new(List {
            is_list: true,
            ival: 0,
            lst: Vec::new(),
        }))
    }

    fn append(&mut self, child: Rc<RefCell<List>>) {
        self.lst.push(child);
        self.ival += 1;
    }

    fn from_string<'a>(s: &'a str, e: &mut Option<&'a str>, parent: Option<Rc<RefCell<List>>>) -> Rc<RefCell<List>> {
        // Modified: Declare `parent` as mutable and initialize it properly
        let mut parent = if let Some(p) = parent { p } else { List::new_list() };
        let mut s = s;

        while !s.is_empty() {
            if s.starts_with(']') {
                if let Some(e_ptr) = e {
                    *e_ptr = &s[1..];
                }
                return parent;
            }
            if s.starts_with('[') {
                // Modified: Declare `ret` as mutable
                let mut ret = List::new_list();
                ret.borrow_mut().is_list = true;
                ret.borrow_mut().ival = 0;
                parent.borrow_mut().append(ret.clone());
                let (_, rest) = s.split_at(1);
                List::from_string(rest, &mut Some(&rest), Some(parent.clone()));
                continue;
            }
            if s.chars().next().unwrap().is_digit(10) {
                // Modified: Declare `ret` as mutable
                let mut ret = List::new_list();
                ret.borrow_mut().is_list = false;
                let num_end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
                ret.borrow_mut().ival = i32::from_str(&s[..num_end]).unwrap();
                parent.borrow_mut().append(ret.clone());
                s = &s[num_end..];
                continue;
            }
            s = &s[1..];
        }

        if let Some(e_ptr) = e {
            *e_ptr = s;
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
            item.borrow().show_list();
            if i < self.ival as usize - 1 {
                print!(", ");
            }
        }
        print!("]");
    }

    fn flatten(from: &Rc<RefCell<List>>, to: Option<Rc<RefCell<List>>>) -> Rc<RefCell<List>> {
        // Modified: Declare `to` as mutable and initialize it properly
        let mut to = if let Some(t) = to { t } else { List::new_list() };
        if !from.borrow().is_list {
            // Modified: Declare `t` as mutable
            let mut t = List::new_list();
            // Modified: Copy necessary fields from `**from` to `t`
            t.borrow_mut().is_list = from.borrow().is_list;
            t.borrow_mut().ival = from.borrow().ival;
            to.borrow_mut().append(t.clone());
        } else {
            for item in &from.borrow().lst {
                List::flatten(item, Some(to.clone()));
            }
        }
        to
    }
}

fn main() {
    let l = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []", &mut None, None);

    print!("Nested: ");
    l.borrow().show_list();
    println!();

    let flat = List::flatten(&l, None);
    print!("Flattened: ");
    flat.borrow().show_list();
}