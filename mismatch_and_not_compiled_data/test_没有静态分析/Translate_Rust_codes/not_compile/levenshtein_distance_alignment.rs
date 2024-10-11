#[derive(Clone)] // Added: Derive Clone trait for Edit struct
struct Edit {
    c1: char,
    c2: char,
    n: i32,
    next: Option<Box<Edit>>,
}

fn leven(a: &str, b: &str) {
    let la = a.len();
    let lb = b.len();
    let mut tbl: Vec<Vec<Edit>> = vec![vec![Edit { c1: '\0', c2: '\0', n: 0, next: None }; lb + 1]; la + 1];

    for i in (0..=la).rev() {
        let aa = &a[i..];
        for j in (0..=lb).rev() {
            let bb = &b[j..];
            if aa.is_empty() && bb.is_empty() {
                continue;
            }

            // Use split_at_mut to obtain non-overlapping mutable slices
            let (left, right) = tbl.split_at_mut(i + 1);
            let e = &mut left[i][j];

            // Ensure indices are within bounds before calling nth
            if i < la {
                e.c1 = a.chars().nth(i).unwrap_or('\0');
            }
            if j < lb {
                e.c2 = b.chars().nth(j).unwrap_or('\0');
            }

            if aa.is_empty() {
                if j + 1 <= lb {
                    let delb_clone = left[i][j + 1].clone(); // Modified: Clone delb before mutable borrow ends
                    let delb = &delb_clone;
                    e.next = Some(Box::new(delb.clone())); // Modified: Clone delb
                    e.n = e.next.as_ref().unwrap().n + 1;
                }
                continue;
            }
            if bb.is_empty() {
                if i + 1 <= la {
                    let dela = &right[0][j];
                    e.next = Some(Box::new(dela.clone())); // Modified: Clone dela
                    e.n = e.next.as_ref().unwrap().n + 1;
                }
                continue;
            }

            let repl = &right[0][j + 1];
            e.next = Some(Box::new(repl.clone())); // Modified: Clone repl
            if e.c1 == e.c2 {
                e.n = e.next.as_ref().unwrap().n;
                continue;
            }

            if j + 1 <= lb {
                let delb_clone = left[i][j + 1].clone(); // Modified: Clone delb before mutable borrow ends
                let delb = &delb_clone;
                if e.next.as_ref().unwrap().n > delb.n {
                    e.next = Some(Box::new(delb.clone())); // Modified: Clone delb
                    e.c1 = '\0';
                }
            }
            if i + 1 <= la {
                let dela = &right[0][j];
                if e.next.as_ref().unwrap().n > dela.n {
                    e.next = Some(Box::new(dela.clone())); // Modified: Clone dela
                    e.c1 = e.c1;
                    e.c2 = '\0';
                }
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