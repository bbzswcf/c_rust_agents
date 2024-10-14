use std::mem;

#[derive(Clone)]
struct Edit {
    c1: char,
    c2: char,
    n: i32,
    next: Option<Box<Edit>>,
}

fn leven(a: &str, b: &str) {
    let la = a.len();
    let lb = b.len();
    let mut tbl: Vec<Vec<Edit>> = Vec::with_capacity(la + 1);
    tbl.push(vec![Edit { c1: '\0', c2: '\0', n: 0, next: None }; lb + 1]);

    for i in 1..=la {
        let mut row = Vec::with_capacity(lb + 1);
        row.push(Edit { c1: '\0', c2: '\0', n: 0, next: None });
        tbl.push(row);
    }

    for i in (0..=la).rev() {
        let aa = &a[i..];
        for j in (0..=lb).rev() {
            let bb = &b[j..];
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            let mut e = &mut tbl[i][j];
            let repl = &tbl[i + 1][j + 1];
            let dela = &tbl[i + 1][j];
            let delb = &tbl[i][j + 1];

            e.c1 = aa.chars().next().unwrap_or('\0');
            e.c2 = bb.chars().next().unwrap_or('\0');

            if aa.is_empty() {
                e.next = Some(Box::new(delb.clone()));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }
            if bb.is_empty() {
                e.next = Some(Box::new(dela.clone()));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }

            e.next = Some(Box::new(repl.clone()));
            if e.c1 == e.c2 {
                e.n = e.next.as_ref().unwrap().n;
                continue;
            }

            if e.next.as_ref().unwrap().n > delb.n {
                e.next = Some(Box::new(delb.clone()));
                e.c1 = '\0';
            }
            if e.next.as_ref().unwrap().n > dela.n {
                e.next = Some(Box::new(dela.clone()));
                e.c1 = e.c1;
                e.c2 = '\0';
            }
            e.n = e.next.as_ref().unwrap().n + 1;
        }
    }

    let mut p = &tbl[0][0];
    println!("{} -> {}: {} edits", a, b, p.n);

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

        p = next;
    }
    println!();
}

fn main() {
    leven("raisethysword", "rosettacode");
}