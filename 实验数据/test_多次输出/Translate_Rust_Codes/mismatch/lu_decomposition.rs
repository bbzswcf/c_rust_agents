use std::fmt;

type Mat = Vec<Vec<f64>>;

fn mat_zero(n: usize) -> Mat {
    vec![vec![0.0; n]; n]
}

fn mat_new(n: usize) -> Mat {
    let mut x = mat_zero(n);
    for i in 0..n {
        for j in 0..n {
            x[i][j] = 0.0;
        }
    }
    x
}

fn mat_copy(s: &Mat) -> Mat {
    s.clone()
}

fn mat_show(x: &Mat, fmt: Option<&str>) {
    let fmt = fmt.unwrap_or("%8.4g");
    let n = x.len();
    for (i, row) in x.iter().enumerate() {
        print!("{}", if i == 0 { " [ " } else { "   " });
        for (j, &val) in row.iter().enumerate() {
            print!("{:>8.4}", val);
            if j < n - 1 {
                print!("  ");
            } else if i == n - 1 {
                println!(" ]");
            } else {
                println!();
            }
        }
    }
}

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let n = a.len();
    let mut c = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn mat_pivot(a: &Mat) -> Mat {
    let n = a.len();
    let mut p = mat_new(n);
    for i in 0..n {
        p[i][i] = 1.0;
    }
    for i in 0..n {
        let mut max_j = i;
        for j in i..n {
            if a[j][i].abs() > a[max_j][i].abs() {
                max_j = j;
            }
        }
        if max_j != i {
            for k in 0..n {
                p.swap(i, max_j);
            }
        }
    }
    p
}

fn mat_lu(a: &Mat) -> (Mat, Mat, Mat) {
    let n = a.len();
    let mut l = mat_zero(n);
    let mut u = mat_zero(n);
    let p = mat_pivot(a);
    let aprime = mat_mul(&p, a);

    for i in 0..n {
        l[i][i] = 1.0;
    }
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            if j <= i {
                for k in 0..j {
                    s += l[j][k] * u[k][i];
                }
                u[j][i] = aprime[j][i] - s;
            }
            if j >= i {
                for k in 0..i {
                    s += l[j][k] * u[k][i];
                }
                l[j][i] = (aprime[j][i] - s) / u[i][i];
            }
        }
    }

    (l, u, p)
}

fn main() {
    let a3 = vec![
        vec![1.0, 3.0, 5.0],
        vec![2.0, 4.0, 7.0],
        vec![1.0, 1.0, 0.0],
    ];

    let a4 = vec![
        vec![11.0, 9.0, 24.0, 2.0],
        vec![1.0, 5.0, 2.0, 6.0],
        vec![3.0, 17.0, 18.0, 1.0],
        vec![2.0, 5.0, 7.0, 1.0],
    ];

    let (l, u, p) = mat_lu(&a3);
    println!("A3 =");
    mat_show(&a3, None);
    println!("L =");
    mat_show(&l, None);
    println!("U =");
    mat_show(&u, None);
    println!("P =");
    mat_show(&p, None);

    println!();

    let (l, u, p) = mat_lu(&a4);
    println!("A4 =");
    mat_show(&a4, None);
    println!("L =");
    mat_show(&l, None);
    println!("U =");
    mat_show(&u, None);
    println!("P =");
    mat_show(&p, None);
}