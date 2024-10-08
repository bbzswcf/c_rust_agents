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
    let mut tbl: Vec<Vec<Edit>> = vec![vec![Edit { c1: '\0', c2: '\0', n: 0, next: None }; lb + 1]; la + 1];

    // Correctly initialize the first row and column of `tbl`
    for i in 0..=la {
        tbl[i][0] = Edit { c1: a.chars().nth(i).unwrap_or('\0'), c2: '\0', n: i as i32, next: None };
    }
    for j in 0..=lb {
        tbl[0][j] = Edit { c1: '\0', c2: b.chars().nth(j).unwrap_or('\0'), n: j as i32, next: None };
    }

    for i in 1..=la {
        for j in 1..=lb {
            let repl = &tbl[i - 1][j - 1];
            let dela = &tbl[i - 1][j];
            let delb = &tbl[i][j - 1];

            let repl_n = repl.n + if a.chars().nth(i - 1) == b.chars().nth(j - 1) { 0 } else { 1 };
            let dela_n = dela.n + 1;
            let delb_n = delb.n + 1;

            let (_, rest) = tbl.split_at_mut(i);
            let (_, rest) = rest[0].split_at_mut(j);
            let e = &mut rest[0];

            e.c1 = a.chars().nth(i - 1).unwrap_or('\0');
            e.c2 = b.chars().nth(j - 1).unwrap_or('\0');

            if repl_n <= dela_n && repl_n <= delb_n {
                e.next = Some(Box::new(repl.clone()));
                e.n = repl_n;
            } else if dela_n <= delb_n {
                e.next = Some(Box::new(dela.clone()));
                e.n = dela_n;
            } else {
                e.next = Some(Box::new(delb.clone()));
                e.n = delb_n;
            }
        }
    }

    let mut p = &tbl[la][lb];
    println!("{} -> {}: {} edits", a, b, p.n);

    // Collect edits in a vector and print them in the correct order
    let mut edits = Vec::new();
    while let Some(next) = &p.next {
        edits.push((p.c1, p.c2));
        p = next;
    }

    // Print the edits in the correct order
    for (c1, c2) in edits {
        if c1 == c2 {
            print!("{}", c1);
        } else {
            print!("(");
            if c1 != '\0' {
                print!("{}", c1);
            }
            print!(",");
            if c2 != '\0' {
                print!("{}", c2);
            }
            print!(")");
        }
    }
    println!();
}

fn main() {
    leven("raisethysword", "rosettacode");
}