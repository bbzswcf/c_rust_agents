use std::f64::NAN;

struct SquareMatrix {
    n: usize,
    elems: Vec<Vec<f64>>,
}

impl SquareMatrix {
    fn new(n: usize, elems: Vec<Vec<f64>>) -> Self {
        Self { n, elems }
    }

    fn copy(&self) -> Self {
        let elems = self.elems.iter().map(|row| row.clone()).collect();
        Self { n: self.n, elems }
    }

    fn det(&mut self) -> f64 {
        let mut det = 1.0;

        for j in 0..self.n {
            let mut i_max = j;
            for i in j..self.n {
                if self.elems[i][j] > self.elems[i_max][j] {
                    i_max = i;
                }
            }

            if i_max != j {
                for k in 0..self.n {
                    let tmp = self.elems[i_max][k];
                    self.elems[i_max][k] = self.elems[j][k];
                    self.elems[j][k] = tmp;
                }
                det *= -1.0;
            }

            if self.elems[j][j].abs() < 1e-12 {
                println!("Singular matrix!");
                return NAN;
            }

            for i in (j + 1)..self.n {
                let mult = -self.elems[i][j] / self.elems[j][j];
                for k in 0..self.n {
                    self.elems[i][k] += mult * self.elems[j][k];
                }
            }
        }

        for i in 0..self.n {
            det *= self.elems[i][i];
        }

        det
    }
}

fn cramer_solve(a: &SquareMatrix, det_a: f64, b: &[f64], var: usize) -> f64 {
    let mut tmp = a.copy();
    for i in 0..tmp.n {
        tmp.elems[i][var] = b[i];
    }

    let det_tmp = tmp.det();
    det_tmp / det_a
}

fn main() {
    const N: usize = 4;
    let elems = vec![
        vec![2.0, -1.0, 5.0, 1.0],
        vec![3.0, 2.0, 2.0, -6.0],
        vec![1.0, 3.0, 3.0, -1.0],
        vec![5.0, -2.0, -3.0, 3.0],
    ];
    let mut a = SquareMatrix::new(N, elems);

    let mut tmp = a.copy();
    let det_a = tmp.det();

    let b = vec![-3.0, -32.0, -47.0, 49.0];

    for i in 0..N {
        println!("{:7.3}", cramer_solve(&a, det_a, &b, i));
    }
}