use std::process::exit;

struct SquareMatrix {
    n: i32,
    elems: Vec<Vec<f64>>,
}

impl SquareMatrix {
    fn init_square_matrix(n: i32, elems: Vec<Vec<f64>>) -> Self {
        let mut matrix = SquareMatrix {
            n,
            elems: Vec::with_capacity(n as usize),
        };
        for i in 0..n {
            matrix.elems.push(Vec::with_capacity(n as usize));
            for j in 0..n {
                matrix.elems[i as usize].push(elems[i as usize][j as usize]);
            }
        }
        matrix
    }

    fn copy_square_matrix(&self) -> Self {
        let mut dest = SquareMatrix {
            n: self.n,
            elems: Vec::with_capacity(self.n as usize),
        };
        for i in 0..self.n {
            dest.elems.push(Vec::with_capacity(self.n as usize));
            for j in 0..self.n {
                dest.elems[i as usize].push(self.elems[i as usize][j as usize]);
            }
        }
        dest
    }

    fn det(&self) -> f64 {
        let mut det = 1.0;
        let mut matrix = self.copy_square_matrix();

        for j in 0..matrix.n {
            let mut i_max = j;
            for i in j..matrix.n {
                if matrix.elems[i as usize][j as usize] > matrix.elems[i_max as usize][j as usize] {
                    i_max = i;
                }
            }

            if i_max != j {
                for k in 0..matrix.n {
                    let tmp = matrix.elems[i_max as usize][k as usize];
                    matrix.elems[i_max as usize][k as usize] = matrix.elems[j as usize][k as usize];
                    matrix.elems[j as usize][k as usize] = tmp;
                }
                det *= -1.0;
            }

            if f64::abs(matrix.elems[j as usize][j as usize]) < 1e-12 {
                println!("Singular matrix!");
                return f64::NAN;
            }

            for i in (j + 1)..matrix.n {
                let mult = -matrix.elems[i as usize][j as usize] / matrix.elems[j as usize][j as usize];
                for k in 0..matrix.n {
                    matrix.elems[i as usize][k as usize] += mult * matrix.elems[j as usize][k as usize];
                }
            }
        }

        for i in 0..matrix.n {
            det *= matrix.elems[i as usize][i as usize];
        }

        det
    }

    fn cramer_solve(&self, det_a: f64, b: &[f64], var: i32) -> f64 {
        let mut tmp = self.copy_square_matrix();
        for i in 0..tmp.n {
            tmp.elems[i as usize][var as usize] = b[i as usize];
        }

        let det_tmp = tmp.det();
        det_tmp / det_a
    }
}

fn main() {
    const N: i32 = 4;
    let elems = vec![
        vec![2.0, -1.0, 5.0, 1.0],
        vec![3.0, 2.0, 2.0, -6.0],
        vec![1.0, 3.0, 3.0, -1.0],
        vec![5.0, -2.0, -3.0, 3.0],
    ];
    let a = SquareMatrix::init_square_matrix(N, elems);

    let tmp = a.copy_square_matrix();
    let det_a = tmp.det();

    let b = vec![-3.0, -32.0, -47.0, 49.0];

    for i in 0..N {
        print!("{:7.3}\n", a.cramer_solve(det_a, &b, i));
    }

    exit(0);
}