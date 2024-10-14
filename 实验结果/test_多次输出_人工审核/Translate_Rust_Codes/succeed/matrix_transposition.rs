fn transpose(dest: &mut [Vec<f64>], src: &[Vec<f64>]) {
    let src_h = src.len();
    let src_w = src[0].len();
    for i in 0..src_h {
        for j in 0..src_w {
            dest[j][i] = src[i][j];
        }
    }
}

fn main() {
    let a = vec![
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0, 9.0],
        vec![1.0, 0.0, 0.0, 0.0, 42.0],
    ];
    let mut b = vec![vec![0.0; 3]; 5];

    transpose(&mut b, &a);

    for i in 0..5 {
        for j in 0..3 {
            print!("{}{}", b[i][j], if j == 2 { '\n' } else { ' ' });
        }
    }
}