// Define a newtype wrapper for Vec<Vec<f64>>
struct Mat(Vec<Vec<f64>>);

impl Mat {
    // Method to swap elements in the matrix
    fn swap(&mut self, i1: usize, j1: usize, i2: usize, j2: usize) {
        let temp = self.0[i1][j1];
        self.0[i1][j1] = self.0[i2][j2];
        self.0[i2][j2] = temp;
    }
}

// Function to initialize a matrix with zeros
fn mat_zero(x: &mut Mat, n: usize) {
    for i in 0..n {
        for j in 0..n {
            x.0[i][j] = 0.0;
        }
    }
}

// Function to create a new matrix initialized with zeros
fn mat_new(n: usize) -> Mat {
    Mat(vec![vec![0.0; n]; n])
}

// Function to copy a matrix
fn mat_copy(s: &Mat, n: usize) -> Mat {
    let mut x = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            x.0[i][j] = s.0[i][j];
        }
    }
    x
}

// Function to display a matrix
fn mat_show(x: &Mat, fmt: Option<&str>, n: usize) {
    let fmt = fmt.unwrap_or("%8.4g");
    for i in 0..n {
        print!("{}", if i == 0 { " [ " } else { "      " });
        for j in 0..n {
            print!("{:>8.4}", x.0[i][j]); // Corrected: Use format specifier {:>8.4}
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

// Function to multiply two matrices
fn mat_mul(a: &Mat, b: &Mat, n: usize) -> Mat {
    let mut c = mat_new(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c.0[i][j] += a.0[i][k] * b.0[k][j]; // Corrected: Inner loop should iterate over columns of a and rows of b
            }
        }
    }
    c
}

// Function to pivot a matrix
fn mat_pivot(a: &mut Mat, p: &mut Mat, n: usize) {
    for i in 0..n {
        for j in 0..n {
            p.0[i][j] = if i == j { 1.0 } else { 0.0 };
        }
    }
    for i in 0..n {
        let mut max_j = i;
        for j in i..n {
            if f64::abs(a.0[j][i]) > f64::abs(a.0[max_j][i]) {
                max_j = j;
            }
        }
        if max_j != i {
            for k in 0..n {
                p.swap(i, k, max_j, k); // Corrected: Swap rows in pivot matrix
                a.swap(i, k, max_j, k); // Corrected: Swap rows in matrix a
            }
        }
    }
}

// Function to perform LU decomposition
fn mat_lu(a: &mut Mat, l: &mut Mat, u: &mut Mat, p: &mut Mat, n: usize) {
    mat_zero(l, n);
    mat_zero(u, n);
    mat_pivot(a, p, n);

    // Modified: Removed `&mut` from `p` since it is already a mutable borrow
    // Modified: Removed `&mut` from `a` since it is already a mutable borrow
    let aprime = mat_mul(p, a, n);

    for i in 0..n {
        l.0[i][i] = 1.0;
    }
    for i in 0..n {
        for j in 0..n {
            let mut s = 0.0;
            if j <= i {
                for k in 0..j {
                    s += l.0[j][k] * u.0[k][i];
                }
                u.0[j][i] = aprime.0[j][i] - s;
            }
            if j >= i {
                for k in 0..i {
                    s += l.0[j][k] * u.0[k][i];
                }
                if u.0[i][i] != 0.0 {
                    l.0[j][i] = (aprime.0[j][i] - s) / u.0[i][i];
                } else {
                    // Corrected: Handle division by zero gracefully
                    l.0[j][i] = 0.0;
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
    let a3 = Mat(vec![vec![1.0, 3.0, 5.0], vec![2.0, 4.0, 7.0], vec![1.0, 1.0, 0.0]]);
    let mut a = mat_copy(&a3, n); // Corrected: Ensure a is mutable
    mat_lu(&mut a, &mut l, &mut u, &mut p, n); // Corrected: Ensure a is passed as mutable
    println!("A =");
    mat_show(&a, None, n);
    println!("L =");
    mat_show(&l, None, n);
    println!("U =");
    mat_show(&u, None, n);
    println!("P =");
    mat_show(&p, None, n);

    println!();

    let n = 4;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
    let a4 = Mat(vec![
        vec![11.0, 9.0, 24.0, 2.0],
        vec![1.0, 5.0, 2.0, 6.0],
        vec![3.0, 17.0, 18.0, 1.0],
        vec![2.0, 5.0, 7.0, 1.0],
    ]);
    let mut a = mat_copy(&a4, n); // Corrected: Ensure a is mutable
    mat_lu(&mut a, &mut l, &mut u, &mut p, n); // Corrected: Ensure a is passed as mutable
    println!("A =");
    mat_show(&a, None, n);
    println!("L =");
    mat_show(&l, None, n);
    println!("U =");
    mat_show(&u, None, n);
    println!("P =");
    mat_show(&p, None, n);
}