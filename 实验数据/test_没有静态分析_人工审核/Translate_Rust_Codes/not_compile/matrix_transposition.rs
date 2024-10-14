fn transpose(dest: &mut [f64], src: &[f64], src_h: usize, src_w: usize) {
    let d = dest.chunks_mut(src_h);
    let s = src.chunks(src_w);
    for (i, row) in s.enumerate() {
        for (j, &value) in row.iter().enumerate() {
            // Modified: Directly use `d.enumerate()` on the `ChunksMut` iterator
            for (chunk_index, chunk) in d.enumerate() {
                if chunk_index == j {
                    chunk[i] = value;
                }
            }
        }
    }
}

fn main() {
    let a = [
        [0.0, 1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0, 9.0],
        [1.0, 0.0, 0.0, 0.0, 42.0],
    ];

    let src: Vec<f64> = a.iter().flatten().copied().collect();
    // Modified: Correctly initialize `dest` with the required size
    let mut dest: Vec<f64> = vec![0.0; src_h * src_w];

    transpose(&mut dest, &src, 3, 5);

    let b: Vec<[f64; 3]> = dest.chunks(3).map(|chunk| {
        let mut arr = [0.0; 3];
        arr.copy_from_slice(chunk);
        arr
    }).collect();

    for row in b.iter() {
        for (j, &value) in row.iter().enumerate() {
            print!("{}", value);
            if j < 2 {
                print!(" ");
            } else {
                println!();
            }
        }
    }
}