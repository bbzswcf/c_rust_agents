fn mat_zero(x: &mut [Vec<f64>], n: usize) {
    // 修改: 使用 iter_mut().flatten() 将所有元素设置为 0.0
    x.iter_mut().flatten().for_each(|e| *e = 0.0);
}

fn mat_new(n: usize) -> Vec<Vec<f64>> {
    // 修改: 创建一个 n x n 的矩阵，所有元素初始化为 0.0
    vec![vec![0.0; n]; n]
}

fn mat_copy(s: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    // 修改: 使用 to_vec() 进行矩阵的深拷贝
    s.to_vec()
}

fn mat_show(x: &[Vec<f64>], fmt: Option<&str>, n: usize) {
    let fmt = fmt.unwrap_or("%8.4g");
    for i in 0..n {
        if i > 0 {
            print!("      ");
        } else {
            print!(" [ ");
        }
        for j in 0..n {
            // 修改: 使用 format! 进行格式化输出
            if fmt == "%8.4g" {
                print!("{:8.4}", x[i][j]);
            } else {
                print!("{}", x[i][j]);
            }
            if j < n - 1 {
                print!("  ");
            } else if i == n - 1 {
                print!(" ]\n");
            } else {
                print!("\n");
            }
        }
    }
}

fn mat_mul(a: &[Vec<f64>], b: &[Vec<f64>], n: usize) -> Vec<Vec<f64>> {
    let mut c = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                // 修改: 使用 += 进行矩阵乘法
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

fn mat_pivot(a: &[Vec<f64>], p: &mut [Vec<f64>], n: usize) {
    for i in 0..n {
        for j in 0..n {
            p[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }
    for i in 0..n {
        let mut max_j = i;
        for j in i..n {
            if f64::abs(a[j][i]) > f64::abs(a[max_j][i]) {
                max_j = j;
            }
        }
        if max_j != i {
            for k in 0..n {
                let tmp = p[i][k];
                p[i][k] = p[max_j][k];
                p[max_j][k] = tmp;
            }
        }
    }
}

fn mat_lu(a: &[Vec<f64>], l: &mut [Vec<f64>], u: &mut [Vec<f64>], p: &mut [Vec<f64>], n: usize) {
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
                if u[i][i] != 0.0 {
                    l[j][i] = (aprime[j][i] - s) / u[i][i];
                }
            }
        }
    }
}

fn main() {
    let n = 3;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
    let a = mat_copy(&vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 7.0], vec![1.0, 1.0, 0.0]], n);
    mat_lu(&a, &mut l, &mut u, &mut p, n);
    print!("A ="); mat_show(&a, None, n);
    print!("L ="); mat_show(&l, None, n);
    print!("U ="); mat_show(&u, None, n);
    print!("P ="); mat_show(&p, None, n);

    print!("\n");

    let n = 4;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
    let a = mat_copy(&vec![vec![11.0, 9.0, 24.0, 2.0], vec![1.0, 5.0, 2.0, 6.0], vec![3.0, 17.0, 18.0, 1.0], vec![2.0, 5.0, 7.0, 1.0]], n);
    mat_lu(&a, &mut l, &mut u, &mut p, n);
    print!("A ="); mat_show(&a, None, n);
    print!("L ="); mat_show(&l, None, n);
    print!("U ="); mat_show(&u, None, n);
    print!("P ="); mat_show(&p, None, n);
}