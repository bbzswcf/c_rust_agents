type Mat = Vec<Vec<f64>>;

macro_rules! foreach {
    ($i:ident, $b:expr, $e:expr, $body:block) => {
        for $i in $b..$e { $body }
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
        let tmp = $x; // Ensure tmp is defined within the scope of the macro
        $x = $y;
        $y = tmp;
    };
}

macro_rules! _sum_k {
    ($a:expr, $b:expr, $c:expr, $s:ident) => {
        let mut $s = 0.0; // Ensure s is defined within the scope of the macro
        foreach!(k, $a, $b, { $s += $c; })
    };
}

fn mat_zero(x: &mut Mat) {
    for_ij!(x.len(), { x[i][j] = 0.0; }); // Added semicolon to resolve unexpected token error
}

fn mat_new(n: usize) -> Mat {
    let mut x = vec![vec![0.0; n]; n]; // Ensure n is defined within the scope of the function
    mat_zero(&mut x);
    x
}

fn mat_copy(s: &Mat) -> Mat {
    let n = s.len(); // Ensure n is defined within the scope of the function
    let mut x = mat_new(n);
    for_ij!(n, { x[i][j] = s[i][j]; }); // Added semicolon to resolve unexpected token error
    x
}

fn mat_show(x: &Mat, fmt: Option<&str>) {
    let fmt = fmt.unwrap_or("%8.4g"); // Ensure fmt is defined within the scope of the function
    for_i!(x.len(), {
        print!("{}", if i == 0 { " [ " } else { "      " });
        for_j!(x.len(), {
            print!("{}{}", fmt, x[i][j]); // Modified to include a string literal for formatting
            // Collapsed the nested `if` block to simplify the code
            print!("{}", if j < x.len() - 1 { "  " } else if i == x.len() - 1 { " ]\n" } else { "\n" });
        })
    })
}

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let n = a.len(); // Ensure n is defined within the scope of the function
    let mut c = mat_new(n);
    for_ijk!(n, { c[i][j] += a[i][k] * b[k][j]; }); // Added semicolon to resolve unexpected token error
    c
}

fn mat_pivot(a: &Mat, p: &mut Mat) {
    let n = a.len(); // Ensure n is defined within the scope of the function
    for_ij!(n, { p[i][j] = if i == j { 1.0 } else { 0.0 }; }); // Added semicolon to resolve unexpected token error
    for_i!(n, {
        let mut max_j = i; // Ensure max_j is defined within the scope of the function
        foreach!(j, i, n, {
            if a[j][i].abs() > a[max_j][i].abs() {
                max_j = j;
            }
        });
        if max_j != i {
            for_k!(n, { _swap!(p[i][k], p[max_j][k]); }); // Added semicolon to resolve unexpected token error
        }
    })
}

fn mat_lu(a: &Mat, l: &mut Mat, u: &mut Mat, p: &mut Mat) {
    let n = a.len(); // Ensure n is defined within the scope of the function
    mat_zero(l);
    mat_zero(u);
    mat_pivot(a, p);

    let aprime = mat_mul(p, a); // Ensure aprime is defined within the scope of the function

    for_i!(n, { l[i][i] = 1.0; }); // Added semicolon to resolve unexpected token error
    for_ij!(n, {
        let mut s; // Ensure s is defined within the scope of the function
        if j <= i {
            _sum_k!(0, j, l[j][k] * u[k][i], s);
            u[j][i] = aprime[j][i] - s;
        }
        if j >= i {
            _sum_k!(0, i, l[j][k] * u[k][i], s);
            l[j][i] = (aprime[j][i] - s) / u[i][i];
        }
    })
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

    let mut l = mat_new(3); // Ensure l is defined within the scope of the function
    let mut p = mat_new(3); // Ensure p is defined within the scope of the function
    let mut u = mat_new(3); // Ensure u is defined within the scope of the function
    let a = mat_copy(&a3); // Ensure a is defined within the scope of the function
    mat_lu(&a, &mut l, &mut u, &mut p);
    println!("A =");
    mat_show(&a, None);
    println!("L =");
    mat_show(&l, None);
    println!("U =");
    mat_show(&u, None);
    println!("P =");
    mat_show(&p, None);

    println!("\n");

    let mut l = mat_new(4); // Ensure l is defined within the scope of the function
    let mut p = mat_new(4); // Ensure p is defined within the scope of the function
    let mut u = mat_new(4); // Ensure u is defined within the scope of the function
    let a = mat_copy(&a4); // Ensure a is defined within the scope of the function
    mat_lu(&a, &mut l, &mut u, &mut p);
    println!("A =");
    mat_show(&a, None);
    println!("L =");
    mat_show(&l, None);
    println!("U =");
    mat_show(&u, None);
    println!("P =");
    mat_show(&p, None);
}