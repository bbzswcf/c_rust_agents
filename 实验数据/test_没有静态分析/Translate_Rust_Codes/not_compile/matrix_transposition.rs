fn transpose(dest: &mut [f64], src: &[f64], src_h: usize, src_w: usize) {
    for i in 0..src_h {
        for j in 0..src_w {
            dest[j * src_h + i] = src[i * src_w + j]; // Corrected indexing logic for transposition
        }
    }
}

fn main() {
    let a = [
        [0.0, 1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
        [1.0, 0.0, 0.0, 0.0, 42.0],
    ];
    let mut b = [[0.0; 3]; 5]; // Corrected matrix dimensions for b to 5x3

    // Flatten the nested arrays into a single vector
    let src: Vec<f64> = a.iter().flatten().cloned().collect();
    let mut dest: Vec<f64> = b.iter().flatten().cloned().collect();

    transpose(&mut dest, &src, 3, 5);

    let b: Vec<Vec<f64>> = dest.chunks(5).map(|chunk| chunk.to_vec()).collect(); // Corrected reshaping logic for 5x3 matrix

    for i in 0..5 { // Corrected outer loop to iterate over the width of the transposed matrix (5)
        for j in 0..3 { // Corrected inner loop to iterate over the height of the transposed matrix (3)
            print!("{}", b[i][j]);
            if j < 2 { // Adjusted condition for spacing
                print!(" ");
            } else {
                println!();
            }
        }
    }
}