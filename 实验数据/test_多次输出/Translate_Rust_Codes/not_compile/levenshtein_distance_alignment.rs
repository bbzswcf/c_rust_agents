use std::mem;

#[derive(Debug)]
struct Edit {
    c1: char,
    c2: char,
    n: i32,
    next: Option<Box<Edit>>,
}

fn leven(a: &str, b: &str) {
    let la = a.len();
    let lb = b.len();
    let mut tbl: Vec<Option<Box<Edit>>> = vec![None; (la + 1) * (lb + 1)];

    for i in (0..=la).rev() {
        let aa = &a[i..];
        for j in (0..=lb).rev() {
            let bb = &b[j..];
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            let e = &mut tbl[i * (lb + 1) + j];
            let repl = &mut tbl[(i + 1) * (lb + 1) + j + 1];
            let dela = &mut tbl[(i + 1) * (lb + 1) + j];
            let delb = &mut tbl[i * (lb + 1) + j + 1];

            let c1 = aa.chars().next().unwrap_or('\0');
            let c2 = bb.chars().next().unwrap_or('\0');

            if let Some(e) = e {
                e.c1 = c1;
                e.c2 = c2;

                if aa.is_empty() {
                    e.next = delb.take();
                    e.n = e.next.as_ref().map_or(0, |n| n.n) + 1;
                    continue;
                }
                if bb.is_empty() {
                    e.next = dela.take();
                    e.n = e.next.as_ref().map_or(0, |n| n.n) + 1;
                    continue;
                }

                e.next = repl.take();
                if c1 == c2 {
                    e.n = e.next.as_ref().map_or(0, |n| n.n);
                    continue;
                }

                if e.next.as_ref().map_or(0, |n| n.n) > delb.as_ref().map_or(0, |n| n.n) {
                    e.next = delb.take();
                    e.c1 = '\0';
                }
                if e.next.as_ref().map_or(0, |n| n.n) > dela.as_ref().map_or(0, |n| n.n) {
                    e.next = dela.take();
                    e.c1 = c1;
                    e.c2 = '\0';
                }
                e.n = e.next.as_ref().map_or(0, |n| n.n) + 1;
            } else {
                *e = Some(Box::new(Edit {
                    c1,
                    c2,
                    n: 0,
                    next: None,
                }));
            }
        }
    }

    if let Some(p) = &tbl[0] {
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
}

fn main() {
    leven("raisethysword", "rosettacode");
}