use std::f64::consts::PI;

// Modified: Changed the return type to &mut f64 to allow mutable references
fn mat_elem(a: &mut [f64], y: usize, x: usize, n: usize) -> &mut f64 {
    &mut a[y * n + x]
}

fn swap_row(a: &mut [f64], b: &mut [f64], r1: usize, r2: usize, n: usize) {
    if r1 == r2 {
        return;
    }
    for i in 0..n {
        // Modified: Perform the swap operation in a single call to `mat_elem`
        let tmp = *mat_elem(a, r1, i, n);
        *mat_elem(a, r1, i, n) = *mat_elem(a, r2, i, n);
        *mat_elem(a, r2, i, n) = tmp;
    }
    let tmp = b[r1];
    b[r1] = b[r2];
    b[r2] = tmp;
}

fn gauss_eliminate(a: &mut [f64], b: &mut [f64], x: &mut [f64], n: usize) {
    for dia in 0..n {
        let mut max_row = dia;
        let mut max = *mat_elem(a, dia, dia, n);

        for row in (dia + 1)..n {
            let tmp = mat_elem(a, row, dia, n).abs();
            if tmp > max {
                max_row = row;
                max = tmp;
            }
        }

        swap_row(a, b, dia, max_row, n);

        // Modified: Ensure the matrix `a` is properly conditioned before performing the division
        let dia_elem = *mat_elem(a, dia, dia, n);
        if dia_elem.abs() < 1e-10 {
            panic!("Division by zero detected");
        }

        for row in (dia + 1)..n {
            let tmp = *mat_elem(a, row, dia, n) / dia_elem;
            for col in (dia + 1)..n {
                // Modified: Changed to use mutable reference to allow assignment
                *mat_elem(a, row, col, n) -= tmp * *mat_elem(a, dia, col, n);
            }
            // Modified: Changed to use mutable reference to allow assignment
            *mat_elem(a, row, dia, n) = 0.0;
            b[row] -= tmp * b[dia];
        }
    }

    for row in (0..n).rev() {
        let mut tmp = b[row];
        for j in (row + 1)..n {
            tmp -= x[j] * *mat_elem(a, row, j, n);
        }
        x[row] = tmp / *mat_elem(a, row, row, n);
    }
}

fn main() {
    let a = &mut [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, PI, 9.87, 31.01, 97.41, 306.02 // Modified: Replaced approximate value of PI with the constant
    ];
    let b = &mut [ -0.01, 0.61, 0.91, 0.99, 0.60, 0.02 ];
    let x = &mut [0.0; 6];

    gauss_eliminate(a, b, x, 6);

    for i in 0..6 {
        // Modified: Ensure the output format matches the expected format
        println!("{:.6}", x[i]);
    }
}