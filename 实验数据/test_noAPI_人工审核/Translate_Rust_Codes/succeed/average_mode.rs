use std::cmp::Ordering;

#[derive(Debug)]
struct VCount {
    v: f64,
    c: i32,
}

fn cmp_dbl(a: &f64, b: &f64) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn vc_cmp(a: &VCount, b: &VCount) -> Ordering {
    b.c.cmp(&a.c)
}

fn get_mode(x: &mut [f64]) -> Vec<VCount> {
    x.sort_by(cmp_dbl);

    let mut vc = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < x.len() {
        let mut count = 1;
        while i + 1 < x.len() && x[i] == x[i + 1] {
            count += 1;
            i += 1;
        }
        vc.push(VCount { v: x[i], c: count });
        i += 1;
        j += 1;
    }

    vc.sort_by(vc_cmp);

    let max_count = vc[0].c;
    vc.retain(|vc| vc.c == max_count);

    vc
}

fn main() {
    let mut values = vec![1.0, 3.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 12.0, 12.0, 12.0, 12.0, 17.0];
    let vc = get_mode(&mut values);

    println!("got {} modes:", vc.len());
    for vcount in vc {
        println!("\tvalue = {}, count = {}", vcount.v, vcount.c);
    }
}