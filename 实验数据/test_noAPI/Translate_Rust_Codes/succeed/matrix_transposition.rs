use std::fmt::Write;

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

    let src_h = 3;
    let src_w = 5;

    let src_flat: Vec<f64> = a.iter().flatten().cloned().collect();
    let mut dest_flat: Vec<f64> = vec![0.0; src_h * src_w];

    transpose(&mut dest_flat, &src_flat, src_h, src_w);

    let mut output = String::new();
    for i in 0..5 {
        for j in 0..3 {
            write!(output, "{}", dest_flat[i * 3 + j]).unwrap();
            if j < 2 {
                write!(output, " ").unwrap();
            } else {
                writeln!(output).unwrap();
            }
        }
    }
    print!("{}", output);
}