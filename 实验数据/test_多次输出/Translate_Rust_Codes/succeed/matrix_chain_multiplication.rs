use std::cmp;

fn optimal_matrix_chain_order(dims: &[i32]) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let n = dims.len() - 1;
    let mut m = vec![vec![0; n]; n];
    let mut s = vec![vec![0; n]; n];

    for len in 1..n {
        for i in 0..n - len {
            let j = i + len;
            m[i][j] = i32::MAX;
            for k in i..j {
                let temp = dims[i] * dims[k + 1] * dims[j + 1];
                let cost = m[i][k] + m[k + 1][j] + temp;
                if cost < m[i][j] {
                    m[i][j] = cost;
                    s[i][j] = k as i32;
                }
            }
        }
    }

    (m, s)
}

fn print_optimal_chain_order(s: &[Vec<i32>], i: usize, j: usize) {
    if i == j {
        print!("{}", (i as u8 + 65) as char);
    } else {
        print!("(");
        print_optimal_chain_order(s, i, s[i][j] as usize);
        print_optimal_chain_order(s, s[i][j] as usize + 1, j);
        print!(")");
    }
}

fn main() {
    let dims_list = vec![
        vec![5, 6, 3, 1],
        vec![1, 5, 25, 30, 100, 70, 2, 1, 100, 250, 1, 1000, 2],
        vec![1000, 1, 500, 12, 1, 700, 2500, 3, 2, 5, 14, 10],
    ];

    for dims in dims_list {
        print!("Dims  : [");
        for (j, &dim) in dims.iter().enumerate() {
            print!("{}", dim);
            if j < dims.len() - 1 {
                print!(", ");
            } else {
                print!("]\n");
            }
        }

        let (m, s) = optimal_matrix_chain_order(&dims);
        print!("Order : ");
        print_optimal_chain_order(&s, 0, dims.len() - 2);
        println!("\nCost  : {}", m[0][dims.len() - 2]);
        println!();
    }
}