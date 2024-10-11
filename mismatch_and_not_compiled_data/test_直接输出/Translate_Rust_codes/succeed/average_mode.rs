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
    // Sort values
    x.sort_by(cmp_dbl);

    // Count occurrence of each value
    let mut j = 1;
    for i in 0..x.len() - 1 {
        if x[i] != x[i + 1] {
            j += 1;
        }
    }

    let mut vc = Vec::with_capacity(j);
    vc.push(VCount { v: x[0], c: 1 });

    // Generate list value-count pairs
    let mut k = 0;
    for i in 0..x.len() - 1 {
        if x[i] != x[i + 1] {
            k += 1;
            vc.push(VCount { v: x[i + 1], c: 1 });
        } else {
            vc[k].c += 1;
        }
    }

    // Sort that by count in descending order
    vc.sort_by(vc_cmp);

    vc
}

fn main() {
    let mut values = vec![1.0, 3.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 12.0, 12.0, 12.0, 12.0, 17.0];
    let vc = get_mode(&mut values);

    let n_modes = vc.iter().take_while(|&v| v.c == vc[0].c).count();

    println!("got {} modes:", n_modes);
    for i in 0..n_modes {
        println!("\tvalue = {}, count = {}", vc[i].v, vc[i].c);
    }
}