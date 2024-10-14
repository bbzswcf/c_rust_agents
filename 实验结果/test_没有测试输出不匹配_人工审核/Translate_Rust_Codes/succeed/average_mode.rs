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
    while i < x.len() {
        let mut count = 1;
        while i + count < x.len() && x[i] == x[i + count] {
            count += 1;
        }
        vc.push(VCount { v: x[i], c: count as i32 });
        i += count;
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