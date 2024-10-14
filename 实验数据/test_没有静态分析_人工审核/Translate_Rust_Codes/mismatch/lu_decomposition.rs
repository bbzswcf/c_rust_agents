type Mat = Vec<Vec<f64>>;

fn mat_zero(x: &mut Mat, n: usize) {
    for i in 0..n {
        for j in 0..n {
            x[i][j] = 0.0; // Ensure all elements are set to 0.0
        }
    }
}

fn mat_new(n: usize) -> Mat {
    let mut x = Vec::with_capacity(n);
    for _ in 0..n {
        x.push(vec![0.0; n]); // Ensure all elements are initialized to 0.0
    }
    x
}

fn mat_copy(s: &Mat, n: usize) -> Mat {
    let mut x = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            x[i][j] = s[i][j]; // Ensure all elements are correctly copied
        }
    }
    x
}

fn mat_show(x: &Mat, fmt: Option<&str>, n: usize) {
    let fmt = fmt.unwrap_or("%8.4g");
    for i in 0..n {
        print!("{}", if i == 0 { " [ " } else { "   " });
        for j in 0..n {
            print!("{:>8.4}", x[i][j]); // Ensure elements are correctly formatted and aligned
            print!("{}", if j < n - 1 { "  " } else if i == n - 1 { " ]\n" } else { "\n" });
        }
    }
}

fn mat_mul(a: &Mat, b: &Mat, n: usize) -> Mat {
    let mut c = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j]; // Ensure matrix multiplication logic is correct
            }
        }
    }
    c
}

fn mat_pivot(a: &Mat, p: &mut Mat, n: usize) {
    for i in 0..n {
        for j in 0..n {
            p[i][j] = if i == j { 1.0 } else { 0.0 }; // Ensure diagonal elements are set to 1.0
        }
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
                let tmp = p[i][k];
                p[i][k] = p[max_j][k];
                p[max_j][k] = tmp; // Ensure matrix elements are correctly swapped
            }
        }
    }
}

fn mat_lu(a: &Mat, l: &mut Mat, u: &mut Mat, p: &mut Mat, n: usize) {
    mat_zero(l, n);
    mat_zero(u, n);
    mat_pivot(a, p, n);

    let aprime = mat_mul(p, a, n);

    for i in 0..n {
        l[i][i] = 1.0;
    }
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            if j >= i { // Ensure correct condition for calculating u[j][i]
                for k in 0..i {
                    s += l[j][k] * u[k][i];
                }
                u[j][i] = aprime[j][i] - s; // Ensure matrix elements are correctly assigned
            }
            if j < i { // Ensure correct condition for calculating l[j][i]
                for k in 0..j {
                    s += l[j][k] * u[k][i];
                }
                l[j][i] = (aprime[j][i] - s) / u[j][j]; // Ensure matrix elements are correctly assigned
            }
        }
    }
}

fn main() {
    let n = 3;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
    let a3 = vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 7.0], vec![1.0, 1.0, 0.0]];
    let a = mat_copy(&a3, n);
    mat_lu(&a, &mut l, &mut u, &mut p, n);
    println!("A =");
    mat_show(&a, None, n);
    println!("L =");
    mat_show(&l, None, n);
    println!("U =");
    mat_show(&u, None, n);
    println!("P =");
    mat_show(&p, None, n);

    println!("\n");

    let n = 4;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
    let a4 = vec![
        vec![11.0, 9.0, 24.0, 2.0],
        vec![1.0, 5.0, 2.0, 6.0],
        vec![3.0, 17.0, 18.0, 1.0],
        vec![2.0, 5.0, 7.0, 1.0],
    ];
    let a = mat_copy(&a4, n);
    mat_lu(&a, &mut l, &mut u, &mut p, n);
    println!("A =");
    mat_show(&a, None, n);
    println!("L =");
    mat_show(&l, None, n);
    println!("U =");
    mat_show(&u, None, n);
    println!("P =");
    mat_show(&p, None, n);
}