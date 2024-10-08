fn transpose(dest: &mut [f64], src: &[f64], src_h: usize, src_w: usize) {
    for i in 0..src_h {
        for j in 0..src_w {
            dest[j * src_h + i] = src[i * src_w + j];
        }
    }
}

fn main() {
    let a = [
        [0.0, 1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
        [1.0, 0.0, 0.0, 0.0, 42.0],
    ];
    let mut b = [[0.0; 3]; 5];

    let src: Vec<f64> = a.iter().flatten().cloned().collect();
    let mut dest: Vec<f64> = b.iter().flatten().cloned().collect();

    transpose(&mut dest, &src, 3, 5);

    let b: Vec<[f64; 3]> = dest.chunks_exact(3).map(|chunk| {
        let mut arr = [0.0; 3];
        arr.copy_from_slice(chunk);
        arr
    }).collect();

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