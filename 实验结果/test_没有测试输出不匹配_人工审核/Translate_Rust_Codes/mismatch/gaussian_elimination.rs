use std::f64::consts::{E, PI};

fn mat_elem(a: &mut [f64], y: usize, x: usize, n: usize) -> &mut f64 {
    &mut a[y * n + x]
}

fn swap_row(a: &mut [f64], b: &mut [f64], r1: usize, r2: usize, n: usize) {
    if r1 == r2 {
        return;
    }
    // Modified: Perform the swap operation in a single mutable borrow scope
    for i in 0..n {
        a.swap(r1 * n + i, r2 * n + i);
    }
    b.swap(r1, r2);
}

fn gauss_eliminate(a: &mut [f64], b: &mut [f64], x: &mut [f64], n: usize) {
    for dia in 0..n {
        let mut max_row = dia;
        let mut max = *mat_elem(a, dia, dia, n);

        for row in (dia + 1)..n {
            let tmp = f64::abs(*mat_elem(a, row, dia, n));
            if tmp > max {
                max_row = row;
                max = tmp;
            }
        }

        swap_row(a, b, dia, max_row, n);

        for row in (dia + 1)..n {
            // Modified: Perform the subtraction operation in a single mutable borrow scope
            let tmp = *mat_elem(a, row, dia, n) / *mat_elem(a, dia, dia, n);
            for col in (dia + 1)..n {
                *mat_elem(a, row, col, n) -= tmp * *mat_elem(a, dia, col, n);
            }
            *mat_elem(a, row, dia, n) = 0.0;
            b[row] -= tmp * b[dia];
        }
    }

    for row in (0..n).rev() {
        // Modified: Perform the operations in a single mutable borrow scope
        let mut tmp = b[row];
        for j in (row + 1)..n {
            tmp -= x[j] * *mat_elem(a, row, j, n);
        }
        x[row] = tmp / *mat_elem(a, row, row, n);
    }
}

fn main() {
    let mut a = [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, PI, 9.87, 31.01, 97.41, 306.02 // Modified: Replaced approximate value 3.14 with std::f64::consts::PI
    ];
    let mut b = [-0.01, 0.61, 0.91, 0.99, 0.60, 0.02];
    let mut x = [0.0; 6];

    gauss_eliminate(&mut a, &mut b, &mut x, 6);

    for val in x.iter() { // Modified: Replaced range-based loop with an iterator over the elements of x
        println!("{}", val);
    }
}