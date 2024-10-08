use std::cell::RefCell;

#[derive(Default, Clone)]
struct Edit {
    c1: char,
    c2: char,
    n: i32,
    next: Option<Box<Edit>>,
}

fn leven(a: &str, b: &str) {
    let la = a.len();
    let lb = b.len();
    //  Vec::with_capacity 
    let mut tbl: Vec<Vec<RefCell<Edit>>> = Vec::with_capacity(la + 1);
    for _ in 0..=la {
        // 修改: 使用 &RefCell::new(Edit::default()) 来避免 Clone 问题
        tbl.push(vec![RefCell::new(Edit::default()); lb + 1]);
    }

    for i in (0..=la).rev() {
        let aa = match a.get(i..) {
            Some(s) => s,
            None => "",
        };
        for j in (0..=lb).rev() {
            let bb = match b.get(j..) {
                Some(s) => s,
                None => "",
            };
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            let e = &mut *tbl[i][j].borrow_mut();
            //  i + 1  j + 1 
            let repl = if i + 1 <= la && j + 1 <= lb {
                &*tbl[i + 1][j + 1].borrow()
            } else {
                &Edit::default()
            };
            let dela = if i + 1 <= la {
                &*tbl[i + 1][j].borrow()
            } else {
                &Edit::default()
            };
            let delb = if j + 1 <= lb {
                &*tbl[i][j + 1].borrow()
            } else {
                &Edit::default()
            };

            e.c1 = match aa.chars().next() {
                Some(c) => c,
                None => '\0',
            };
            e.c2 = match bb.chars().next() {
                Some(c) => c,
                None => '\0',
            };
            if aa.is_empty() {
                // 修改: 使用 delb.clone() 来避免 Clone 问题
                e.next = Some(Box::new(delb.clone()));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }
            if bb.is_empty() {
                // 修改: 使用 dela.clone() 来避免 Clone 问题
                e.next = Some(Box::new(dela.clone()));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }

            // 修改: 使用 repl.clone() 来避免 Clone 问题
            e.next = Some(Box::new(repl.clone()));
            if e.c1 == e.c2 {
                e.n = e.next.as_ref().unwrap().n;
                continue;
            }

            if e.next.as_ref().unwrap().n > delb.n {
                // 修改: 使用 delb.clone() 来避免 Clone 问题
                e.next = Some(Box::new(delb.clone()));
                e.c1 = '\0';
            }
            if e.next.as_ref().unwrap().n > dela.n {
                // 修改: 使用 dela.clone() 来避免 Clone 问题
                e.next = Some(Box::new(dela.clone()));
                e.c1 = match aa.chars().next() {
                    Some(c) => c,
                    None => '\0',
                };
                e.c2 = '\0';
            }
            e.n = e.next.as_ref().unwrap().n + 1;
        }
    }

    let p = &*tbl[0][0].borrow();
    print!("{} -> {}: {} edits\n", a, b, p.n);

    let mut p = p;
    while let Some(next) = &p.next {
        if p.c1 == p.c2 {
            print!("{}", p.c1);
        } else {
            print!("(");
            if p.c1 != '\0' {
                print!("{}", p.c1);
            }
            print!(",");
            if p.c2 != '\0' {
                print!("{}", p.c2);
            }
            print!(")");
        }
        //  borrow 
        p = next.as_ref();
    }
    print!("\n");
}

fn main() {
    leven("raisethysword", "rosettacode");
}