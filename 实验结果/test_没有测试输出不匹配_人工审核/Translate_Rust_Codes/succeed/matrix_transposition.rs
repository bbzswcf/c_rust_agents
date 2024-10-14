fn transpose(dest: &mut [f64], src: &[f64], src_h: usize, src_w: usize) {
    // Ensure the sizes match the expected dimensions
    // Use safe indexing instead of unsafe casting
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

    // Modified: Flatten the arrays manually since `flatten_mut` and `flatten` methods are not implemented
    let src_flat: Vec<f64> = a.iter().flat_map(|row| row.iter()).cloned().collect();
    let mut dest_flat: Vec<f64> = b.iter().flat_map(|row| row.iter()).cloned().collect();

    transpose(&mut dest_flat, &src_flat, 3, 5);

    // Copy the flattened result back into the 2D array
    for (i, &value) in dest_flat.iter().enumerate() {
        let row = i / 3;
        let col = i % 3;
        b[row][col] = value;
    }

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