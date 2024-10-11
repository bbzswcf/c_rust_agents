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

    while i < x.len() - 1 {
        if x[i] != x[i + 1] {
            j += 1;
        }
        i += 1;
    }

    vc.resize(j + 1, VCount { v: 0.0, c: 0 });
    vc[0].v = x[0];
    vc[0].c = 1;

    i = 0;
    j = 0;

    while i < x.len() - 1 {
        vc[j].c += 1;
        if x[i] != x[i + 1] {
            j += 1;
            vc[j].v = x[i + 1];
            vc[j].c = 0;
        }
        i += 1;
    }

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