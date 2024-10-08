use std::str::FromStr;

#[derive(Clone)]
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
        let mut ret: Option<Box<List>> = None;
        let mut parent = if let Some(p) = parent { p } else { List::new_list() };

        let mut chars = s.chars().peekable();
        let mut current_str = String::new();
        while let Some(c) = chars.next() {
            match c {
                ']' => {
                    if let Some(e_ref) = e {
                        *e_ref = chars.collect::<String>().as_str(); // 修改: 使用 chars.collect::<String>().as_str() 替代 chars.as_str()
                    }
                    return parent;
                }
                '[' => {
                    ret = Some(List::new_list());
                    if let Some(ret_ref) = ret.as_mut() {
                        ret_ref.is_list = true;
                        ret_ref.ival = 0;
                        parent.append(ret_ref.clone());
                        parent = List::from_string(&current_str, e, Some(ret_ref.clone())); // 修改: 使用 current_str 替代 chars.as_str()
                        current_str.clear();
                    }
                    continue;
                }
                '0'..='9' => {
                    ret = Some(List::new_list());
                    if let Some(ret_ref) = ret.as_mut() {
                        ret_ref.is_list = false;
                        let num_str: String = chars.clone().take_while(|&ch| ch.is_digit(10)).collect();
                        if num_str.is_empty() {
                            continue;
                        } else {
                            ret_ref.ival = i32::from_str(&num_str).unwrap_or(0); // 修改: 使用 unwrap_or 替代 expect
                            parent.append(ret_ref.clone());
                        }
                    }
                    continue;
                }
                ',' => {
                    continue;
                }
                _ => {
                    current_str.push(c);
                }
            }
        }

        if let Some(e_ref) = e {
            *e_ref = chars.collect::<String>().as_str(); // 修改: 使用 chars.collect::<String>().as_str() 替代 chars.as_str()
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
        let mut to = if let Some(t) = to { t } else { List::new_list() };

        if !self.is_list {
            to.append(Box::new(self.clone()));
        } else {
            for item in &self.lst {
                item.flatten(Some(to.clone()));
            }
        }
        to
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
}