use std::process;

struct SquareMatrix {
    n: usize,
    elems: Vec<Vec<f64>>,
}

impl SquareMatrix {
    fn init_square_matrix(n: usize, elems: Vec<Vec<f64>>) -> Self {
        let mut matrix = SquareMatrix {
            n,
            elems: Vec::with_capacity(n),
        };
        for i in 0..n {
            matrix.elems.push(Vec::with_capacity(n));
            for j in 0..n {
                matrix.elems[i].push(elems[i][j]);
            }
        }
        matrix
    }

    fn copy_square_matrix(&self) -> Self {
        let mut matrix = SquareMatrix {
            n: self.n,
            elems: Vec::with_capacity(self.n),
        };
        for i in 0..self.n {
            matrix.elems.push(Vec::with_capacity(self.n));
            for j in 0..self.n {
                matrix.elems[i].push(self.elems[i][j]);
            }
        }
        matrix
    }

    fn det(&self) -> f64 {
        let mut det = 1.0;
        let mut matrix = self.copy_square_matrix();

        for j in 0..matrix.n {
            let mut i_max = j;
            for i in j..matrix.n {
                if matrix.elems[i][j] > matrix.elems[i_max][j] {
                    i_max = i;
                }
            }

            if i_max != j {
                for k in 0..matrix.n {
                    let tmp = matrix.elems[i_max][k];
                    matrix.elems[i_max][k] = matrix.elems[j][k];
                    matrix.elems[j][k] = tmp;
                }
                det *= -1.0;
            }

            if f64::abs(matrix.elems[j][j]) < 1e-12 {
                println!("Singular matrix!");
                return f64::NAN;
            }

            for i in (j + 1)..matrix.n {
                let mult = -matrix.elems[i][j] / matrix.elems[j][j];
                for k in 0..matrix.n {
                    matrix.elems[i][k] += mult * matrix.elems[j][k];
                }
            }
        }

        for i in 0..matrix.n {
            det *= matrix.elems[i][i];
        }

        det
    }

    fn cramer_solve(&self, det_a: f64, b: &[f64], var: usize) -> f64 {
        let mut tmp = self.copy_square_matrix();
        for i in 0..tmp.n {
            tmp.elems[i][var] = b[i];
        }

        let det_tmp = tmp.det();
        det_tmp / det_a
    }
}

fn main() {
    const N: usize = 4;
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
        println!("{:7.3}", a.cramer_solve(det_a, &b, i));
    }

    process::exit(0);
}