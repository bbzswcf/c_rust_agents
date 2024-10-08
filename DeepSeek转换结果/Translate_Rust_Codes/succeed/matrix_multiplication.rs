struct Matrix {
    h: usize,
    w: usize,
    x: Vec<f64>,
}

fn dot(a: &[f64], b: &[f64], len: usize, step: usize) -> f64 {
    let mut r = 0.0;
    let mut a_iter = a.iter();
    let mut b_iter = b.iter().step_by(step);
    for _ in 0..len {
        r += a_iter.next().unwrap() * b_iter.next().unwrap();
    }
    r
}

fn mat_new(h: usize, w: usize) -> Matrix {
    Matrix {
        h,
        w,
        x: vec![0.0; h * w],
    }
}

fn mat_mul(a: &Matrix, b: &Matrix) -> Option<Matrix> {
    if a.w != b.h {
        return None;
    }

    let mut r = mat_new(a.h, b.w);
    let mut p = r.x.as_mut_slice();
    for i in 0..a.h {
        for j in 0..b.w {
            p[i * b.w + j] = dot(&a.x[i * a.w..(i + 1) * a.w], &b.x[j..], a.w, b.w);
        }
    }
    Some(r)
}

fn mat_show(a: &Matrix) {
    for i in 0..a.h {
        for j in 0..a.w {
            print!("\t{:7.3}", a.x[i * a.w + j]);
        }
        println!();
    }
    println!();
}

fn main() {
    let da = vec![1.0, 1.0, 1.0, 1.0,
                  2.0, 4.0, 8.0, 16.0,
                  3.0, 9.0, 27.0, 81.0,
                  4.0, 16.0, 64.0, 256.0];
    let db = vec![4.0, -3.0, 4.0 / 3.0,
                  -13.0 / 3.0, 19.0 / 4.0, -7.0 / 3.0,
                  3.0 / 2.0, -2.0, 7.0 / 6.0,
                  -1.0 / 6.0, 1.0 / 4.0, -1.0 / 6.0];

    let a = Matrix { h: 4, w: 4, x: da };
    let b = Matrix { h: 4, w: 3, x: db };
    let c = mat_mul(&a, &b).unwrap();

    mat_show(&c);
}