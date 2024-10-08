fn swap_row(a: &mut [f64], b: &mut [f64], r1: usize, r2: usize, n: usize) {
    if r1 == r2 {
        return;
    }
    for i in 0..n {
        // 修改: 使用临时变量tmp进行交换
        let tmp = a[r1 * n + i];
        a[r1 * n + i] = a[r2 * n + i];
        a[r2 * n + i] = tmp;
    }
    b.swap(r1, r2);
}

fn gauss_eliminate(a: &mut [f64], b: &mut [f64], x: &mut [f64], n: usize) {
    for dia in 0..n {
        let mut max_row = dia;
        let mut max = a[dia * n + dia];

        for row in dia + 1..n {
            let tmp = f64::abs(a[row * n + dia]);
            // 修改: 使用f64::EPSILON进行比较
            if tmp > max + f64::EPSILON {
                max_row = row;
                max = tmp;
            }
        }

        swap_row(a, b, dia, max_row, n);

        for row in dia + 1..n {
            // 修改: 使用f64::EPSILON进行比较
            if a[dia * n + dia].abs() < f64::EPSILON {
                panic!("Division by zero detected");
            }
            let tmp = a[row * n + dia] / a[dia * n + dia];
            for col in dia + 1..n {
                a[row * n + col] -= tmp * a[dia * n + col];
            }
            a[row * n + dia] = 0.0;
            b[row] -= tmp * b[dia];
        }
    }

    for row in (0..n).rev() {
        let mut tmp = b[row];
        for j in (row + 1..n).rev() {
            tmp -= x[j] * a[row * n + j];
        }
        // 修改: 使用f64::EPSILON进行比较
        if a[row * n + row].abs() < f64::EPSILON {
            panic!("Division by zero detected");
        }
        x[row] = tmp / a[row * n + row];
    }
}

fn main() {
    let a = [
        1.00, 0.00, 0.00,  0.00,  0.00, 0.00,
        1.00, 0.63, 0.39,  0.25,  0.16, 0.10,
        1.00, 1.26, 1.58,  1.98,  2.49, 3.13,
        1.00, 1.88, 3.55,  6.70, 12.62, 23.80,
        1.00, 2.51, 6.32, 15.88, 39.90, 100.28,
        1.00, std::f64::consts::PI, 9.87, 31.01, 97.41, 306.02 // 修改: 使用std::f64::consts::PI
    ];
    let b = [ -0.01, 0.61, 0.91, 0.99, 0.60, 0.02 ];
    // 修改: 使用Vec<f64>存储x
    let mut x = vec![0.0; 6];

    // 修改: 直接传递数组切片，避免不必要的可变引用
    gauss_eliminate(&mut a, &mut b, &mut x, 6);

    for i in 0..6 {
        println!("{}", x[i]);
    }
}