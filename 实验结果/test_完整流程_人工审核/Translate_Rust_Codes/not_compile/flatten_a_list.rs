use std::str::FromStr;

#[derive(Clone)] // Added: Implement the Clone trait for the List struct
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

    fn from_string<'a>(s: &'a str, e: &mut Option<&'a str>, parent: Option<Box<List>>) -> Box<List> {
        let mut value = if let Some(p) = parent { p } else { List::new_list() };
        let mut s = s;

        while !s.is_empty() {
            if s.starts_with(']') {
                if let Some(e_ref) = e {
                    *e_ref = &s[1..];
                }
                return value;
            }
            if s.starts_with('[') {
                let mut ret = List::new_list();
                ret.is_list = true;
                ret.ival = 0;
                value.append(ret);
                let (_, rest) = s.split_at(1);
                // Modified: Correctly wrap value.clone() in Box::new and pass e directly
                List::from_string(rest, e, Some(Box::new(value.clone())));
                continue;
            }
            if s.chars().next().unwrap().is_digit(10) {
                let mut ret = List::new_list();
                ret.is_list = false;
                // Modified: Use match to handle the None case explicitly
                let num_end = match s.find(|c: char| !c.is_digit(10)) {
                    Some(index) => index,
                    None => s.len(),
                };
                // Modified: Use unwrap_or_else for more idiomatic error handling
                ret.ival = i32::from_str(&s[..num_end]).unwrap_or_else(|_| panic!("Invalid integer"));
                value.append(ret);
                s = &s[num_end..];
                continue;
            }
            s = &s[1..];
        }

        if let Some(e_ref) = e {
            *e_ref = s;
        }
        value
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

    fn flatten(from: &List, to: Option<Box<List>>) -> Box<List> {
        // Modified: Use if let for better readability
        let mut to = if let Some(t) = to { *t } else { List::new_list() };

        if !from.is_list {
            // Modified: Clone the from list to avoid moving out of a shared reference
            (*to).append(Box::new(from.clone()));
            return to;
        } else {
            let mut value = to;
            for item in &from.lst {
                value = List::flatten(item, Some(*value));
            }
            value
        }
    }
}

fn main() {
    let l = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []", &mut None, None);

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = List::flatten(&l, None);
    print!("Flattened: ");
    flat.show_list();
}