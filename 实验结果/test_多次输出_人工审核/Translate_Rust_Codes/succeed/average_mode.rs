use std::cmp::Ordering;

#[derive(Debug)]
struct VCount {
    v: f64,
    c: i32,
}

fn cmp_dbl(a: &f64, b: &f64) -> Ordering {
    a.partial_cmp(b).unwrap_or(Ordering::Equal)
}

fn vc_cmp(a: &VCount, b: &VCount) -> Ordering {
    b.c.cmp(&a.c)
}

fn get_mode(x: &mut [f64]) -> Vec<VCount> {
    // Sort values
    x.sort_by(cmp_dbl);

    // Count occurrences of each value
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

    // Sort by count in descending order
    vc.sort_by(vc_cmp);

    vc
}

fn main() {
    let mut values = vec![1.0, 3.0, 6.0, 6.0, 6.0, 6.0, 7.0, 7.0, 12.0, 12.0, 12.0, 12.0, 17.0];
    let vc = get_mode(&mut values);

    let n_modes = vc.iter().take_while(|&v| v.c == vc[0].c).count();

    println!("got {} modes:", n_modes);
    for v in vc.iter().take(n_modes) {
        println!("\tvalue = {}, count = {}", v.v, v.c);
    }
}