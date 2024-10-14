use std::mem;
use std::ptr;
use std::fmt;

type Mat = Vec<Vec<f64>>;

macro_rules! foreach {
    ($i:ident, $b:expr, $e:expr, $body:block) => {
        for $i in $b..$e $body
    };
}

macro_rules! for_i {
    ($n:expr, $body:block) => {
        foreach!(i, 0, $n, $body)
    };
}

macro_rules! for_j {
    ($n:expr, $body:block) => {
        foreach!(j, 0, $n, $body)
    };
}

macro_rules! for_k {
    ($n:expr, $body:block) => {
        foreach!(k, 0, $n, $body)
    };
}

macro_rules! for_ij {
    ($n:expr, $body:block) => {
        for_i!($n, { for_j!($n, $body) })
    };
}

macro_rules! for_ijk {
    ($n:expr, $body:block) => {
        for_ij!($n, { for_k!($n, $body) })
    };
}

macro_rules! _swap {
    ($x:expr, $y:expr) => {
        { let tmp = $x; $x = $y; $y = tmp; }
    };
}

macro_rules! _sum_k {
    ($a:expr, $b:expr, $c:expr, $s:expr) => {
        { $s = 0.0; foreach!(k, $a, $b, { $s += $c; }) }
    };
}

fn mat_zero(x: &mut Mat, n: usize) {
    for_ij!(n, { x[i][j] = 0.0; })
}

fn mat_new(n: usize) -> Mat {
    let mut x = vec![vec![0.0; n]; n];
    mat_zero(&mut x, n);
    x
}

fn mat_copy(s: &Mat, n: usize) -> Mat {
    let mut x = mat_new(n);
    for_ij!(n, { x[i][j] = s[i][j]; });
    x
}

fn mat_del(_x: Mat) {}

fn mat_show(x: &Mat, fmt: Option<&str>, n: usize) {
    let fmt = fmt.unwrap_or("%8.4g");
    for_i!(n, {
        if i > 0 {
            print!("      ");
        } else {
            print!(" [ ");
        }
        for_j!(n, {
            // Modified: Replaced `fmt!` with `format!` for string formatting
            print!("{}", format!("{}", x[i][j]));
            if j < n - 1 {
                print!("  ");
            } else if i == n - 1 {
                println!(" ]");
            } else {
                println!();
            }
        });
    });
}

fn mat_mul(a: &Mat, b: &Mat, n: usize) -> Mat {
    let mut c = mat_new(n);
    for_ijk!(n, { c[i][j] += a[i][k] * b[k][j]; });
    c
}

fn mat_pivot(a: &Mat, p: &mut Mat, n: usize) {
    for_ij!(n, { p[i][j] = if i == j { 1.0 } else { 0.0 }; });
    for_i!(n, {
        let mut max_j = i; // Modified: Define `max_j` before using it
        foreach!(j, i, n, {
            if a[j][i].abs() > a[max_j][i].abs() {
                max_j = j;
            }
        });
        if max_j != i {
            for_k!(n, { _swap!(p[i][k], p[max_j][k]); });
        }
    });
}

fn mat_lu(a: &Mat, l: &mut Mat, u: &mut Mat, p: &mut Mat, n: usize) {
    mat_zero(l, n);
    mat_zero(u, n);
    mat_pivot(a, p, n);

    let aprime = mat_mul(p, a, n);

    for_i!(n, { l[i][i] = 1.0; });
    for_ij!(n, {
        let mut s = 0.0; // Modified: Define `s` before using it in _sum_k!
        if j <= i {
            _sum_k!(0, j, l[j][k] * u[k][i], s);
            u[j][i] = aprime[j][i] - s;
        }
        if j >= i {
            _sum_k!(0, i, l[j][k] * u[k][i], s);
            l[j][i] = (aprime[j][i] - s) / u[i][i];
        }
    });

    mat_del(aprime);
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

    let n = 3;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
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
    mat_del(a);
    mat_del(l);
    mat_del(u);
    mat_del(p);

    println!();

    let n = 4;
    let mut l = mat_new(n);
    let mut p = mat_new(n);
    let mut u = mat_new(n);
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
    mat_del(a);
    mat_del(l);
    mat_del(u);
    mat_del(p);
}