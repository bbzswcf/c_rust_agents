use std::i32;

fn optimal_matrix_chain_order(dims: &[i32], n: usize) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut m = vec![vec![0; n - 1]; n - 1];
    let mut s = vec![vec![0; n - 1]; n - 1];

    for len in 1..n - 1 {
        for i in 0..n - 1 - len {
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

fn print_optimal_chain_order(s: &[Vec<i32>], i: i32, j: i32) {
    if i == j {
        print!("{}", (i + 65) as u8 as char);
    } else {
        print!("(");
        print_optimal_chain_order(s, i, s[i as usize][j as usize]);
        print_optimal_chain_order(s, s[i as usize][j as usize] + 1, j);
        print!(")");
    }
}

fn main() {
    let a1 = vec![5, 6, 3, 1];
    let a2 = vec![1, 5, 25, 30, 100, 70, 2, 1, 100, 250, 1, 1000, 2];
    let a3 = vec![1000, 1, 500, 12, 1, 700, 2500, 3, 2, 5, 14, 10];
    let dims_list = vec![a1, a2, a3];
    let sizes = vec![4, 13, 12];

    for (i, dims) in dims_list.iter().enumerate() {
        print!("Dims  : [");
        let n = sizes[i];
        for j in 0..n {
            print!("{}", dims[j]);
            if j < n - 1 {
                print!(", ");
            } else {
                println!("]");
            }
        }

        let (m, s) = optimal_matrix_chain_order(dims, n);
        print!("Order : ");
        print_optimal_chain_order(&s, 0, (n - 2) as i32);
        println!("\nCost  : {}", m[0][n - 2]);
        println!();
    }
}