use std::mem;

// Implement the Clone trait for the Edit struct
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
    // Initialize the table with Edit structs that implement the Clone trait
    let mut tbl: Vec<Vec<Edit>> = vec![vec![Edit { c1: '\0', c2: '\0', n: 0, next: None }; lb + 1]; la + 1];

    for i in (0..=la).rev() {
        let aa = &a[i..];
        for j in (0..=lb).rev() {
            let bb = &b[j..];
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            // Clone the Edit structs before borrowing `tbl[i][j]` as mutable
            let repl = if i + 1 <= la && j + 1 <= lb {
                tbl[i + 1][j + 1].clone()
            } else {
                Edit { c1: '\0', c2: '\0', n: 0, next: None }
            };
            let dela = if i + 1 <= la {
                tbl[i + 1][j].clone()
            } else {
                Edit { c1: '\0', c2: '\0', n: 0, next: None }
            };
            let delb = if j + 1 <= lb {
                tbl[i][j + 1].clone()
            } else {
                Edit { c1: '\0', c2: '\0', n: 0, next: None }
            };

            let e = &mut tbl[i][j];

            e.c1 = aa.chars().next().unwrap_or('\0');
            e.c2 = bb.chars().next().unwrap_or('\0');
            if aa.is_empty() {
                e.next = Some(Box::new(delb));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }
            if bb.is_empty() {
                e.next = Some(Box::new(dela));
                e.n = e.next.as_ref().unwrap().n + 1;
                continue;
            }

            e.next = Some(Box::new(repl));
            if e.c1 == e.c2 {
                e.n = e.next.as_ref().unwrap().n;
                continue;
            }

            if e.next.as_ref().unwrap().n > delb.n {
                e.next = Some(Box::new(delb));
                e.c1 = '\0';
            }
            if e.next.as_ref().unwrap().n > dela.n {
                e.next = Some(Box::new(dela));
                e.c1 = aa.chars().next().unwrap_or('\0');
                e.c2 = '\0';
            }
            e.n = e.next.as_ref().unwrap().n + 1;
        }
    }

    let p = &tbl[0][0];
    println!("{} -> {}: {} edits", a, b, p.n);

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
        p = next;
    }
    println!();
}

fn main() {
    leven("raisethysword", "rosettacode");
}