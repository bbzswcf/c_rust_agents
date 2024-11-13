#[derive(Clone)]
struct List {
    is_list: bool,
    ival: i32,
    lst: Vec<Box<List>>,
}

// ChangeLog:2 @ /tmp/tmpw4wtrcm1/temp.rs
// FixDescription: Fix mismatched types by dereferencing the '&mut Box<List>' to 'Box<List>'.
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
    fn from_string(s: &str, parent: Option<&mut Box<List>>) -> (Box<List>, usize) {
        let mut parent = parent.unwrap_or_else(|| Box::new(*List::new_list()));
        let mut s = s;
        // ChangeLog:2 @ /tmp/tmp_8dl9hl1/temp.rs
        // FixDescription: Fix mismatched types by mutably borrowing the Box in the unwrap_or_else closure.
        let mut ret: Option<Box<List>> = None;
        while !s.is_empty() {
            if s.starts_with("]") {
                return (parent.clone(), s.len() - 1);
            }
            if s.starts_with("[") {
                ret = Some(List::new_list());
                if let Some(ret) = ret {
                    ret.is_list = true;
                    ret.ival = 0;
                    parent.append(ret.clone());
                    let (_, len) = List::from_string(&s[1..], Some(&mut Box::new(*ret)));
                    s = &s[len + 1..];
                    continue;
                }
            }
            if s.chars().next().unwrap().is_digit(10) {
                ret = Some(List::new_list());
                if let Some(ref mut ret) = ret {
                    ret.is_list = false;
                    let num_end = s.find(|c: char| !c.is_digit(10)).unwrap_or(s.len());
                    ret.ival = i32::from_str(&s[..num_end]).unwrap();
                    parent.append(ret.clone());
                    s = &s[num_end..];
                    continue;
                }
            }
            s = &s[1..];
        }
        (*parent, s.len())
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
    fn flatten(&self, to: Option<&mut Box<List>>) -> Box<List> {
        let mut to = to.unwrap_or_else(|| Box::new(*List::new_list()));
        // ChangeLog:2 @ /tmp/tmplccu09ud/temp.rs
        // FixDescription: Fix the error of moving out of a mutable reference by cloning the value if the performance cost is acceptable.
        if !self.is_list {
            to.append(Box::new(self.clone()));
        } else {
            for item in &self.lst {
                item.flatten(Some(&mut to));
            }
        }
        to.clone()
        // ChangeLog:3 @ /tmp/tmplccu09ud/temp.rs
        // FixDescription: Fix the error of borrowing `ret` as mutable by removing `&mut` from the function call.
    }
}

fn main() {
    let (l, _) = List::from_string("[1], 2, [[3,4], 5], [[[]]], [[[6]]], 7, 8, []", None);
    print!("Nested: ");
    l.show_list();
    println!();
    let flat = l.flatten(None);
    print!("Flattened: ");
    flat.show_list();
}