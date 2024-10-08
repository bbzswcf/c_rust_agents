fn transpose(dest: &mut Vec<Vec<f64>>, src: &Vec<Vec<f64>>) {
    let src_h = src.len();
    let src_w = src[0].len();
    // 修改: 确保 dest 的尺寸与 src 的转置尺寸一致
    for i in 0..src_w {
        dest[i].resize(src_h, 0.0);
    }
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
            print!("{}", b[i][j]);
            if j < 2 {
                print!(" ");
            } else {
                println!();
            }
        }
    }
}