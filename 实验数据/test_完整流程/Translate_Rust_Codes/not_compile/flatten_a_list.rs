use std::str::FromStr;

#[derive(Clone)] // Added: Implement the `Clone` trait for `List`
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

    fn from_string(s: &str) -> (Box<List>, usize) {
        let mut root = List::new_list();
        let mut stack = vec![root.as_mut()];
        let mut i = 1; // Skip the initial '['

        while i < s.len() {
            match s.chars().nth(i).unwrap() {
                '[' => {
                    let new_list = List::new_list(); // Modified: Declare `new_list` as mutable
                    stack.last_mut().unwrap().append(new_list.clone());
                    stack.push(new_list.as_mut()); // Modified: Push a mutable reference to the `Box<List>`
                    i += 1;
                }
                ']' => {
                    stack.pop();
                    i += 1;
                }
                ',' => {
                    i += 1;
                }
                _ => {
                    let num_end = s[i..].find(|c: char| !c.is_digit(10)).unwrap_or(s.len() - i) + i;
                    let num = i32::from_str(&s[i..num_end]).unwrap();
                    let new_elem = Box::new(List {
                        is_list: false,
                        ival: num,
                        lst: Vec::new(),
                    });
                    stack.last_mut().unwrap().append(new_elem);
                    i = num_end;
                }
            }
        }

        (root, s.len())
    }

    fn show_list(&self) {
        if !self.is_list {
            print!("{}", self.ival);
            return;
        }

        print!("[");
        for (i, child) in self.lst.iter().enumerate() {
            child.show_list();
            if i < self.ival as usize - 1 {
                print!(", ");
            }
        }
        print!("]");
    }

    fn flatten(&self) -> Box<List> {
        let mut flat_list = List::new_list();
        self.flatten_recursive(&mut flat_list);
        flat_list
    }

    fn flatten_recursive(&self, flat_list: &mut Box<List>) {
        if !self.is_list {
            flat_list.append(Box::new(self.clone()));
        } else {
            for child in &self.lst {
                child.flatten_recursive(flat_list);
            }
        }
    }
}

fn main() {
    let (l, _) = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []");

    print!("Nested: ");
    l.show_list();
    println!();

    let flat = l.flatten();
    print!("Flattened: ");
    flat.show_list();
}